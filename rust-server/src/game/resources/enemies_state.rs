use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct EnemiesState {
    pub spwan_every_millis: u32,
    pub last_spawn_at_millis: u32,
}
impl EnemiesState {
    pub fn new() -> Self {
        Self {
            spwan_every_millis: 1000,
            last_spawn_at_millis: 0,
        }
    }
}
