use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct Time {
    pub delta: f32,
}
impl Time {
    pub fn new() -> Self {
        Self { delta: 0.0 }
    }
}
