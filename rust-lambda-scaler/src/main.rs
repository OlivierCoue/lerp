use std::time::Duration;

use aes_gcm_siv::{aead::OsRng, Aes256GcmSiv, KeyInit};
use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use k8s_openapi::api::core::v1::{ConfigMap, Pod, Service};
use kube::{
    api::{DeleteParams, ListParams, Patch, PatchParams, PostParams},
    Api, Client, ResourceExt,
};
use lambda_runtime::{
    run, service_fn,
    tracing::{self},
    Error, LambdaEvent,
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Empty {}

struct GameServer {
    uuid: Uuid,
    udp_port: i32,
}

pub const ENV_POSTGRES_DATABASE_URL: &str = "DATABASE_URL";

const K8S_NS_LERP: &str = "lerp";
const K8S_NS_INGRESS_NGINX: &str = "ingress-nginx";
const K8S_APP_NAME_GAME_SERVER: &str = "game-server";
const K8S_APPLY_MANAGER_NAME: &str = "server-scaler";

async fn function_handler(_: LambdaEvent<EventBridgeEvent<Empty>>) -> Result<(), Error> {
    // Local = 1h | dev = 50sec
    // This is because localy we just start it once and it runs "forever", while on AWS, this lambda is call every 1 minute by EventBridge
    let lambda_duration_sec = match env!("TARGET_ENV") {
        "local" => 3600,
        "dev" => 50,
        _ => panic!("Invalid TARGET_ENV value"),
    };

    let pg_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(
            std::env::var(ENV_POSTGRES_DATABASE_URL)
                .unwrap_or_else(|_| panic!("env var {ENV_POSTGRES_DATABASE_URL} is not set"))
                .as_str(),
        )
        .await
        .unwrap();

    let mut time_passed = 0;
    let rate_sec = 5;

    let client = Client::try_default().await.unwrap();

    let pods_lerp: Api<Pod> = Api::namespaced(client.clone(), K8S_NS_LERP);
    let services_lerp: Api<Service> = Api::namespaced(client.clone(), K8S_NS_LERP);
    let config_map_ingress_nginx: Api<ConfigMap> =
        Api::namespaced(client.clone(), K8S_NS_INGRESS_NGINX);

    loop {
        // Run every <rate_sec> seconds
        sleep(Duration::from_secs(rate_sec)).await;
        time_passed += time_passed;
        if time_passed >= lambda_duration_sec {
            break;
        }

        // Ensure sync with k8s and pg

        delete_game_servers_in_k8s_but_not_in_pg(&pg_pool, &pods_lerp, &services_lerp).await?;
        delete_game_servers_in_pg_but_not_in_k8s(&pg_pool, &pods_lerp, &services_lerp).await?;

        // Create new game server if needed

        let desired_game_server_count = 1;
        let game_servers = get_game_server_pg_entries(&pg_pool).await?;
        let game_server_to_start_count = desired_game_server_count - game_servers.len();

        if game_server_to_start_count > 0 {
            let udp_ports = get_available_udp_ports(game_server_to_start_count, &game_servers);
            for udp_port in udp_ports {
                create_game_server(udp_port, &pg_pool, &pods_lerp, &services_lerp).await;
            }
        }

        // Update ingress-nginx-udp config map with the currently running server game services port mapping
        // Ingress Nginx is already doing the diff when the config map change, so we can safely update the config map pretty often

        update_ingress_nginx_udp_config_map(&services_lerp, &config_map_ingress_nginx).await?;
    }

    Ok(())
}

async fn delete_game_servers_in_k8s_but_not_in_pg(
    pg_pool: &Pool<Postgres>,
    pods_lerp: &Api<Pod>,
    services_lerp: &Api<Service>,
) -> Result<(), Error> {
    let game_servers = get_game_server_pg_entries(pg_pool).await?;
    let game_servers_uuids = Vec::from_iter(game_servers.iter().map(|gs| gs.uuid.to_string()));

    for p in get_game_server_k8s_pods(pods_lerp).await? {
        let game_server_uuid = p.labels().get("game-server-uuid").unwrap().to_owned();
        if !game_servers_uuids.contains(&game_server_uuid) {
            if let Err(err) = pods_lerp
                .delete(&p.name_any(), &DeleteParams::default())
                .await
            {
                println!(
                    "[delete_game_servers_in_k8s_but_not_in_pg] Failed to delete pod: {}",
                    err
                );
            }
        }
    }

    for s in get_game_server_k8s_services(services_lerp).await? {
        let game_server_uuid = s.labels().get("game-server-uuid").unwrap().to_owned();
        if !game_servers_uuids.contains(&game_server_uuid) {
            if let Err(err) = services_lerp
                .delete(&s.name_any(), &DeleteParams::default())
                .await
            {
                println!(
                    "[delete_game_servers_in_k8s_but_not_in_pg] Failed to delete service: {}",
                    err
                );
            }
        }
    }

    Ok(())
}

async fn delete_game_servers_in_pg_but_not_in_k8s(
    pg_pool: &Pool<Postgres>,
    pods_lerp: &Api<Pod>,
    services_lerp: &Api<Service>,
) -> Result<(), Error> {
    let game_servers_uuids_from_pods = Vec::from_iter(
        get_game_server_k8s_pods(pods_lerp)
            .await?
            .iter()
            .map(|p| p.labels().get("game-server-uuid").unwrap().to_owned()),
    );

    let game_servers_uuids_from_services = Vec::from_iter(
        get_game_server_k8s_services(services_lerp)
            .await?
            .iter()
            .map(|s| s.labels().get("game-server-uuid").unwrap().to_owned()),
    );

    for game_server in get_game_server_pg_entries(pg_pool).await? {
        if !game_servers_uuids_from_pods.contains(&game_server.uuid.to_string())
            || !game_servers_uuids_from_services.contains(&game_server.uuid.to_string())
        {
            sqlx::query!("DELETE FROM game_servers WHERE uuid = $1", game_server.uuid)
                .execute(pg_pool)
                .await?;
        }
    }

    Ok(())
}

async fn create_game_server(
    udp_port: i32,
    pg_pool: &Pool<Postgres>,
    pods_lerp: &Api<Pod>,
    services_lerp: &Api<Service>,
) {
    let database_url =
        String::from("postgresql://admin:password@host.docker.internal:5432/lerp-local");

    let game_server_uuid = Uuid::new_v4();
    let game_server_service_name = format!("s-rust-server-game-{}", game_server_uuid);
    let game_server_pod_name = format!("p-rust-server-game-{}", game_server_uuid);

    let game_server_aes_key = hex::encode(Aes256GcmSiv::generate_key(&mut OsRng));
    let game_server_aes_nonce = "aaaaaaaaaaaa".to_string();

    sqlx::query!(
        "INSERT INTO game_servers (uuid, udp_port, aes_key, aes_nonce) VALUES ($1, $2, $3, $4);",
        game_server_uuid,
        udp_port,
        game_server_aes_key,
        game_server_aes_nonce,
    )
    .execute(pg_pool)
    .await
    .unwrap();

    let game_server_service: Service = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Service",
        "metadata": {
            "namespace": "lerp",
            "name": game_server_service_name,
            "labels": {
                "app": K8S_APP_NAME_GAME_SERVER,
                "game-server-uuid": game_server_uuid.to_string()
            }
        },
        "spec": {
            "selector": {
                "game-server-uuid": game_server_uuid.to_string()
            },
            "type": "ClusterIP",
            "ports": [{
                "protocol": "UDP",
                "port": udp_port,
                "targetPort": udp_port,
                "name": "udp"
            }],
        }
    }))
    .unwrap();

    let pp = PostParams::default();
    match services_lerp.create(&pp, &game_server_service).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(game_server_service.name_any(), name);
            println!("Service created {}", name);
        }
        Err(e) => println!("{}", e),
    }

    let game_server_pod: Pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "namespace": "lerp",
            "name": game_server_pod_name,
            "labels": {
                "app": K8S_APP_NAME_GAME_SERVER,
                "game-server-uuid": game_server_uuid.to_string()
            }
        },
        "spec": {
            "containers": [{
                "name": "rust-server-game",
                "image": "rust-server-game:1",
                "ports": [{
                    "containerPort": udp_port,
                    "protocol": "UDP",
                    "name": "udp"
                }],
                "env": [
                    { "name": "DATABASE_URL", "value": database_url },
                    { "name": "UDP_PORT", "value": udp_port.to_string() },
                    { "name": "AES_KEY", "value": game_server_aes_key },
                    { "name": "AES_NONCE", "value": game_server_aes_nonce },
                    { "name": "GAME_SERVER_UUID", "value": game_server_uuid.to_string() }
                ]
            }],
        }
    }))
    .unwrap();

    let pp = PostParams::default();
    match pods_lerp.create(&pp, &game_server_pod).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(game_server_pod.name_any(), name);
            println!("Pods created {}", name);
        }
        Err(e) => println!("{}", e),
    }
}

async fn update_ingress_nginx_udp_config_map(
    services_lerp: &Api<Service>,
    config_map_ingress_nginx: &Api<ConfigMap>,
) -> Result<(), Error> {
    // Example of config map data:
    // data:
    //   "34254": lerp/s-rust-server-game-1:34254
    let mut ingress_nginx_udp_data = serde_json::Map::new();
    for s in get_game_server_k8s_services(services_lerp).await? {
        let s_name = s.name_any();
        let s_port = s.spec.unwrap().ports.unwrap().first().unwrap().port;

        ingress_nginx_udp_data.insert(
            s_port.to_string(),
            serde_json::Value::String(format!("{}/{}:{}", K8S_NS_LERP, s_name, s_port)),
        );
    }

    let patch = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "data": serde_json::Value::Object(ingress_nginx_udp_data)
    });
    // Because the ingress-nginx-udp config map is created by helm, we must force the aplly to bypass conflicts
    let params = PatchParams::apply(K8S_APPLY_MANAGER_NAME).force();
    let patch = Patch::Apply(&patch);

    match config_map_ingress_nginx
        .patch("ingress-nginx-udp", &params, &patch)
        .await
    {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    Ok(())
}

async fn get_game_server_pg_entries(pg_pool: &Pool<Postgres>) -> Result<Vec<GameServer>, Error> {
    Ok(sqlx::query_as!(
        GameServer,
        "SELECT uuid, udp_port FROM game_servers ORDER BY udp_port ASC;"
    )
    .fetch_all(pg_pool)
    .await?)
}

async fn get_game_server_k8s_pods(pods_lerp: &Api<Pod>) -> Result<Vec<Pod>, Error> {
    let pods_lp: ListParams =
        ListParams::default().labels(&("app=".to_owned() + K8S_APP_NAME_GAME_SERVER));
    let mut game_server_pods = Vec::new();

    for p in pods_lerp.list(&pods_lp).await? {
        game_server_pods.push(p);
    }

    Ok(game_server_pods)
}

async fn get_game_server_k8s_services(services_lerp: &Api<Service>) -> Result<Vec<Service>, Error> {
    let services_lp: ListParams =
        ListParams::default().labels(&("app=".to_owned() + K8S_APP_NAME_GAME_SERVER));

    let mut game_server_services = Vec::new();

    for s in services_lerp.list(&services_lp).await? {
        game_server_services.push(s);
    }

    Ok(game_server_services)
}

fn get_available_udp_ports(count: usize, running_game_servers: &[GameServer]) -> Vec<i32> {
    let base = 34254;

    let mut ports = Vec::new();
    let ports_in_use = Vec::from_iter(running_game_servers.iter().map(|g| g.udp_port));

    for i in 0..1000 {
        let canditate_port = base + i;
        if !ports_in_use.contains(&canditate_port) {
            ports.push(canditate_port);
            if ports.len() == count {
                break;
            }
        }
    }

    ports
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
