use bson::oid::ObjectId;
use bson::{doc, Document};
use mongodb;
use rust_common::proto::udp_down::{
    UdpMsgDown, UdpMsgDownType, UdpMsgDownUserConnectFailed,
    UdpMsgDownUserCreateWorldInstanceSuccess,
};
use rust_common::proto::udp_up::{UdpMsgUp, UdpMsgUpType};
use rust_common::proto::{udp_down::UdpMsgDownWrapper, udp_up::UdpMsgUpWrapper};
use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;

use crate::game::internal_message::{InboundAreaMessage, PlayerInitPayload};
use crate::game::Game;

use super::*;

pub struct ApiService {}
impl ApiService {
    pub async fn handle_msg_up_wrapper(
        mongo_client: mongodb::Client,
        tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
        udp_peer_id: u16,
        connections_state: Arc<Mutex<ConnectionsState>>,
        udp_msg_up_wrapper: UdpMsgUpWrapper,
    ) {
        let mut opt_udp_messages_down = None;
        for udp_msg_up in udp_msg_up_wrapper.messages {
            match udp_msg_up._type.enum_value() {
                Err(_) => {
                    println!("[UsersManager][handle_msg_up_wrapper] Received invalid enum value in udp_msg_up.")
                }
                Ok(udp_msg_up_type) => match udp_msg_up_type {
                    UdpMsgUpType::USER_CONNECT => {
                        if let Some(username) = udp_msg_up.user_connect_username {
                            opt_udp_messages_down = Self::user_connect(
                                mongo_client.clone(),
                                udp_peer_id,
                                connections_state.clone(),
                                username,
                            )
                            .await;
                        }
                    }
                    UdpMsgUpType::USER_DISCONNECT => {
                        opt_udp_messages_down = Self::user_disconnect(
                            mongo_client.clone(),
                            connections_state.clone(),
                            udp_peer_id,
                        )
                        .await;
                    }
                    UdpMsgUpType::USER_CREATE_WORLD_INSTANCE => {
                        opt_udp_messages_down = Self::user_create_world_instance(
                            mongo_client.clone(),
                            connections_state.clone(),
                            udp_peer_id,
                            tx_udp_sender.clone(),
                        )
                        .await;
                    }
                    UdpMsgUpType::USER_JOIN_WOLD_INSTANCE => {
                        if let Some(payload) = udp_msg_up.user_join_world_instance.into_option() {
                            println!("USER_JOIN_WOLD_INSTANCE {}", payload.id);
                            opt_udp_messages_down = Self::user_join_world_instance(
                                mongo_client.clone(),
                                connections_state.clone(),
                                udp_peer_id,
                                payload.id,
                            )
                            .await;
                        }
                    }
                    _ => Self::queue_udp_msg_up_for_game_instance(
                        udp_peer_id,
                        connections_state.clone(),
                        udp_msg_up,
                    ),
                },
            }
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

    async fn user_connect(
        mongo_client: mongodb::Client,
        udp_peer_id: u16,
        connections_state: Arc<Mutex<ConnectionsState>>,
        username: String,
    ) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();
        let user_collection = mongo_client.database("main").collection::<Document>("user");

        // If a user is already localy register with the incoming udp_peer_id send an error
        if connections_state
            .lock()
            .unwrap()
            .udp_peer_id_user_id_map
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
            })
        } else {
            let existing_db_user = user_collection
                .find_one(
                    doc! {
                          "username": username.clone()
                    },
                    None,
                )
                .await;

            // If the user exist in DB, check if he is already localy registered, if yes send error else register it localy and send success
            if let Ok(Some(existing_db_user)) = existing_db_user {
                let _id = existing_db_user.get_object_id("_id").unwrap();
                let mut connections_state_lock = connections_state.lock().unwrap();
                if connections_state_lock.user_id_user_map.get(&_id).is_some() {
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
                    connections_state_lock
                        .user_id_user_map
                        .insert(_id, User::new(_id, udp_peer_id));
                    connections_state_lock
                        .udp_peer_id_user_id_map
                        .insert(udp_peer_id, _id);

                    udp_messages.push(UdpMsgDown {
                        _type: UdpMsgDownType::USER_CONNECT_SUCCESS.into(),
                        ..Default::default()
                    })
                }
            // Else if the user does not exist in DB, create it register it localy and send success
            } else {
                let new_user_doc = doc! {
                   "username": username.clone()
                };

                let insert_result = user_collection.insert_one(new_user_doc.clone(), None).await;

                if let Ok(insert_success) = insert_result {
                    let _id = insert_success.inserted_id.as_object_id().unwrap();

                    {
                        let mut connections_state_lock = connections_state.lock().unwrap();
                        connections_state_lock
                            .user_id_user_map
                            .insert(_id, User::new(_id, udp_peer_id));
                        connections_state_lock
                            .udp_peer_id_user_id_map
                            .insert(udp_peer_id, _id);
                    }

                    udp_messages.push(UdpMsgDown {
                        _type: UdpMsgDownType::USER_CONNECT_SUCCESS.into(),
                        ..Default::default()
                    })
                } else {
                    udp_messages.push(UdpMsgDown {
                        _type: UdpMsgDownType::USER_CONNECT_FAILED.into(),
                        user_connect_failed: Some(UdpMsgDownUserConnectFailed {
                            error_message: "Failed to register user.".into(),
                            ..Default::default()
                        })
                        .into(),
                        ..Default::default()
                    })
                }
            }
        }

        Some(udp_messages)
    }

    async fn user_disconnect(
        mongo_client: mongodb::Client,
        connections_state: Arc<Mutex<ConnectionsState>>,
        udp_peer_id: u16,
    ) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        let opt_user_id;
        {
            let mut connections_state_lock = connections_state.lock().unwrap();
            let Some(user_id) = connections_state_lock
                .udp_peer_id_user_id_map
                .remove(&udp_peer_id)
            else {
                return None;
            };
            connections_state_lock.user_id_user_map.remove(&user_id);
            opt_user_id = Some(user_id);
        }

        let Some(user_id) = opt_user_id else {
            return None;
        };

        let user_collection = mongo_client.database("main").collection::<Document>("user");

        let opt_mongo_user_in_word_instance = user_collection
            .find_one(
                doc! {
                    "_id": user_id,
                    "current_word_instance_id": { "$exists": true }
                },
                None,
            )
            .await
            .unwrap();

        if let Some(mongo_user_in_word_instance) = opt_mongo_user_in_word_instance {
            println!(
                "user is currently in world: {}",
                mongo_user_in_word_instance
                    .get_object_id("current_word_instance_id")
                    .unwrap()
            );
        } else {
            println!("user is not in any world");
        }

        udp_messages.push(UdpMsgDown {
            _type: UdpMsgDownType::USER_DISCONNECT_SUCCESS.into(),
            ..Default::default()
        });

        Some(udp_messages)
    }

    async fn user_create_world_instance(
        mongo_client: mongodb::Client,
        connections_state: Arc<Mutex<ConnectionsState>>,
        udp_peer_id: u16,
        tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
    ) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        let mut opt_user = None;
        {
            let connections_state_lock: std::sync::MutexGuard<'_, ConnectionsState> =
                connections_state.lock().unwrap();
            if let Some(user_id) = connections_state_lock
                .udp_peer_id_user_id_map
                .get(&udp_peer_id)
            {
                if let Some(user) = connections_state_lock.user_id_user_map.get(user_id) {
                    opt_user = Some(*user);
                }
            }
        }

        let Some(user) = opt_user else {
            return None;
        };

        let user_collection = mongo_client.database("main").collection::<Document>("user");
        let wolrd_instance_collection = mongo_client
            .database("main")
            .collection::<Document>("world_instance");

        let world_instance_id = ObjectId::new();
        user_collection
            .update_one(
                doc! {
                   "_id": user._id
                },
                doc! {
                    "$set": { "current_word_instance_id": world_instance_id },
                },
                None,
            )
            .await
            .unwrap();
        wolrd_instance_collection
            .insert_one(
                doc! {
                    "_id": world_instance_id,
                   "created_by": user._id
                },
                None,
            )
            .await
            .unwrap();

        let mut connections_state_lock = connections_state.lock().unwrap();
        let received_internal_messages = Arc::new(Mutex::new(VecDeque::new()));
        let udp_msg_up_dequeue = Arc::new(Mutex::new(VecDeque::new()));
        connections_state_lock.world_instance_map.insert(
            world_instance_id,
            WorldInstance {
                received_internal_messages: Arc::clone(&received_internal_messages),
                udp_msg_up_dequeue: Arc::clone(&udp_msg_up_dequeue),
                ..Default::default()
            },
        );
        thread::spawn(move || {
            let mut game = Game::new(
                tx_udp_sender,
                received_internal_messages,
                udp_msg_up_dequeue,
            );
            game.start();
        });

        udp_messages.push(UdpMsgDown {
            _type: UdpMsgDownType::USER_CREATE_WORDL_INSTANCE_SUCCESS.into(),
            user_create_world_instance_success: Some(UdpMsgDownUserCreateWorldInstanceSuccess {
                id: world_instance_id.to_string(),
                ..Default::default()
            })
            .into(),
            ..Default::default()
        });

        Some(udp_messages)
    }

    async fn user_join_world_instance(
        mongo_client: mongodb::Client,
        connections_state: Arc<Mutex<ConnectionsState>>,
        udp_peer_id: u16,
        world_instance_id: String,
    ) -> Option<Vec<UdpMsgDown>> {
        let Ok(world_instance_id) = ObjectId::from_str(&world_instance_id) else {
            println!(
                "[user_join_world_instance] invalid world_instance_id: {}",
                world_instance_id
            );
            return None;
        };

        let mut user_id_copy = None;

        {
            let mut connections_state_lock = connections_state.lock().unwrap();
            if let Some(mut wolrd_instance) = connections_state_lock
                .world_instance_map
                .remove(&world_instance_id)
            {
                if let Some(user_id) = connections_state_lock
                    .udp_peer_id_user_id_map
                    .remove(&udp_peer_id)
                {
                    if let Some(user) = connections_state_lock.user_id_user_map.get_mut(&user_id) {
                        if !wolrd_instance.user_ids.contains(&user_id)
                            && user.current_world_instance_id != Some(world_instance_id)
                        {
                            user.current_world_instance_id = Some(world_instance_id);
                            wolrd_instance.user_ids.push(user_id);
                            if let Ok(mut received_internal_messages) =
                                wolrd_instance.received_internal_messages.lock()
                            {
                                user_id_copy = Some(user_id);
                                received_internal_messages.push_back(
                                    InboundAreaMessage::PlayerInit(PlayerInitPayload {
                                        user_id,
                                        udp_peer_id,
                                    }),
                                );
                            } else {
                                println!("[UsersManager][user_join_world_instance] Failed to get received_internal_messages lock, area may have crashed.")
                            }
                        }
                    }
                    connections_state_lock
                        .udp_peer_id_user_id_map
                        .insert(udp_peer_id, user_id);
                }
                connections_state_lock
                    .world_instance_map
                    .insert(world_instance_id, wolrd_instance);
            }
        }

        if let Some(user_id) = user_id_copy {
            let wolrd_instance_collection = mongo_client
                .database("main")
                .collection::<Document>("world_instance");
            wolrd_instance_collection
                .update_one(
                    doc! { "_id": world_instance_id },
                    doc! { "$set": { "user_ids": { user_id.to_string() : user_id } } },
                    None,
                )
                .await
                .unwrap();
        }

        None
    }

    fn queue_udp_msg_up_for_game_instance(
        udp_peer_id: u16,
        connections_state: Arc<Mutex<ConnectionsState>>,
        udp_msg_up: UdpMsgUp,
    ) {
        let connections_state_lock = connections_state.lock().unwrap();
        if let Some(user_id) = connections_state_lock
            .udp_peer_id_user_id_map
            .get(&udp_peer_id)
        {
            if let Some(user) = connections_state_lock.user_id_user_map.get(user_id) {
                if let Some(world_istance_id) = &user.current_world_instance_id {
                    if let Some(wolrd_instance) = connections_state_lock
                        .world_instance_map
                        .get(world_istance_id)
                    {
                        wolrd_instance
                            .udp_msg_up_dequeue
                            .lock()
                            .unwrap()
                            .push_back((udp_peer_id, udp_msg_up))
                    }
                }
            }
        }
    }
}
