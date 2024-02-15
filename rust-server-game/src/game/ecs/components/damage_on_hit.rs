use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct DamageOnHit {
    pub despawn_after_first_apply: bool,
    pub damage_value: u32,
    pub hitted_entities: HashMap<Entity, bool>,
}
