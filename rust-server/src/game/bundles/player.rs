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
}
impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::CHARACTER),
            player: Player::default(),
            position: Position {
                current: Vector2::ZERO,
            },
            velocity: Velocity::new(Vector2::ZERO, 600.0, false),
            shape: Shape {
                rect: Vector2 { x: 100.0, y: 200.0 },
            },
            health: Health {
                max: 100,
                min: 0,
                current: 100,
            },
        }
    }
}
