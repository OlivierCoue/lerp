use bevy::{prelude::*, utils::hashbrown::HashSet};
use lightyear::prelude::{
    client::{Predicted, PredictionDespawnCommandsExt},
    server::ReplicationTarget,
    NetworkIdentity,
};

use crate::prelude::*;

#[derive(Component, Default)]
pub struct HitSource(pub Team);

#[derive(Component, Default)]
pub struct Hittable {
    /// SkillInstanceHash
    pub hit_track_map: HashSet<u64>,
}

pub struct HitEventData {
    pub source: Entity,
    pub skill: Entity,
    pub target: Entity,
}

#[derive(Event)]
pub struct HitEvent(pub Vec<HitEventData>);

pub fn on_hit_event(
    identity: NetworkIdentity,
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    mut source_q: Query<
        (
            &HitSource,
            &SkillInstanceHash,
            Option<&DamageOnHit>,
            Option<&mut Pierce>,
        ),
        (With<HitSource>, Without<Skill>, Without<Hittable>),
    >,
    _skill_q: Query<&SkillDamageOnHit, (With<Skill>, Without<HitSource>, Without<Hittable>)>,
    mut target: Query<
        (&mut Hittable, Option<&Team>, Option<&mut Health>, Has<Wall>),
        (
            With<Hittable>,
            Without<HitSource>,
            Without<Skill>,
            Or<(With<Predicted>, With<ReplicationTarget>, With<NotNetworked>)>,
        ),
    >,
) {
    // We use this hash set to store entities despawn in the current system run and prevent dup despawn
    let mut despawned_entities = HashSet::new();

    for event in hit_events.read() {
        for event_data in &event.0 {
            let Ok((hit_source, skill_instance_hash, damage_on_hit, pierce)) =
                source_q.get_mut(event_data.source)
            else {
                if !despawned_entities.contains(&event_data.source) {
                    error!("[on_hit_event] Hit source does not exist in world");
                }
                continue;
            };

            let Ok((mut target_hittable, target_team, target_health, target_is_wall)) =
                target.get_mut(event_data.target)
            else {
                if !despawned_entities.contains(&event_data.target) {
                    error!("[on_hit_event] Hit target does not exist in world");
                }
                continue;
            };

            // Despawn the source if it hit a wall
            // TODO: this this true because we only have projectile which can hit, but will change in the future
            if target_is_wall {
                if despawned_entities.insert(event_data.source) {
                    if identity.is_server() {
                        commands.entity(event_data.source).despawn();
                    } else {
                        commands.entity(event_data.source).prediction_despawn();
                    }
                }
                continue;
            }

            // If the source hit something from its own team we just ignore it
            if target_team.is_none_or(|target_team| *target_team == hit_source.0) {
                continue;
            }

            // Insert the skill instance hash in the hit track map.
            // If it was already in, then we stop and do not apply any on hit effect.
            // This prevent shotguning.
            if !target_hittable.hit_track_map.insert(**skill_instance_hash) {
                continue;
            }

            // If the source apply DamageOnHit and the target has Health, then apply damages.
            if let (Some(damage_on_hit), Some(mut target_health)) = (damage_on_hit, target_health) {
                target_health.current = (target_health.current - damage_on_hit.value)
                    .min(target_health.max)
                    .max(0.);
            }

            // Try to apply pierce, decrement count and continue if pierced applied.
            if let Some(mut pierce) = pierce {
                if pierce.count >= 1 {
                    pierce.count -= 1;
                    continue;
                }
            }

            if despawned_entities.insert(event_data.source) {
                if identity.is_server() {
                    commands.entity(event_data.source).despawn();
                } else {
                    commands.entity(event_data.source).prediction_despawn();
                }
            }
        }
    }
}
