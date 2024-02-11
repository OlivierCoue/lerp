use rust_common::proto::*;

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
            match MsgUpType::try_from(udp_msg_up.r#type) {
                Ok(MsgUpType::UserConnect) => {
                    if !udp_msg_up.user_connect_username.is_empty() {
                        opt_udp_messages_down = ApiServiceUser::connect(
                            app.clone(),
                            udp_peer_id,
                            udp_msg_up.user_connect_username,
                        )
                        .await;
                    }
                }
                Ok(udp_msg_up_type) => {
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
                Err(_) => {
                    println!("handle_msg_up_wrapper: invalid enum value");
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
            MsgUpType::UserDisconnect => ApiServiceUser::disconnect(app, user).await,
            MsgUpType::UserCreateWorldInstance => ApiServiceArea::create(app, user).await,
            MsgUpType::UserJoinWoldInstance => {
                if let Some(payload) = &udp_msg_up.user_join_world_instance {
                    ApiServiceArea::join(app, user, payload.id.clone()).await
                } else {
                    None
                }
            }
            MsgUpType::UserLeaveWorldInstance => ApiServiceArea::leave(app, user).await,
            _ => {
                ApiServiceArea::forward_msg(app, user, udp_msg_up);
                None
            }
        }
    }

    pub async fn handle_outbound_area_message(
        app: App,
        outbound_area_message: OutboundAreaMessage,
    ) {
        match outbound_area_message {
            OutboundAreaMessage::AreaClosing(payload) => {
                ApiServiceArea::close(app, payload.area_uuid).await;
            }
        }
    }
}
