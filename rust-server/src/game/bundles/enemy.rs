use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::proto::common::GameEntityBaseType;

use crate::game::components::prelude::*;

#[derive(Component, Default)]
pub struct Enemie {
    pub last_action_at_millis: u32,
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
}
impl EnemyBundle {
    pub fn new(position_current: Vector2) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::ENEMY),
            enemie: Enemie::default(),
            position: Position::new(position_current),
            velocity: Velocity::new(Some(Vector2::new(1024.0, 1024.0)), 150.0, false),
            collider_dmg_in: ColliderDmgIn::new(Vector2 { x: 50.0, y: 50.0 }),
            collider_mvt: ColliderMvt::new(Vector2 { x: 20.0, y: 20.0 }),
            health: Health::new(10),
        }
    }
}
