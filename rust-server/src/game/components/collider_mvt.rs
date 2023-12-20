use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

#[derive(Component)]
pub struct ColliderMvt {
    pub rect: Vector2,
}
impl ColliderMvt {
    pub fn new(rect: Vector2) -> Self {
        Self { rect }
    }
}
