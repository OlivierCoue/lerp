use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

#[derive(Component)]
pub struct Shape {
    pub rect: Vector2,
}
