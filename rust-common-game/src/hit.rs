use bevy::prelude::*;
use lightyear::prelude::{
    client::{Predicted, PredictionDespawnCommandsExt},
    server::ReplicationTarget,
    NetworkIdentity,
};

use crate::{health::Health, skill::*};

#[derive(Event)]
pub struct HitEvent {
    pub hit_source: Entity,
    pub hit_skill_source: Entity,
    pub hit_target: Entity,
}

pub fn on_hit_event(
    identity: NetworkIdentity,
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    hit_skill_source_q: Query<&SkillDamageOnHit, With<Skill>>,
    mut hit_target_q: Query<
        &mut Health,
        (
            Without<Skill>,
            Or<(With<Predicted>, With<ReplicationTarget>)>,
        ),
    >,
) {
    for event in hit_events.read() {
        if let Ok(mut health) = hit_target_q.get_mut(event.hit_target) {
            if let Ok(skill_damage_on_hit) = hit_skill_source_q.get(event.hit_skill_source) {
                health.current = (health.current - skill_damage_on_hit.value)
                    .min(health.max)
                    .max(0.);

                if health.current == 0. {
                    if identity.is_server() {
                        commands.entity(event.hit_target).despawn();
                    } else {
                        commands.entity(event.hit_target).prediction_despawn();
                    }
                }
            }
        }
    }
}
