use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct EnemiesState {
    is_enable: bool,
    pub spwan_every_millis: u32,
    pub last_spawn_at_millis: u32,
}
impl EnemiesState {
    pub fn new() -> Self {
        Self {
            is_enable: false,
            spwan_every_millis: 300,
            last_spawn_at_millis: 0,
        }
    }

    pub fn is_enable(&self) -> bool {
        self.is_enable
    }

    pub fn toggle_enable(&mut self) {
        self.is_enable = !self.is_enable
    }
}
