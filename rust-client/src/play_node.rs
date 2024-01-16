use std::collections::HashMap;

use godot::{engine::ISprite2D, prelude::*};
use rust_common::proto::udp_down::{UdpMsgDownGameEntityRemoved, UdpMsgDownGameEntityUpdate};

use crate::{
    entity::GameEntity, play_node_debug::PlayNodeDebug, root::DEBUG,
    server_entity::GameServerEntity,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayNode {
    #[base]
    base: Base<Node2D>,

    entities: HashMap<u32, Gd<GameEntity>>,
    server_entities: HashMap<u32, Gd<GameServerEntity>>,
}

#[godot_api]
impl INode2D for PlayNode {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("PlayNode init");

        Self {
            base,
            entities: HashMap::new(),
            server_entities: HashMap::new(),
        }
    }

    fn ready(&mut self) {
        self.base.set_y_sort_enabled(true);
        let play_node_debug = Gd::<PlayNodeDebug>::from_init_fn(PlayNodeDebug::init);
        self.base.add_child(play_node_debug.upcast());
    }
}

impl PlayNode {
    pub fn update_entity(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        if let Some(entity) = self.entities.get_mut(&entity_update.id) {
            entity.bind_mut().update_from_server(entity_update);
        } else {
            let mut entity = Gd::<GameEntity>::from_init_fn(GameEntity::init);
            entity.bind_mut().set_init_state(entity_update);
            self.entities.insert(entity_update.id, entity.clone());
            self.base.add_child(entity.upcast());
        }
        if DEBUG {
            if let Some(entity) = self.server_entities.get_mut(&entity_update.id) {
                entity.bind_mut().update_from_server(entity_update);
            } else {
                let mut entity = Gd::<GameServerEntity>::from_init_fn(GameServerEntity::init);
                entity.bind_mut().set_init_state(entity_update);
                self.server_entities
                    .insert(entity_update.id, entity.clone());
                self.base.add_child(entity.upcast());
            }
        }
    }

    pub fn remove_entity(&mut self, entity_removed: &UdpMsgDownGameEntityRemoved) {
        if let Some(mut entity) = self.entities.remove(&entity_removed.id) {
            entity.queue_free()
        }
        if DEBUG {
            if let Some(mut entity) = self.server_entities.remove(&entity_removed.id) {
                entity.queue_free()
            }
        }
    }
}
