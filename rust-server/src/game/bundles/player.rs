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
    collider_dmg_in: ColliderDmgIn,
    collider_mvt: ColliderMvt,
    health: Health,
    team: Team,
}
impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::CHARACTER),
            player: Player::default(),
            position: Position::new(Vector2::ZERO),
            velocity: Velocity::new(None, 400.0, false),
            collider_dmg_in: ColliderDmgIn::new(Vector2 { x: 50.0, y: 50.0 }),
            collider_mvt: ColliderMvt::new(Vector2 { x: 20.0, y: 20.0 }),
            health: Health::new(900),
            team: Team::Player,
        }
    }
}
