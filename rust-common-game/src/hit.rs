use bevy::prelude::*;

#[derive(Event)]
pub struct HitEvent {
    pub hit_source: Entity,
    pub hit_target: Entity,
}
