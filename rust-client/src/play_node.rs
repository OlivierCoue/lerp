use std::collections::HashMap;

use godot::{engine::ISprite2D, prelude::*};
use rust_common::proto::data::{UdpMsgDownGameEntityRemoved, UdpMsgDownGameEntityUpdate};

use crate::entity::GameEntity;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayNode {
    #[base]
    base: Base<Node2D>,

    entities: HashMap<u32, Gd<GameEntity>>,
}

#[godot_api]
impl INode2D for PlayNode {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("PlayNode init");

        Self {
            base,
            entities: HashMap::new(),
        }
    }

    fn ready(&mut self) {}
}

impl PlayNode {
    pub fn update_entity(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        if let Some(entity) = self.entities.get_mut(&entity_update.id) {
            entity.bind_mut().set_position_target(&Vector2 {
                x: entity_update.location_target.x as f32,
                y: entity_update.location_target.y as f32,
            })
        } else {
            let mut entity = Gd::<GameEntity>::from_init_fn(GameEntity::init);
            entity.bind_mut().set_init_state(entity_update);
            self.entities.insert(entity_update.id, entity.clone());
            self.base.add_child(entity.upcast());
        }
    }

    pub fn remove_entity(&mut self, entity_removed: &UdpMsgDownGameEntityRemoved) {
        if let Some(mut entity) = self.entities.remove(&entity_removed.id) {
            entity.queue_free()
        }
    }
}
