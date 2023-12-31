use bevy_ecs::entity::Entity;
use rust_common::{helper::get_timestamp_millis, proto::udp_down::UdpMsgDownWrapper};

use std::{collections::HashMap, sync::mpsc::Sender};

pub struct User<'a> {
    pub id: u32,
    pub enet_peer_id: u16,
    pub tx_enet_sender: &'a Sender<(u16, UdpMsgDownWrapper)>,
    last_ping_at_millis: u128,
    pub player_entity: Entity,
    pub entity_id_revision_map: HashMap<u32, u32>,
}
impl<'a> User<'a> {
    pub fn new(
        id: u32,
        enet_peer_id: u16,
        tx_enet_sender: &'a Sender<(u16, UdpMsgDownWrapper)>,
        player_entity: Entity,
    ) -> User<'a> {
        User {
            id,
            enet_peer_id,
            tx_enet_sender,
            last_ping_at_millis: get_timestamp_millis(),
            player_entity,
            entity_id_revision_map: HashMap::new(),
        }
    }

    pub fn should_be_deleted(&self) -> bool {
        get_timestamp_millis() - self.last_ping_at_millis > 5000
    }

    pub fn send_message(&self, msg: UdpMsgDownWrapper) {
        if let Err(err) = self.tx_enet_sender.send((self.enet_peer_id, msg)) {
            println!("Failed to send thread message from player");
            println!("{}", err)
        }
    }

    pub fn user_ping(&mut self) {
        self.last_ping_at_millis = get_timestamp_millis()
    }
}
