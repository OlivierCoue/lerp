use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::proto::common::GameEntityBaseType;

use crate::game::components::prelude::*;

#[derive(Bundle)]
pub struct WallBundle {
    game_entity: GameEntity,
    position: Position,
    collider_dmg_in: ColliderDmgIn,
    collider_mvt: ColliderMvt,
}
impl WallBundle {
    pub fn new(position_current: Vector2, size: Vector2) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::WALL),
            position: Position::new(position_current),
            collider_dmg_in: ColliderDmgIn::new(size),
            collider_mvt: ColliderMvt::new(size),
        }
    }
}
