use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

#[derive(Component)]
pub struct Position {
    pub current: Vector2,
}
