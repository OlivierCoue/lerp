use std::str::FromStr;

use rust_common::proto::*;
use uuid::Uuid;

use self::service_area::ApiServiceArea;
use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};

use super::*;

pub struct ApiServiceUser {}
impl ApiServiceUser {
    pub async fn connect(
        app: App,
        udp_peer_id: u16,
        user_uuid: String,
        signed_message: String,
    ) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        let Ok(user_uuid) = Uuid::from_str(&user_uuid) else {
            udp_messages.push(UdpMsgDown {
                r#type: UdpMsgDownType::UserConnectFailed.into(),
                user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                    error_message: "Invalid user_uuid format.".into(),
                }),
                ..Default::default()
            });

            return Some(udp_messages);
        };

        // If a user is already localy register with the incoming udp_peer_id send an error
        if app
            .get_users_state_lock()
            .udp_peer_id_user_uuid_map
            .get(&udp_peer_id)
            .is_some()
        {
            udp_messages.push(UdpMsgDown {
                r#type: UdpMsgDownType::UserConnectFailed.into(),
                user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                    error_message: "A user is already connected on this client.".into(),
                }),
                ..Default::default()
            });

            return Some(udp_messages);
        };

        struct PgResult {
            uuid: Uuid,
            game_server_aes_key: Option<String>,
            game_server_aes_nonce: Option<String>,
            game_server_handshake_challenge: Option<Uuid>,
        }

        let user = sqlx::query_as!(
            PgResult,
            r#"
            SELECT 
                uuid,
                game_server_aes_key,
                game_server_aes_nonce,
                game_server_handshake_challenge 
            FROM users 
            WHERE uuid = $1"#,
            user_uuid
        )
        .fetch_optional(app.pg_pool())
        .await;

        let user = match user {
            Ok(opt_user) => opt_user,
            Err(err) => {
                println!(
                    "[ApiServiceUser][connect] Failed to query user, error: {}",
                    err
                );
                udp_messages.push(UdpMsgDown {
                    r#type: UdpMsgDownType::UserConnectFailed.into(),
                    user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                        error_message: "Failed to query user.".into(),
                    }),
                    ..Default::default()
                });
                return Some(udp_messages);
            }
        };

        let Some(user) = user else {
            udp_messages.push(UdpMsgDown {
                r#type: UdpMsgDownType::UserConnectFailed.into(),
                user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                    error_message: "Invalid user_uuid.".into(),
                }),
                ..Default::default()
            });
            return Some(udp_messages);
        };

        let (
            Some(game_server_aes_key),
            Some(game_server_aes_nonce),
            Some(game_server_handshake_challenge),
        ) = (
            user.game_server_aes_key,
            user.game_server_aes_nonce,
            user.game_server_handshake_challenge,
        )
        else {
            udp_messages.push(UdpMsgDown {
                r#type: UdpMsgDownType::UserConnectFailed.into(),
                user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                    error_message: "Invalid user keys state.".into(),
                }),
                ..Default::default()
            });
            return Some(udp_messages);
        };

        let vec_u8_aes_key = hex::decode(game_server_aes_key).unwrap();
        let cipher = Aes256GcmSiv::new_from_slice(&vec_u8_aes_key[..]).unwrap();
        let game_server_aes_nonce = game_server_aes_nonce.to_string();
        let game_server_aes_nonce = game_server_aes_nonce.as_bytes();
        let nonce = Nonce::from_slice(game_server_aes_nonce);

        let plain_message = cipher
            .decrypt(nonce, hex::decode(signed_message).unwrap().as_slice())
            .unwrap();

        if String::from_utf8(plain_message.clone()).unwrap()
            != game_server_handshake_challenge.to_string()
        {
            println!("Failed");
            udp_messages.push(UdpMsgDown {
                r#type: UdpMsgDownType::UserConnectFailed.into(),
                user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                    error_message: "Invalid signed_message.".into(),
                }),
                ..Default::default()
            });
            return Some(udp_messages);
        }

        println!("Success");

        let mut users_state_lock = app.get_users_state_lock();
        if users_state_lock
            .user_uuid_user_map
            .get(&user.uuid)
            .is_some()
        {
            udp_messages.push(UdpMsgDown {
                r#type: UdpMsgDownType::UserConnectFailed.into(),
                user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                    error_message: "User is already connected from another client.".into(),
                }),
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
                r#type: UdpMsgDownType::UserConnectSuccess.into(),
                ..Default::default()
            })
        }

        Some(udp_messages)
    }

    pub async fn disconnect(app: App, user: &User) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        ApiServiceArea::leave(app.clone(), user).await;

        {
            let mut users_state_lock = app.get_users_state_lock();
            users_state_lock
                .udp_peer_id_user_uuid_map
                .remove(&user.udp_peer_id);
            users_state_lock.user_uuid_user_map.remove(&user.uuid);
        }

        udp_messages.push(UdpMsgDown {
            r#type: UdpMsgDownType::UserDisconnectSuccess.into(),
            ..Default::default()
        });

        Some(udp_messages)
    }
}
