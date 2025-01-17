use bevy::prelude::*;
use lightyear::prelude::{
    client::{Predicted, PredictionDespawnCommandsExt},
    server::ReplicationTarget,
    NetworkIdentity,
};

use crate::health::Health;

#[derive(Event)]
pub struct HitEvent {
    pub hit_source: Entity,
    pub hit_target: Entity,
}

pub fn on_hit_event(
    identity: NetworkIdentity,
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    mut hit_target_q: Query<&mut Health, (Or<(With<Predicted>, With<ReplicationTarget>)>,)>,
) {
    for event in hit_events.read() {
        if let Ok(mut health) = hit_target_q.get_mut(event.hit_target) {
            health.current = (health.current - 1.).min(health.max).max(0.);
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
