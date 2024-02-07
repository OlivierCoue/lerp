use rust_common::proto::udp_down::*;
use uuid::Uuid;

use self::service_area::ApiServiceArea;

use super::*;

pub struct ApiServiceUser {}
impl ApiServiceUser {
    pub async fn connect(app: App, udp_peer_id: u16, username: String) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        // If a user is already localy register with the incoming udp_peer_id send an error
        if app
            .get_users_state_lock()
            .udp_peer_id_user_uuid_map
            .get(&udp_peer_id)
            .is_some()
        {
            udp_messages.push(UdpMsgDown {
                _type: UdpMsgDownType::USER_CONNECT_FAILED.into(),
                user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                    error_message: "A user is already connected on this client.".into(),
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            });

            return Some(udp_messages);
        };

        struct PgResult {
            uuid: Uuid,
        }

        let user = sqlx::query_as!(
            PgResult,
            r#"SELECT uuid FROM users WHERE username = $1"#,
            username.clone()
        )
        .fetch_optional(&app.pg_pool)
        .await;

        let user = match user {
            Ok(opt_user) => opt_user,
            Err(err) => {
                println!(
                    "[ApiServiceUser][connect] Failed to query user, error: {}",
                    err
                );
                udp_messages.push(UdpMsgDown {
                    _type: UdpMsgDownType::USER_CONNECT_FAILED.into(),
                    user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                        error_message: "Failed to query user.".into(),
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                });
                return Some(udp_messages);
            }
        };

        // If the user exist in DB, check if he is already localy registered, if yes send error else register it localy and send success
        if let Some(user) = user {
            let mut users_state_lock = app.get_users_state_lock();
            if users_state_lock
                .user_uuid_user_map
                .get(&user.uuid)
                .is_some()
            {
                udp_messages.push(UdpMsgDown {
                    _type: UdpMsgDownType::USER_CONNECT_FAILED.into(),
                    user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                        error_message: "User is already connected from another client.".into(),
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                })
            } else {
                users_state_lock
                    .user_uuid_user_map
                    .insert(user.uuid, User::new(user.uuid, udp_peer_id));
                users_state_lock
                    .udp_peer_id_user_uuid_map
                    .insert(udp_peer_id, user.uuid);

                udp_messages.push(UdpMsgDown {
                    _type: UdpMsgDownType::USER_CONNECT_SUCCESS.into(),
                    ..Default::default()
                })
            }

            return Some(udp_messages);
        }

        // Else if the user does not exist in DB, create it register it localy and send success
        let user_uuid = Uuid::new_v4();
        let insert_result = sqlx::query!(
            "INSERT INTO users (uuid, username) VALUES ($1, $2)",
            user_uuid,
            username.clone()
        )
        .fetch_all(&app.pg_pool)
        .await;

        match insert_result {
            Ok(_) => {
                {
                    let mut users_state_lock = app.get_users_state_lock();
                    users_state_lock
                        .user_uuid_user_map
                        .insert(user_uuid, User::new(user_uuid, udp_peer_id));
                    users_state_lock
                        .udp_peer_id_user_uuid_map
                        .insert(udp_peer_id, user_uuid);
                }

                udp_messages.push(UdpMsgDown {
                    _type: UdpMsgDownType::USER_CONNECT_SUCCESS.into(),
                    ..Default::default()
                })
            }
            Err(err) => {
                println!(
                    "[ApiServiceUser][connect] Failed to insert user, error: {}",
                    err
                );
                udp_messages.push(UdpMsgDown {
                    _type: UdpMsgDownType::USER_CONNECT_FAILED.into(),
                    user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                        error_message: "Failed to register user.".into(),
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                });
            }
        }

        Some(udp_messages)
    }

    pub async fn disconnect(app: App, user: &User) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        ApiServiceArea::leave(app.clone(), user);

        {
            let mut users_state_lock = app.get_users_state_lock();
            users_state_lock
                .udp_peer_id_user_uuid_map
                .remove(&user.udp_peer_id);
            users_state_lock.user_uuid_user_map.remove(&user.uuid);
        }

        udp_messages.push(UdpMsgDown {
            _type: UdpMsgDownType::USER_DISCONNECT_SUCCESS.into(),
            ..Default::default()
        });

        Some(udp_messages)
    }
}
