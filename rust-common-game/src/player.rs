use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;

use crate::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    physics: PhysicsBundle,
    position: Position,
    character_controller: CharacterController,
    movement_speed: MovementSpeed,
    health: Health,
    mana: Mana,
    skill_slot_map: SkillSlotMap,
    pub skills_available: SkillsAvailable,
    skill_speed: SkillSpeed,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        let mut skill_slot_map = SkillSlotMap::default();
        skill_slot_map.insert(PlayerActions::SkillSlot1, SkillName::BowAttack);
        skill_slot_map.insert(PlayerActions::SkillSlot2, SkillName::SplitArrow);
        skill_slot_map.insert(PlayerActions::SkillSlot3, SkillName::FlowerArrow);
        Self {
            marker: Player(ClientId::Netcode(0)),
            physics: Self::physics(),
            position: Position::default(),
            character_controller: CharacterController,
            movement_speed: MovementSpeed(PLAYER_BASE_MOVEMENT_SPEED),
            health: Health::new(PLAYER_BASE_HEALTH),
            mana: Mana::new(PLAYER_BASE_MANA),
            skill_slot_map,
            skills_available: SkillsAvailable::default(),
            skill_speed: SkillSpeed {
                value: Duration::from_millis(200),
            },
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
