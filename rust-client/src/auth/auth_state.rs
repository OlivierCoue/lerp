use prost::Message;
use rust_common::proto::{HttpError, HttpLoginInput};
use std::sync::{mpsc, Arc, Mutex};

pub enum AuthNodeEvent {
    ConnectButtonPressed(String),
    Closed,
}

pub enum AuthStateEvent {
    IsLoadingChanged,
    ConnectErrorChanged,
    ConnectSuccess,
}

#[derive(Clone)]
pub struct AuthState {
    pub is_loading: bool,
    pub connect_error: Option<String>,
}

const SERVER_AUTH_URL: &str = "http://127.0.0.1:3000/lambda-url/rust-server-auth";

pub struct AuthStateManager {
    state: Arc<Mutex<AuthState>>,
    tx_state_events: mpsc::Sender<AuthStateEvent>,
    http_client: reqwest::Client,
}
impl AuthStateManager {
    pub fn new(
        state: Arc<Mutex<AuthState>>,
        tx_state_events: mpsc::Sender<AuthStateEvent>,
    ) -> Self {
        Self {
            state,
            tx_state_events,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn start(&mut self, rx_node_events: mpsc::Receiver<AuthNodeEvent>) {
        'outer: for node_event in &rx_node_events {
            match node_event {
                AuthNodeEvent::ConnectButtonPressed(username) => {
                    self.on_connect_button_pressed(username).await;
                }
                AuthNodeEvent::Closed => {
                    break 'outer;
                }
            }
        }
    }

    async fn on_connect_button_pressed(&mut self, username: String) {
        {
            let mut state_lock = self.state.lock().unwrap();
            // Stop if an action is already ongoing
            if state_lock.is_loading {
                return;
            }
            state_lock.is_loading = true;
            self.tx_state_events
                .send(AuthStateEvent::IsLoadingChanged)
                .unwrap()
        }

        let body = HttpLoginInput {
            username,
            password: "abc".into(), // TODO
        };
        let mut buf = Vec::with_capacity(body.encoded_len());
        body.encode(&mut buf).unwrap();

        let resp = self
            .http_client
            .post(SERVER_AUTH_URL.to_owned() + "/login")
            .body(buf)
            .send()
            .await
            .unwrap();

        if resp.status().is_success() {
            self.tx_state_events
                .send(AuthStateEvent::ConnectSuccess)
                .unwrap();
        } else {
            let error = HttpError::decode(resp.bytes().await.unwrap()).unwrap();

            let mut state_lock = self.state.lock().unwrap();
            state_lock.connect_error = Some(error.message);
            state_lock.is_loading = false;
            self.tx_state_events
                .send(AuthStateEvent::ConnectErrorChanged)
                .unwrap();
            self.tx_state_events
                .send(AuthStateEvent::IsLoadingChanged)
                .unwrap();
        }
    }
}
