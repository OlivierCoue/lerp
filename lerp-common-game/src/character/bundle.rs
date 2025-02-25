use std::str::FromStr;

use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CharacterId {
    Player,
    Enemy,
}
impl CharacterId {
    pub fn data(&self) -> CharacterData {
        match self {
            Self::Player => CharacterData {
                team: Team::Player,
                health: PLAYER_BASE_HEALTH,
                collider_diameter: PLAYER_SIZE,
                movement_speed: PLAYER_BASE_MOVEMENT_SPEED,
            },
            Self::Enemy => CharacterData {
                team: Team::Enemy,
                health: ENEMY_BASE_HEALTH,
                collider_diameter: ENEMY_SIZE,
                movement_speed: ENEMY_BASE_MOVEMENT_SPEED,
            },
        }
    }

    pub fn animation_data(&self) -> String {
        match self {
            Self::Player => String::from_str("archer").unwrap(),
            Self::Enemy => String::from_str("enemy").unwrap(),
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Character {
    pub uid: u64,
    pub id: CharacterId,
}

#[derive(Component)]
pub struct CharacterLocal;

#[derive(Component)]
pub struct CharacterIsInit;

pub struct CharacterData {
    pub team: Team,
    pub health: f32,
    pub collider_diameter: f32,
    pub movement_speed: f32,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub marker: Character,
    pub position: Position,
    pub health: Health,
}
impl CharacterBundle {
    pub fn new(uid: u64, id: CharacterId, position: &Vec2) -> Self {
        let data = id.data();
        Self {
            marker: Character { uid, id },
            position: Position(*position),
            health: Health::new(data.health),
        }
    }
}

#[derive(Bundle)]
pub struct CharacterLocalBundle {
    marker: CharacterLocal,
    team: Team,
}

impl CharacterLocalBundle {
    pub fn new(team: Team) -> Self {
        Self {
            marker: CharacterLocal,
            team,
        }
    }
}

#[derive(Bundle)]
pub struct CharacterAliveBundle {
    marker: Alive,
    physics: PhysicsBundle,
    hittable: Hittable,
    character_controller: CharacterController,
    movement_speed: MovementSpeed,
}
impl CharacterAliveBundle {
    pub fn init(alive_data: &CharacterData) -> Self {
        Self {
            marker: Alive,
            character_controller: CharacterController,
            hittable: Hittable::default(),
            physics: PhysicsBundle {
                rigid_body: RigidBody::Kinematic,
                collider: Collider::circle(alive_data.collider_diameter / 2.),
            },
            movement_speed: MovementSpeed(alive_data.movement_speed),
        }
    }
}
