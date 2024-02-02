use rust_common::proto::{udp_down::*, udp_up::*};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use self::service_area::ApiServiceArea;
use self::service_user::ApiServiceUser;

use super::*;

pub struct ApiResolver {}
impl ApiResolver {
    pub async fn handle_msg_up_wrapper(
        mongo_client: mongodb::Client,
        tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
        udp_peer_id: u16,
        connections_state: Arc<Mutex<ConnectionsState>>,
        udp_msg_up_wrapper: MsgUpWrapper,
    ) {
        let mut opt_udp_messages_down = None;

        for udp_msg_up in udp_msg_up_wrapper.messages {
            let udp_msg_up_type = match udp_msg_up._type.enum_value() {
                Ok(udp_msg_up_type) => udp_msg_up_type,
                Err(err) => {
                    println!("[ApiService][handle_msg_up_wrapper] Received invalid _type in udp_msg_up, error: {}", err);
                    continue;
                }
            };

            match udp_msg_up_type {
                MsgUpType::USER_CONNECT => {
                    if let Some(username) = udp_msg_up.user_connect_username {
                        opt_udp_messages_down = ApiServiceUser::connect(
                            mongo_client.clone(),
                            udp_peer_id,
                            connections_state.clone(),
                            username,
                        )
                        .await;
                    }
                }
                _ => {
                    let mut opt_user = None;
                    {
                        let connections_state_lock: std::sync::MutexGuard<'_, ConnectionsState> =
                            connections_state.lock().unwrap();
                        if let Some(user_id) = connections_state_lock
                            .udp_peer_id_user_id_map
                            .get(&udp_peer_id)
                        {
                            if let Some(user) = connections_state_lock.user_id_user_map.get(user_id)
                            {
                                opt_user = Some(*user);
                            }
                        }
                    }

                    if let Some(user) = opt_user {
                        opt_udp_messages_down = Self::handle_authenticated_msg(
                            mongo_client.clone(),
                            udp_peer_id,
                            connections_state.clone(),
                            tx_udp_sender.clone(),
                            &udp_msg_up,
                            udp_msg_up_type,
                            &user,
                        )
                        .await;
                    };
                }
            };
        }

        if let Some(udp_messages_down) = opt_udp_messages_down {
            if !udp_messages_down.is_empty() {
                tx_udp_sender
                    .send((
                        udp_peer_id,
                        UdpMsgDownWrapper {
                            messages: udp_messages_down,
                            ..Default::default()
                        },
                    ))
                    .await
                    .unwrap();
            }
        }
    }

    async fn handle_authenticated_msg(
        mongo_client: mongodb::Client,
        udp_peer_id: u16,
        connections_state: Arc<Mutex<ConnectionsState>>,
        tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
        udp_msg_up: &MsgUp,
        _type: MsgUpType,
        user: &User,
    ) -> Option<Vec<UdpMsgDown>> {
        match _type {
            MsgUpType::USER_DISCONNECT => {
                ApiServiceUser::disconnect(
                    mongo_client.clone(),
                    connections_state.clone(),
                    udp_peer_id,
                )
                .await
            }
            MsgUpType::USER_CREATE_WORLD_INSTANCE => {
                ApiServiceArea::create(
                    mongo_client.clone(),
                    connections_state.clone(),
                    tx_udp_sender.clone(),
                    user,
                )
                .await
            }
            MsgUpType::USER_JOIN_WOLD_INSTANCE => {
                if let Some(payload) = udp_msg_up.user_join_world_instance.clone().into_option() {
                    ApiServiceArea::join(
                        mongo_client.clone(),
                        connections_state.clone(),
                        user,
                        payload.id,
                    )
                    .await
                } else {
                    None
                }
            }
            _ => {
                ApiServiceArea::forward_msg(user, connections_state.clone(), udp_msg_up);
                None
            }
        }
    }
}
