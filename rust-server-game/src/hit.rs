use bevy::prelude::*;
use rust_common_game::{health::Health, hit::HitEvent};

pub fn on_hit_event(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    mut hit_target_q: Query<&mut Health>,
) {
    for event in hit_events.read() {
        if let Ok(mut health) = hit_target_q.get_mut(event.hit_target) {
            health.current = (health.current - 1.).max(0.);
            if health.current == 0. {
                commands.entity(event.hit_target).despawn();
            }
        }
    }
}
