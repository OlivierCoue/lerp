use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use bson::oid::ObjectId;
use rust_common::proto::udp_up::MsgUp;

use crate::game::internal_message::InboundAreaMessage;

#[derive(Clone, Copy)]
pub struct User {
    pub _id: ObjectId,
    pub udp_peer_id: u16,
    pub current_world_instance_id: Option<ObjectId>,
}
impl User {
    pub fn new(_id: ObjectId, udp_peer_id: u16) -> Self {
        Self {
            _id,
            udp_peer_id,
            current_world_instance_id: None,
        }
    }
}

#[derive(Default)]
pub struct WorldInstance {
    pub _id: ObjectId,
    pub user_ids: Vec<ObjectId>,
    pub udp_msg_up_dequeue: Arc<Mutex<VecDeque<(u16, MsgUp)>>>,
    pub received_internal_messages: Arc<Mutex<VecDeque<InboundAreaMessage>>>,
}

#[derive(Default)]
pub struct ConnectionsState {
    pub udp_peer_id_user_id_map: HashMap<u16, ObjectId>,
    pub user_id_user_map: HashMap<ObjectId, User>,
    pub world_instance_map: HashMap<ObjectId, WorldInstance>,
}
