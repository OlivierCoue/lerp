use rust_common::proto::*;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

use crate::game::internal_message::{InboundAreaMessage, PlayerInitPayload, PlayerLeavePayload};
use crate::game::Game;

use super::*;
pub struct ApiServiceArea {}
impl ApiServiceArea {
    pub async fn create(app: App, user: &User) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        let world_instance_uuid = Uuid::new_v4();

        if let Err(err) = sqlx::query!(
            "INSERT INTO game_worlds (uuid, created_by) VALUES ($1, $2);",
            world_instance_uuid,
            user.uuid,
        )
        .execute(app.pg_pool())
        .await
        {
            println!("[ApiServiceArea][create] Error: {}", err);
            return None;
        }

        let mut users_state_lock = app.get_users_state_lock();

        let received_internal_messages_1 = Arc::new(Mutex::new(VecDeque::new()));
        let udp_msg_up_dequeue_1 = Arc::new(Mutex::new(VecDeque::new()));

        let received_internal_messages_2 = Arc::clone(&received_internal_messages_1);
        let udp_msg_up_dequeue_2 = Arc::clone(&udp_msg_up_dequeue_1);

        let tx_udp_sender = app.tx_udp_sender.clone();
        let tx_from_instance_internal_messages = app.tx_from_instance_internal_messages.clone();

        let thread_join_handle = thread::spawn(move || {
            let mut game: Game = Game::new(
                world_instance_uuid,
                tx_udp_sender,
                received_internal_messages_1,
                tx_from_instance_internal_messages,
                udp_msg_up_dequeue_1,
            );
            game.start();
        });

        users_state_lock.world_instance_map.insert(
            world_instance_uuid,
            WorldInstance {
                uuid: world_instance_uuid,
                user_uuids: HashMap::new(),
                to_instance_internal_messages: received_internal_messages_2,
                udp_msg_up_dequeue: udp_msg_up_dequeue_2,
                thread_join_handle,
            },
        );

        udp_messages.push(UdpMsgDown {
            r#type: UdpMsgDownType::UserCreateWordlInstanceSuccess.into(),
            user_create_world_instance_success: Some(UdpMsgDownUserCreateWorldInstanceSuccess {
                id: world_instance_uuid.to_string(),
            }),
            ..Default::default()
        });

        Some(udp_messages)
    }

    pub async fn join(
        app: App,
        user: &User,
        world_instance_uuid: String,
    ) -> Option<Vec<UdpMsgDown>> {
        let Ok(world_instance_uuid) = Uuid::from_str(&world_instance_uuid) else {
            println!(
                "[ApiServiceArea][join] Invalid world_instance_uuid: {}",
                world_instance_uuid
            );
            return None;
        };

        let mut pg_tx = app.pg_pool().begin().await.unwrap();

        let update_result = sqlx::query!(
            "UPDATE users SET current_game_world_uuid = $1 WHERE uuid = $2 AND current_game_world_uuid IS NULL;",
            world_instance_uuid,
            user.uuid,
        )
        .execute(&mut *pg_tx)
        .await.unwrap();

        if update_result.rows_affected() != 1 {
            println!("[ApiServiceArea][join] User is already in an instance (globaly).");
            return None;
        }

        let mut success = false;
        {
            let mut users_state_lock = app.get_users_state_lock();
            if let Some(mut world_instance) = users_state_lock
                .world_instance_map
                .remove(&world_instance_uuid)
            {
                if let Some(user) = users_state_lock.user_uuid_user_map.get_mut(&user.uuid) {
                    if world_instance.user_uuids.get(&user.uuid).is_none()
                        && user.current_world_instance_uuid.is_none()
                    {
                        user.current_world_instance_uuid = Some(world_instance_uuid);
                        world_instance.user_uuids.insert(user.uuid, true);
                        if let Ok(mut received_internal_messages) =
                            world_instance.to_instance_internal_messages.lock()
                        {
                            received_internal_messages.push_back(InboundAreaMessage::PlayerInit(
                                PlayerInitPayload {
                                    user_uuid: user.uuid,
                                    udp_peer_id: user.udp_peer_id,
                                },
                            ));
                            success = true;
                        } else {
                            println!("[ApiServiceArea][join] Failed to get received_internal_messages lock, area may have crashed.")
                        }
                    } else {
                        println!("[ApiServiceArea][join] User is already in an instance (localy).")
                    }
                }

                users_state_lock
                    .world_instance_map
                    .insert(world_instance_uuid, world_instance);
            }
        }

        if success {
            pg_tx.commit().await.unwrap();
        } else {
            pg_tx.rollback().await.unwrap();
        }

        None
    }

    pub async fn close(app: App, area_uuid: Uuid) {
        {
            let mut users_state = app.get_users_state_lock();
            if let Some(area_instance) = users_state.world_instance_map.remove(&area_uuid) {
                for (user_uuid, _) in area_instance.user_uuids {
                    if let Some(user) = users_state.user_uuid_user_map.get_mut(&user_uuid) {
                        if user.current_world_instance_uuid == Some(area_uuid) {
                            user.current_world_instance_uuid = None;
                        }
                    }
                }
            }
        }

        sqlx::query!(r#"DELETE FROM game_worlds WHERE uuid = $1;"#, area_uuid)
            .execute(app.pg_pool())
            .await
            .unwrap();
    }

    pub async fn leave(app: App, user: &User) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        let Some(instance_uuid) = user.current_world_instance_uuid else {
            return None;
        };

        {
            let mut users_state_lock = app.get_users_state_lock();
            let Some(user_mut) = users_state_lock.user_uuid_user_map.get_mut(&user.uuid) else {
                return None;
            };
            user_mut.current_world_instance_uuid = None;

            let Some(instance) = users_state_lock.world_instance_map.get_mut(&instance_uuid) else {
                return None;
            };

            instance
                .to_instance_internal_messages
                .lock()
                .unwrap()
                .push_back(InboundAreaMessage::PlayerLeave(PlayerLeavePayload {
                    user_uuid: user.uuid,
                }));
            instance.user_uuids.remove(&user.uuid);
        }

        sqlx::query!(
            "UPDATE users SET current_game_world_uuid = NULL WHERE uuid = $1;",
            user.uuid,
        )
        .execute(app.pg_pool())
        .await
        .unwrap();

        udp_messages.push(UdpMsgDown {
            r#type: UdpMsgDownType::UserLeaveWorldInstanceSuccess.into(),
            ..Default::default()
        });

        Some(udp_messages)
    }

    pub fn forward_msg(app: App, user: &User, udp_msg_up: &MsgUp) {
        let users_state_lock = app.get_users_state_lock();

        if let Some(world_istance_id) = &user.current_world_instance_uuid {
            if let Some(wolrd_instance) = users_state_lock.world_instance_map.get(world_istance_id)
            {
                wolrd_instance
                    .udp_msg_up_dequeue
                    .lock()
                    .unwrap()
                    .push_back((user.udp_peer_id, udp_msg_up.clone()))
            }
        }
    }
}
