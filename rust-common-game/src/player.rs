use std::time::Duration;

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    character: CharacterBundle,
    mana: Mana,
    skill_slot_map: SkillSlotMap,
}

impl PlayerBundle {
    pub fn new(position: &Vec2) -> Self {
        let mut skill_slot_map = SkillSlotMap::default();
        skill_slot_map.insert(PlayerActions::SkillSlot1, SkillName::BowAttack);
        skill_slot_map.insert(PlayerActions::SkillSlot2, SkillName::SplitArrow);
        skill_slot_map.insert(PlayerActions::SkillSlot3, SkillName::FlowerArrow);

        Self {
            character: CharacterBundle::new(CharacterId::Player, position),
            mana: Mana::new(PLAYER_BASE_MANA),
            skill_slot_map,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerLocalBundle {
    marker: Player,
    pub skills_available: SkillsAvailable,
    skill_speed: SkillSpeed,
}
impl PlayerLocalBundle {
    pub fn init() -> Self {
        Self {
            marker: Player,
            skills_available: SkillsAvailable::default(),
            skill_speed: SkillSpeed {
                value: Duration::from_millis(200),
            },
        }
    }
}

pub fn player_init_local(entity: Entity, commands: &mut Commands, skill_db: &SkillDb) {
    let mut player_local_bundle = PlayerLocalBundle::init();
    attach_all_skills(
        commands,
        entity,
        &mut player_local_bundle.skills_available,
        skill_db,
    );
    commands.entity(entity).insert_if_new(player_local_bundle);
}
