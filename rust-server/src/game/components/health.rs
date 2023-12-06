use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: u32,
    pub min: u32,
    pub current: u32,
}
