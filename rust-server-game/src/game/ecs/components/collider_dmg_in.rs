use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

#[derive(Component)]
pub struct ColliderDmgIn {
    pub rect: Vector2,
}
impl ColliderDmgIn {
    pub fn new(rect: Vector2) -> Self {
        Self { rect }
    }
}
