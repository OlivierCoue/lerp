use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;

use crate::{
    character_controller::CharacterController,
    health::Health,
    physics::PhysicsBundle,
    protocol::*,
    shared::{PLAYER_BASE_HEALTH, PLAYER_BASE_MOVEMENT_SPEED, PLAYER_SIZE},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    physics: PhysicsBundle,
    position: Position,
    character_controller: CharacterController,
    movement_speed: MovementSpeed,
    health: Health,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player(ClientId::Netcode(0)),
            physics: Self::physics(),
            position: Position::default(),
            character_controller: CharacterController,
            movement_speed: MovementSpeed(PLAYER_BASE_MOVEMENT_SPEED),
            health: Health::new(PLAYER_BASE_HEALTH),
        }
    }
}
impl PlayerBundle {
    pub fn new(client_id: ClientId, position: &Vec2) -> Self {
        Self {
            marker: Player(client_id),
            position: Position(*position),
            ..default()
        }
    }
    pub fn from_protocol() -> Self {
        Self { ..default() }
    }
    pub fn physics() -> PhysicsBundle {
        PhysicsBundle {
            rigid_body: RigidBody::Kinematic,
            collider: Collider::circle(PLAYER_SIZE / 2.),
        }
    }
}
