use bevy_ecs::prelude::*;
use rust_common::{collisions::ColliderShape, math::Vec2, proto::GameEntityBaseType};

use crate::game::ecs::components::prelude::*;

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
    pub fn new(position: Vec2) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::Character),
            player: Player::default(),
            position: Position::new(position),
            velocity: Velocity::new(None, 400.0, false),
            collider_dmg_in: ColliderDmgIn::new(Vec2 { x: 50.0, y: 50.0 }),
            collider_mvt: ColliderMvt::new(ColliderShape::new_circle(14.0, false)),
            health: Health::new(900),
            team: Team::Player,
        }
    }
}
