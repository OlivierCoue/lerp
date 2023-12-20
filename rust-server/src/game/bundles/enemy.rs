use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::proto::common::GameEntityBaseType;

use crate::game::components::prelude::*;

#[derive(Component, Default)]
pub struct Enemie {}

#[derive(Bundle)]
pub struct EnemyBundle {
    game_entity: GameEntity,
    enemie: Enemie,
    position: Position,
    velocity: Velocity,
    collider_dmg_in: ColliderDmgIn,
    collider_mvt: ColliderMvt,
    health: Health,
}
impl EnemyBundle {
    pub fn new(position_current: Vector2) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::ENEMY),
            enemie: Enemie::default(),
            position: Position {
                current: position_current,
            },
            velocity: Velocity::new(Some(Vector2::ZERO), 200.0, false),
            collider_dmg_in: ColliderDmgIn::new(Vector2 { x: 50.0, y: 50.0 }),
            collider_mvt: ColliderMvt::new(Vector2 { x: 10.0, y: 10.0 }),
            health: Health {
                max: 10,
                min: 0,
                current: 10,
            },
        }
    }
}
