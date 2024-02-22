use bevy_ecs::prelude::*;
use rust_common::{collisions::ColliderShape, math::Vec2, proto::GameEntityBaseType};

use crate::game::ecs::components::prelude::*;

#[derive(Component, Default)]
pub struct Enemie {
    pub last_action_at_millis: u32,
    pub is_wizard: bool,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    game_entity: GameEntity,
    enemie: Enemie,
    position: Position,
    velocity: Velocity,
    collider_dmg_in: ColliderDmgIn,
    collider_mvt: ColliderMvt,
    health: Health,
    team: Team,
}
impl EnemyBundle {
    pub fn new(position_current: Vec2, is_wizard: bool) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::Enemy),
            enemie: Enemie {
                last_action_at_millis: 0,
                is_wizard,
            },
            position: Position::new(position_current),
            velocity: Velocity::new(Some(Vec2::new(1024.0, 1024.0)), 150.0, false),
            collider_dmg_in: ColliderDmgIn::new(Vec2 { x: 50.0, y: 50.0 }),
            collider_mvt: ColliderMvt::new(
                ColliderShape::new_rect(Vec2 { x: 20.0, y: 20.0 }),
                false,
            ),
            health: Health::new(10),
            team: Team::Enemy,
        }
    }
}
