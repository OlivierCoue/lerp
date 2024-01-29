use bevy_ecs::prelude::*;

use crate::game::GAME_TIME_TICK_DURATION_MILLIS;

#[derive(Resource)]
pub struct Time {
    pub delta: f32,
    pub current_millis: u32,
}
impl Time {
    pub fn new() -> Self {
        Self {
            delta: 0.0,
            current_millis: 0,
        }
    }

    pub fn inc_current_millis(&mut self) {
        self.current_millis += GAME_TIME_TICK_DURATION_MILLIS;
    }
}
