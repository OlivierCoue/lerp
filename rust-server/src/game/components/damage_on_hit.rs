use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct DamageOnHit {
    pub damage_value: u32,
    pub hitted_entities: HashMap<Entity, bool>,
    pub ignored_entity: Entity,
}
