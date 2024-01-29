use bevy_ecs::entity::Entity;
use bson::oid::ObjectId;
use rust_common::proto::udp_down::UdpMsgDownWrapper;

use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct Player {
    pub id: ObjectId,
    pub enet_peer_id: u16,
    pub tx_enet_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
    pub player_entity: Entity,
    pub entity_id_revision_map: HashMap<u32, u32>,
}
impl Player {
    pub fn new(
        id: ObjectId,
        enet_peer_id: u16,
        tx_enet_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
        player_entity: Entity,
    ) -> Player {
        Player {
            id,
            enet_peer_id,
            tx_enet_sender,
            player_entity,
            entity_id_revision_map: HashMap::new(),
        }
    }

    pub fn send_message(&self, msg: UdpMsgDownWrapper) {
        if let Err(err) = self.tx_enet_sender.blocking_send((self.enet_peer_id, msg)) {
            println!("Failed to send thread message from player");
            println!("{}", err)
        }
    }
}
