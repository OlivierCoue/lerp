use rust_common::{
    api_auth::AuthApi,
    proto::{HttpLoginInput, HttpRegisterInput},
};
use std::sync::{Arc, Mutex};

use crate::global_state::{GlobalState, StateUser};

pub enum AuthNodeEvent {
    LoginButtonPressed(String),
    RegisterButtonPressed(String),
    Closed,
}

pub enum AuthStateEvent {
    IsLoadingChanged,
    LoginErrorChanged,
    LoginSuccess,
    RegisterSuccess,
    RegisterErrorChanged,
}

#[derive(Clone)]
pub struct AuthState {
    pub is_loading: bool,
    pub login_error: Option<String>,
    pub register_error: Option<String>,
}

pub struct AuthStateManager {
    global_state: GlobalState,
    state: Arc<Mutex<AuthState>>,
    tx_state_events: crossbeam_channel::Sender<AuthStateEvent>,
    http_client: reqwest::Client,
}
impl AuthStateManager {
    pub fn new(
        global_state: GlobalState,
        state: Arc<Mutex<AuthState>>,
        tx_state_events: crossbeam_channel::Sender<AuthStateEvent>,
    ) -> Self {
        Self {
            global_state,
            state,
            tx_state_events,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn start(&mut self, rx_node_events: crossbeam_channel::Receiver<AuthNodeEvent>) {
        'outer: for node_event in &rx_node_events {
            match node_event {
                AuthNodeEvent::LoginButtonPressed(username) => {
                    self.on_button_login_pressed(username).await;
                }
                AuthNodeEvent::RegisterButtonPressed(username) => {
                    self.on_button_register_pressed(username).await;
                }
                AuthNodeEvent::Closed => {
                    break 'outer;
                }
            }
        }
    }

    async fn on_button_login_pressed(&mut self, username: String) {
        {
            let mut state_lock = self.state.lock().unwrap();
            if state_lock.is_loading {
                return;
            }
            state_lock.is_loading = true;
            self.tx_state_events
                .send(AuthStateEvent::IsLoadingChanged)
                .unwrap()
        }

        let input = HttpLoginInput {
            username,
            password: "abc".into(), // TODO
        };

        let login_response = match AuthApi::login(&self.http_client, input).await {
            Ok(response) => response,
            Err(err) => {
                let mut state_lock = self.state.lock().unwrap();
                state_lock.login_error = Some(err.message);
                state_lock.is_loading = false;
                self.tx_state_events
                    .send(AuthStateEvent::LoginErrorChanged)
                    .unwrap();
                self.tx_state_events
                    .send(AuthStateEvent::IsLoadingChanged)
                    .unwrap();
                return;
            }
        };

        match AuthApi::user_get_current(&self.http_client, login_response.auth_token.clone()).await
        {
            Ok(response) => response,
            Err(err) => {
                let mut state_lock = self.state.lock().unwrap();
                state_lock.login_error = Some(err.message);
                state_lock.is_loading = false;
                self.tx_state_events
                    .send(AuthStateEvent::LoginErrorChanged)
                    .unwrap();
                self.tx_state_events
                    .send(AuthStateEvent::IsLoadingChanged)
                    .unwrap();
                return;
            }
        };

        self.global_state
            .set_user(Some(StateUser {
                uuid: login_response.uuid,
                username: login_response.username,
                auth_token: login_response.auth_token,
                game_server_aes_key: login_response.game_server_aes_key,
                game_server_aes_nonce: login_response.game_server_aes_nonce,
                game_server_handshake_challenge: login_response.game_server_handshake_challenge,
            }))
            .await;

        println!(
            "Login success for user: {:#?}",
            self.global_state.get_user()
        );

        self.tx_state_events
            .send(AuthStateEvent::LoginSuccess)
            .unwrap();
    }

    async fn on_button_register_pressed(&mut self, username: String) {
        {
            let mut state_lock = self.state.lock().unwrap();
            if state_lock.is_loading {
                return;
            }
            state_lock.is_loading = true;
            self.tx_state_events
                .send(AuthStateEvent::IsLoadingChanged)
                .unwrap()
        }

        let input = HttpRegisterInput {
            username,
            password: "abc".into(), // TODO
        };

        match AuthApi::register(&self.http_client, input).await {
            Ok(_) => {
                let mut state_lock = self.state.lock().unwrap();
                state_lock.register_error = None;
                self.tx_state_events
                    .send(AuthStateEvent::RegisterErrorChanged)
                    .unwrap();
                self.tx_state_events
                    .send(AuthStateEvent::RegisterSuccess)
                    .unwrap();
            }
            Err(err) => {
                let mut state_lock = self.state.lock().unwrap();
                state_lock.register_error = Some(err.message);
                self.tx_state_events
                    .send(AuthStateEvent::RegisterErrorChanged)
                    .unwrap();
            }
        };

        let mut state_lock = self.state.lock().unwrap();
        state_lock.is_loading = false;
        self.tx_state_events
            .send(AuthStateEvent::IsLoadingChanged)
            .unwrap();
    }
}
