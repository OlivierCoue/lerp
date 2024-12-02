use bevy_ecs::prelude::*;
use rust_common::math::Vec2;

#[derive(Component)]
pub struct Position {
    pub current: Vec2,
    pub revision: u32,
    pub revision_checkpoint: u32,
}
impl Position {
    pub fn new(current: Vec2) -> Self {
        Self {
            current,
            revision: 1,
            revision_checkpoint: 0,
        }
    }
}
