use bevy_ecs::prelude::*;
use rust_common::{collisions::ColliderShape, math::Vec2, proto::GameEntityBaseType};

use crate::game::ecs::components::prelude::*;

#[derive(Bundle)]
pub struct WallBundle {
    game_entity: GameEntity,
    position: Position,
    collider_dmg_in: ColliderDmgIn,
    collider_mvt: ColliderMvt,
}
impl WallBundle {
    #[allow(dead_code)]
    pub fn new_rect(position_current: Vec2, rect: Vec2) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::Wall),
            position: Position::new(position_current),
            collider_dmg_in: ColliderDmgIn::new(rect),
            collider_mvt: ColliderMvt::new(ColliderShape::new_rect(rect, false)),
        }
    }
    pub fn new_poly(poly: Vec<Vec2>, reversed: bool) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::Wall),
            position: Position::new(Vec2::new(0.0, 0.0)),
            collider_dmg_in: ColliderDmgIn::new(Vec2::new(0.0, 0.0)),
            collider_mvt: ColliderMvt::new(ColliderShape::new_poly(poly, reversed)),
        }
    }
}
