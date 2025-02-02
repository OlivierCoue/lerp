use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicationTarget, PreSpawnedPlayerObject};
use serde::{Deserialize, Serialize};

use crate::health::Health;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Dead;

pub fn set_dead(
    mut commands: Commands,
    targets: Query<
        (Entity, &Health),
        (
            Without<Dead>,
            Or<(
                With<Predicted>,
                With<PreSpawnedPlayerObject>,
                With<ReplicationTarget>,
            )>,
        ),
    >,
) {
    for (entity, health) in targets.iter() {
        if health.current > 0. {
            continue;
        }

        commands.entity(entity).insert(Dead);
    }
}
