use aes_gcm_siv::{aead::Aead, Aes256GcmSiv, KeyInit, Nonce};
use crossbeam_channel::select;
use rust_common::{
    api_auth::AuthApi,
    api_lobby::LobbyApi,
    proto::{
        HttpGetGameServerInput, MsgUp, MsgUpHandshake, MsgUpType, MsgUpWrapper, UdpMsgDownType,
    },
};
use std::sync::{Arc, Mutex};

use crate::{global_state::GlobalState, udp::prelude::UdpState};

pub enum LobbyNodeEvent {
    ButtonLogoutPressed,
    ButtonCreateGamePressed,
    Closed,
}

pub enum LobbyStateEvent {
    IsLoadingChanged,
    LogoutSuccess,
    CreateWorldInstanceSuccess(String),
}

#[derive(Clone)]
pub struct LobbyState {
    pub is_loading: bool,
}

pub struct LobbyStateManager {
    global_state: GlobalState,
    udp_state: UdpState,
    state: Arc<Mutex<LobbyState>>,
    tx_state_events: crossbeam_channel::Sender<LobbyStateEvent>,
    http_client: reqwest::Client,
}
impl LobbyStateManager {
    pub fn new(
        global_state: GlobalState,
        udp_state: UdpState,
        state: Arc<Mutex<LobbyState>>,
        tx_state_events: crossbeam_channel::Sender<LobbyStateEvent>,
    ) -> Self {
        Self {
            global_state,
            state,
            udp_state,
            tx_state_events,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn start(&mut self, rx_node_events: crossbeam_channel::Receiver<LobbyNodeEvent>) {
        let rx_udp_receiver = self.udp_state.rx_udp_receiver.clone();
        'outer: loop {
            select! {
                recv(rx_node_events) -> node_event => match node_event {
                    Ok(LobbyNodeEvent::ButtonCreateGamePressed) => {
                        self.on_button_create_game_pressed().await;
                    }
                    Ok(LobbyNodeEvent::ButtonLogoutPressed) => {
                        self.on_button_logout_pressed().await;
                    }
                    Ok(LobbyNodeEvent::Closed) => {
                        break 'outer;
                    }
                    Err(_) => println!("LobbyStateManager rx_node_events err")
                },
                recv(rx_udp_receiver) -> udp_msg_wrapper => match udp_msg_wrapper {
                    Ok(udp_msg_wrapper) => {
                        for udp_msg_down in udp_msg_wrapper.messages {
                            match UdpMsgDownType::try_from(udp_msg_down.r#type) {
                                Ok(UdpMsgDownType::UserConnectSuccess) => {
                                    self.on_game_server_connect_success().await;
                                }
                                Ok(UdpMsgDownType::UserConnectFailed) => {
                                    println!(
                                        "UserConnectFailed {}",
                                        udp_msg_down.user_connect_failed.unwrap().error_message
                                    );
                                }
                                Ok(UdpMsgDownType::UserCreateWordlInstanceSuccess) => {
                                   let payload = udp_msg_down.user_create_world_instance_success.unwrap();
                                   self.tx_state_events.send(LobbyStateEvent::CreateWorldInstanceSuccess(payload.id)).unwrap();
                                }
                                _ => {}
                            }
                        }
                    },
                    Err(_) => println!("LobbyStateManager rx_udp_receiver err"),
                },
            };
        }
    }

    async fn on_button_create_game_pressed(&mut self) {
        if !self.set_is_loading_or_cancel() {
            return;
        }

        let Some(user) = self.global_state.get_user() else {
            self.tx_state_events
                .send(LobbyStateEvent::LogoutSuccess)
                .unwrap();
            return;
        };

        // let is_connected_to_game_server = self.global_state.get_is_connected_to_game_server();

        // if is_connected_to_game_server {
        //     self.create_world_instance();
        //     return;
        // }

        let game_server = match LobbyApi::get_game_server(
            &self.http_client,
            user.auth_token,
            HttpGetGameServerInput {},
        )
        .await
        {
            Ok(game_server) => game_server,
            Err(err) => {
                self.state.lock().unwrap().is_loading = false;
                self.tx_state_events
                    .send(LobbyStateEvent::IsLoadingChanged)
                    .unwrap();
                println!("{}", err.message);
                return;
            }
        };

        println!("{:#?}", game_server);

        self.udp_state
            .start_client_async(game_server.udp_port as u16)
            .await;

        let vec_u8_aes_key = hex::decode(game_server.aes_key).unwrap();
        let cipher = Aes256GcmSiv::new_from_slice(&vec_u8_aes_key[..]).unwrap();
        let game_server_aes_nonce = game_server.aes_nonce.to_string();
        let game_server_aes_nonce = game_server_aes_nonce.as_bytes();
        let nonce = Nonce::from_slice(game_server_aes_nonce);

        let signed_message =
            match cipher.encrypt(nonce, user.game_server_handshake_challenge.as_bytes()) {
                Ok(signed_message) => signed_message,
                Err(err) => {
                    println!("Failed to encrypt : {}", err);
                    return;
                }
            };

        self.udp_state.send_udp_handshake(MsgUpHandshake {
            user_uuid: user.uuid,
            signed_message: hex::encode(signed_message),
        })
    }

    async fn on_game_server_connect_success(&mut self) {
        self.global_state
            .set_is_connected_to_game_server(true)
            .await;

        self.create_world_instance();

        self.state.lock().unwrap().is_loading = false;
        self.tx_state_events
            .send(LobbyStateEvent::IsLoadingChanged)
            .unwrap();
    }

    async fn on_button_logout_pressed(&mut self) {
        if !self.set_is_loading_or_cancel() {
            return;
        }

        if self.global_state.get_is_connected_to_game_server() {
            self.udp_state.send_udp(MsgUpWrapper {
                messages: vec![MsgUp {
                    r#type: MsgUpType::UserDisconnect.into(),
                    ..Default::default()
                }],
            });
            self.global_state
                .set_is_connected_to_game_server(false)
                .await;
        }

        let Some(user) = self.global_state.get_user() else {
            self.tx_state_events
                .send(LobbyStateEvent::LogoutSuccess)
                .unwrap();
            return;
        };

        let _ = AuthApi::logout(&self.http_client, user.auth_token).await;

        self.global_state.set_user(None).await;

        self.tx_state_events
            .send(LobbyStateEvent::LogoutSuccess)
            .unwrap();
    }

    fn set_is_loading_or_cancel(&mut self) -> bool {
        let mut state_lock = self.state.lock().unwrap();
        if state_lock.is_loading {
            return false;
        }
        state_lock.is_loading = true;
        self.tx_state_events
            .send(LobbyStateEvent::IsLoadingChanged)
            .unwrap();
        true
    }

    fn create_world_instance(&mut self) {
        self.udp_state.send_udp(MsgUpWrapper {
            messages: vec![MsgUp {
                r#type: MsgUpType::UserCreateWorldInstance.into(),
                ..Default::default()
            }],
        });
    }
}
