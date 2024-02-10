use rust_common::proto::{udp_down::UdpMsgDownWrapper, udp_up::MsgUp};
use sqlx::Postgres;
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
    thread::JoinHandle,
};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::game::internal_message::{InboundAreaMessage, OutboundAreaMessage};

#[derive(Clone, Copy)]
pub struct User {
    pub uuid: Uuid,
    pub udp_peer_id: u16,
    pub current_world_instance_uuid: Option<Uuid>,
}
impl User {
    pub fn new(_id: Uuid, udp_peer_id: u16) -> Self {
        Self {
            uuid: _id,
            udp_peer_id,
            current_world_instance_uuid: None,
        }
    }
}

pub struct WorldInstance {
    pub uuid: Uuid,
    pub user_uuids: HashMap<Uuid, bool>,
    pub udp_msg_up_dequeue: Arc<Mutex<VecDeque<(u16, MsgUp)>>>,
    pub to_instance_internal_messages: Arc<Mutex<VecDeque<InboundAreaMessage>>>,
    pub thread_join_handle: JoinHandle<()>,
}

#[derive(Default)]
pub struct UsersState {
    pub udp_peer_id_user_uuid_map: HashMap<u16, Uuid>,
    pub user_uuid_user_map: HashMap<Uuid, User>,
    pub world_instance_map: HashMap<Uuid, WorldInstance>,
}

#[derive(Clone)]
pub struct App {
    users_state: Arc<Mutex<UsersState>>,
    pg_pool: sqlx::Pool<Postgres>,
    pub tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
    pub tx_from_instance_internal_messages: mpsc::Sender<OutboundAreaMessage>,
}
impl App {
    pub fn new(
        pg_pool: sqlx::Pool<Postgres>,
        tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
        tx_from_instance_internal_messages: mpsc::Sender<OutboundAreaMessage>,
    ) -> Self {
        Self {
            users_state: Arc::new(Mutex::new(UsersState::default())),
            pg_pool,
            tx_udp_sender,
            tx_from_instance_internal_messages,
        }
    }

    pub fn get_users_state_lock(&self) -> std::sync::MutexGuard<'_, UsersState> {
        self.users_state.lock().unwrap()
    }

    pub fn pg_pool(&self) -> &sqlx::Pool<Postgres> {
        &self.pg_pool
    }
}
