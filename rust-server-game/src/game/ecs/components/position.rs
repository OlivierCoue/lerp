use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

#[derive(Component)]
pub struct Position {
    pub current: Vector2,
    pub revision: u32,
    pub revision_checkpoint: u32,
}
impl Position {
    pub fn new(current: Vector2) -> Self {
        Self {
            current,
            revision: 1,
            revision_checkpoint: 0,
        }
    }
}
