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
    shape: Shape,
    health: Health,
    cannot_go_through: MovementCollider,
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
            shape: Shape {
                rect: Vector2 { x: 50.0, y: 50.0 },
            },
            health: Health {
                max: 10,
                min: 0,
                current: 10,
            },
            cannot_go_through: MovementCollider {},
        }
    }
}
