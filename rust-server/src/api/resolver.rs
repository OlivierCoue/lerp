use rust_common::proto::{udp_down::*, udp_up::*};

use self::service_area::ApiServiceArea;
use self::service_user::ApiServiceUser;

use super::*;

pub struct ApiResolver {}
impl ApiResolver {
    pub async fn handle_msg_up_wrapper(
        app: App,
        udp_peer_id: u16,
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
                        opt_udp_messages_down =
                            ApiServiceUser::connect(app.clone(), udp_peer_id, username).await;
                    }
                }
                _ => {
                    let mut opt_user = None;
                    {
                        let users_state_lock: std::sync::MutexGuard<'_, UsersState> =
                            app.get_users_state_lock();
                        if let Some(user_id) =
                            users_state_lock.udp_peer_id_user_uuid_map.get(&udp_peer_id)
                        {
                            if let Some(user) = users_state_lock.user_uuid_user_map.get(user_id) {
                                opt_user = Some(*user);
                            }
                        }
                    }

                    if let Some(user) = opt_user {
                        opt_udp_messages_down = Self::handle_authenticated_msg(
                            app.clone(),
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
                app.tx_udp_sender
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
        app: App,
        udp_msg_up: &MsgUp,
        _type: MsgUpType,
        user: &User,
    ) -> Option<Vec<UdpMsgDown>> {
        match _type {
            MsgUpType::USER_DISCONNECT => ApiServiceUser::disconnect(app, user).await,
            MsgUpType::USER_CREATE_WORLD_INSTANCE => ApiServiceArea::create(app, user).await,
            MsgUpType::USER_JOIN_WOLD_INSTANCE => {
                if let Some(payload) = udp_msg_up.user_join_world_instance.clone().into_option() {
                    ApiServiceArea::join(app, user, payload.id).await
                } else {
                    None
                }
            }
            MsgUpType::USER_LEAVE_WORLD_INSTANCE => ApiServiceArea::leave(app, user),
            _ => {
                ApiServiceArea::forward_msg(app, user, udp_msg_up);
                None
            }
        }
    }
}
