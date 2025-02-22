use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::{self, Sender};
use tracing::*;
use uuid::Uuid;

use crate::game::{start_game_world, GameInstanceConfig};
use rust_common_game::prelude::*;

const MIN_UDP_PORT: u16 = 34000;
const MAX_UDP_PORT: u16 = 35000;

#[derive(Debug)]
struct GameInstance {
    uuid: Uuid,
    port: u16,
    thread_join_handle: Option<JoinHandle<()>>,
    exit_channel_tx: Option<Sender<bool>>,
}

#[derive(Clone)]
struct AppStateDyn {
    pub instance_repo: Arc<dyn GameInstanceRepo>,
}

trait GameInstanceRepo: Send + Sync {
    fn get(&self, id: u16) -> Option<Arc<Mutex<GameInstance>>>;

    fn set(&self, user: GameInstance);
}

#[derive(Debug, Clone, Default)]
struct InMemoryGameInstanceRepo {
    map: Arc<Mutex<HashMap<u16, Arc<Mutex<GameInstance>>>>>,
}

impl GameInstanceRepo for InMemoryGameInstanceRepo {
    fn get(&self, id: u16) -> Option<Arc<Mutex<GameInstance>>> {
        self.map.lock().unwrap().get(&id).cloned()
    }

    fn set(&self, game_instance: GameInstance) {
        self.map
            .lock()
            .unwrap()
            .insert(game_instance.port, Arc::new(Mutex::new(game_instance)));
    }
}

async fn post_server_start(
    State(state): State<AppStateDyn>,
) -> (StatusCode, Json<HttpStartServerResponse>) {
    for port in MIN_UDP_PORT..=MAX_UDP_PORT {
        if state.instance_repo.get(port).is_some() {
            continue;
        }

        let (tx, rx) = oneshot::channel();

        let game_instance_config = GameInstanceConfig {
            port,
            exit_channel_rx: rx,
        };
        let thread_join_handle = thread::spawn(move || {
            start_game_world(game_instance_config);
        });

        let uuid = Uuid::new_v4();
        state.instance_repo.set(GameInstance {
            uuid,
            port,
            thread_join_handle: Some(thread_join_handle),
            exit_channel_tx: Some(tx),
        });

        let response = HttpStartServerResponse {
            instance_port: port,
            instance_uuid: uuid,
        };
        return (StatusCode::OK, Json(response));
    }

    warn!("[post_server_start] All UDP ports already in use");
    let response = HttpStartServerResponse {
        instance_port: 0,
        instance_uuid: Uuid::nil(),
    };
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

async fn post_server_stop(
    State(state): State<AppStateDyn>,
    Json(payload): Json<HttpStopServerInput>,
) -> (StatusCode, Json<HttpStopServerResponse>) {
    if let Some(game_instance) = state.instance_repo.get(payload.instance_port) {
        let mut lock = game_instance.lock().unwrap();
        if lock.uuid != payload.instance_uuid {
            warn!(
                "[post_server_stop] Invalid instance uuid: {}",
                payload.instance_uuid
            );
            let response = HttpStopServerResponse { succcess: true };
            return (StatusCode::BAD_REQUEST, Json(response));
        }

        if let Some(tx) = lock.exit_channel_tx.take() {
            if tx.send(true).is_ok() {
                lock.thread_join_handle.take().unwrap().join().unwrap();
                let response = HttpStopServerResponse { succcess: true };
                return (StatusCode::OK, Json(response));
            } else {
                error!("[post_server_stop] Failed to send stop message to instance");
            }
        }
    }

    warn!("[post_server_stop] Invalid instance port");
    let response = HttpStopServerResponse { succcess: false };
    (StatusCode::BAD_REQUEST, Json(response))
}

pub(crate) async fn start_http_api() {
    let app = Router::new()
        .route("/server/start", post(post_server_start))
        .route("/server/stop", post(post_server_stop))
        .with_state(AppStateDyn {
            instance_repo: Arc::new(InMemoryGameInstanceRepo::default()),
        });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    println!("Starting server");
    axum::serve(listener, app).await.unwrap();
}
