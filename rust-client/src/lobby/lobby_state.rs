use rust_common::api_auth::AuthApi;
use std::sync::{mpsc, Arc, Mutex};

use crate::global_state::GlobalState;

pub enum LobbyNodeEvent {
    LogoutButtonPressed,
    Closed,
}

pub enum LobbyStateEvent {
    IsLoadingChanged,
    LogoutSuccess,
}

#[derive(Clone)]
pub struct LobbyState {
    pub is_loading: bool,
}

pub struct LobbyStateManager {
    global_state: GlobalState,
    state: Arc<Mutex<LobbyState>>,
    tx_state_events: mpsc::Sender<LobbyStateEvent>,
    http_client: reqwest::Client,
}
impl LobbyStateManager {
    pub fn new(
        global_state: GlobalState,
        state: Arc<Mutex<LobbyState>>,
        tx_state_events: mpsc::Sender<LobbyStateEvent>,
    ) -> Self {
        Self {
            global_state,
            state,
            tx_state_events,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn start(&mut self, rx_node_events: mpsc::Receiver<LobbyNodeEvent>) {
        'outer: for node_event in &rx_node_events {
            match node_event {
                LobbyNodeEvent::LogoutButtonPressed => {
                    self.on_logout_button_pressed().await;
                }
                LobbyNodeEvent::Closed => {
                    break 'outer;
                }
            }
        }
    }

    async fn on_logout_button_pressed(&mut self) {
        let Some(user) = self.global_state.get_user() else {
            self.tx_state_events
                .send(LobbyStateEvent::LogoutSuccess)
                .unwrap();
            return;
        };

        {
            let mut state_lock = self.state.lock().unwrap();
            if state_lock.is_loading {
                return;
            }
            state_lock.is_loading = true;
            self.tx_state_events
                .send(LobbyStateEvent::IsLoadingChanged)
                .unwrap()
        }

        let _ = AuthApi::logout(&self.http_client, user.auth_token).await;

        self.global_state.set_user(None).await;

        self.tx_state_events
            .send(LobbyStateEvent::LogoutSuccess)
            .unwrap();
    }
}
