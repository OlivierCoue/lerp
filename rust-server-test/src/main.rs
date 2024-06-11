use env::init_env;
use k8s_openapi::api::core::v1::{Pod, Service};
use kube::{api::PostParams, Client};
use kube::{Api, ResourceExt};
use rust_common::{
    api_auth::AuthApi,
    proto::{HttpLoginInput, HttpRegisterInput},
};
use serde_json::json;
use uuid::Uuid;

mod env;

#[tokio::main]
async fn main() -> Result<(), String> {
    init_env();

    let client = Client::try_default().await.unwrap();
    let pods: Api<Pod> = Api::default_namespaced(client.clone());
    let services: Api<Service> = Api::default_namespaced(client);

    let s: Service = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Service",
        "metadata": {
            "namespace": "lerp",
            "name": "s-rust-server-game"
        },
        "spec": {
            "selector": {
                "game-server-name": "1"
            },
            "type": "ClusterIP",
            "ports": [{
              "protocol": "UDP",
              "port": 34254,
              "targetPort": 34254,
              "name": "udp"
            }],
        }
    }))
    .unwrap();

    let pp = PostParams::default();
    match services.create(&pp, &s).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(s.name_any(), name);
            println!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => println!("{}", e),                           // any other case is probably bad
    }

    let p: Pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
          "namespace": "lerp",
          "name": "p-rust-server-game",
          "labels": { 
            "app": "game-server",
            "game-server-name": "1"
          }
        },
        "spec": {
            "containers": [{
              "name": "rust-server-game",
              "image": "rust-server-game:1",
              "ports": [{
                "containerPort": 34254,
                "protocol": "UDP",
                "name": "udp"
              }],
              "env": [
                { "name": "DATABASE_URL", "value": "postgresql://admin:password@192.168.1.27:5432/lerp-local" },
                { "name": "UDP_PORT", "value": "34254" },
               ]
            }],
        }
    }))
    .unwrap();

    let pp = PostParams::default();
    match pods.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            println!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => println!("{}", e),                           // any other case is probably bad
    }

    println!("[Test] Starting");

    let client = reqwest::Client::new();

    let username = Uuid::new_v4().to_string();

    // Register
    let input = HttpRegisterInput {
        username: username.clone(),
        password: "test".into(),
    };

    match AuthApi::register(&client, input).await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err.message);
            panic!("[Auth][register] failed")
        }
    };

    // Login
    let input = HttpLoginInput {
        username: username.clone(),
        password: "test".into(),
    };

    let res = match AuthApi::login(&client, input).await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err.message);
            panic!("[Auth][login] failed")
        }
    };

    let auth_token = res.auth_token;

    // User get current
    let user = AuthApi::user_get_current(&client, auth_token.clone())
        .await
        .unwrap();

    assert_eq!(user.username, username);

    // Logout
    AuthApi::logout(&client, auth_token.clone()).await.unwrap();

    // User get current (expected to fail)
    if AuthApi::user_get_current(&client, auth_token.clone())
        .await
        .is_ok()
    {
        return Err("Expected user_get_current to fail after logout".to_string());
    }

    println!("[Test] Passed");

    Ok(())
}
