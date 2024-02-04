use bson::{doc, Document};
use rust_common::proto::udp_down::*;
use std::sync::{Arc, Mutex};

use self::service_area::ApiServiceArea;

use super::*;

pub struct ApiServiceUser {}
impl ApiServiceUser {
    pub async fn connect(
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

    pub async fn disconnect(
        mongo_client: mongodb::Client,
        connections_state: Arc<Mutex<ConnectionsState>>,
        user: &User,
    ) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

        ApiServiceArea::leave(user, connections_state.clone());

        {
            let mut connections_state_lock = connections_state.lock().unwrap();
            connections_state_lock
                .udp_peer_id_user_id_map
                .remove(&user.udp_peer_id);
            connections_state_lock.user_id_user_map.remove(&user._id);
        }

        let user_collection = mongo_client.database("main").collection::<Document>("user");

        let opt_mongo_user_in_word_instance = user_collection
            .find_one(
                doc! {
                    "_id": user._id,
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
}
