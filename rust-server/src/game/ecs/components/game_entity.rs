use bevy_ecs::prelude::*;
use rust_common::proto::common::GameEntityBaseType;

use crate::utils::get_id;

#[derive(Component)]
pub struct GameEntity {
    pub id: u32,
    pub revision: u32,
    pub _type: GameEntityBaseType,
    pub pending_despwan: bool,
}
impl GameEntity {
    pub fn new(_type: GameEntityBaseType) -> Self {
        Self {
            id: get_id() as u32,
            revision: 0,
            _type,
            pending_despwan: false,
        }
    }
}
