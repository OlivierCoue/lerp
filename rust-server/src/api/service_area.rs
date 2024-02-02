use bson::oid::ObjectId;
use bson::{doc, Document};
use rust_common::proto::{udp_down::*, udp_up::*};
use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;

use crate::game::internal_message::{InboundAreaMessage, PlayerInitPayload};
use crate::game::Game;

use super::*;
pub struct ApiServiceArea {}
impl ApiServiceArea {
    pub async fn create(
        mongo_client: mongodb::Client,
        connections_state: Arc<Mutex<ConnectionsState>>,
        tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
        user: &User,
    ) -> Option<Vec<UdpMsgDown>> {
        let mut udp_messages = Vec::new();

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

    pub async fn join(
        mongo_client: mongodb::Client,
        connections_state: Arc<Mutex<ConnectionsState>>,
        user: &User,
        world_instance_id: String,
    ) -> Option<Vec<UdpMsgDown>> {
        let Ok(world_instance_id) = ObjectId::from_str(&world_instance_id) else {
            println!(
                "[user_join_world_instance] invalid world_instance_id: {}",
                world_instance_id
            );
            return None;
        };

        let mut success = false;
        {
            let mut connections_state_lock = connections_state.lock().unwrap();
            if let Some(mut wolrd_instance) = connections_state_lock
                .world_instance_map
                .remove(&world_instance_id)
            {
                if let Some(user) = connections_state_lock.user_id_user_map.get_mut(&user._id) {
                    if !wolrd_instance.user_ids.contains(&user._id)
                        && user.current_world_instance_id != Some(world_instance_id)
                    {
                        user.current_world_instance_id = Some(world_instance_id);
                        wolrd_instance.user_ids.push(user._id);
                        if let Ok(mut received_internal_messages) =
                            wolrd_instance.received_internal_messages.lock()
                        {
                            received_internal_messages.push_back(InboundAreaMessage::PlayerInit(
                                PlayerInitPayload {
                                    user_id: user._id,
                                    udp_peer_id: user.udp_peer_id,
                                },
                            ));
                            success = true;
                        } else {
                            println!("[user_join_world_instance] Failed to get received_internal_messages lock, area may have crashed.")
                        }
                    }
                }

                connections_state_lock
                    .world_instance_map
                    .insert(world_instance_id, wolrd_instance);
            }
        }

        if success {
            let wolrd_instance_collection = mongo_client
                .database("main")
                .collection::<Document>("world_instance");
            wolrd_instance_collection
                .update_one(
                    doc! { "_id": world_instance_id },
                    doc! { "$set": { "user_ids": { user._id.to_string() : user._id } } },
                    None,
                )
                .await
                .unwrap();
        }

        None
    }

    pub fn forward_msg(
        user: &User,
        connections_state: Arc<Mutex<ConnectionsState>>,
        udp_msg_up: &MsgUp,
    ) {
        let connections_state_lock = connections_state.lock().unwrap();

        if let Some(world_istance_id) = &user.current_world_instance_id {
            if let Some(wolrd_instance) = connections_state_lock
                .world_instance_map
                .get(world_istance_id)
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
