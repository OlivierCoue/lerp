use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicationTarget, PreSpawnedPlayerObject};
use serde::{Deserialize, Serialize};

use crate::{
    enemy::{Enemy, EnemyAliveBundle},
    health::Health,
};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Alive;

#[derive(Component)]
pub struct Dying {
    pub timer: Timer,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Dead;

pub fn set_character_life_state(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut targets: Query<
        (
            Entity,
            &Health,
            Option<&Alive>,
            Option<&mut Dying>,
            Option<&Dead>,
        ),
        (
            With<Enemy>,
            Without<Dead>,
            Or<(
                With<Predicted>,
                With<PreSpawnedPlayerObject>,
                With<ReplicationTarget>,
            )>,
        ),
    >,
) {
    for (entity, health, alive, dying, dead) in targets.iter_mut() {
        let should_be_dead = health.current <= 0.0;

        if should_be_dead {
            // Remove Alive state if it exists
            if alive.is_some() {
                commands
                    .entity(entity)
                    .remove_with_requires::<EnemyAliveBundle>();
            }

            // Ensure Dying is present if not already
            if dying.is_none() && dead.is_none() {
                commands.entity(entity).insert(Dying {
                    timer: Timer::from_seconds(1.5, TimerMode::Once),
                });
            }
        } else {
            // Restore Alive state if health is > 0
            if alive.is_none() {
                commands.entity(entity).insert(EnemyAliveBundle::init());
            }
        }

        // Tick dying timer if present
        if let Some(mut dying_state) = dying {
            dying_state.timer.tick(time.delta());
            if dying_state.timer.finished() {
                commands.entity(entity).remove::<Dying>().insert(Dead);
            }
        }
    }
}
