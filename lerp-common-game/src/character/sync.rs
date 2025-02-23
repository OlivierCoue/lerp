use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicationTarget, PreSpawnedPlayerObject};

use crate::prelude::*;

pub fn set_character_local(
    skill_db: Res<SkillDb>,
    mut commands: Commands,
    mut character_q: Query<
        (Entity, &Character),
        (
            With<Character>,
            Without<CharacterLocal>,
            Or<(
                With<Predicted>,
                With<PreSpawnedPlayerObject>,
                With<ReplicationTarget>,
            )>,
        ),
    >,
) {
    for (entity, character) in character_q.iter_mut() {
        commands
            .entity(entity)
            .insert(CharacterLocalBundle::new(character.id.data().team));

        match character.id {
            CharacterId::Player => player_init_local(entity, &mut commands, &skill_db),
            CharacterId::Enemy => enemy_init_local(entity, &mut commands),
        }
    }
}
