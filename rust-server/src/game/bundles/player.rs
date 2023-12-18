use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::proto::common::GameEntityBaseType;

use crate::game::components::prelude::*;

#[derive(Component, Default)]
pub struct Player {}

#[derive(Bundle)]
pub struct PlayerBundle {
    game_entity: GameEntity,
    player: Player,
    position: Position,
    velocity: Velocity,
    shape: Shape,
    health: Health,
    cannot_go_through: MovementCollider,
}
impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::CHARACTER),
            player: Player::default(),
            position: Position {
                current: Vector2::ZERO,
            },
            velocity: Velocity::new(None, 400.0, false),
            shape: Shape {
                rect: Vector2 { x: 50.0, y: 50.0 },
            },
            health: Health {
                max: 100,
                min: 0,
                current: 100,
            },
            cannot_go_through: MovementCollider {},
        }
    }
}
