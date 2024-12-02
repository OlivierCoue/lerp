use bevy_ecs::prelude::*;
use rust_common::math::Vec2;

#[derive(Component)]
pub struct ColliderDmgIn {
    pub rect: Vec2,
}
impl ColliderDmgIn {
    pub fn new(rect: Vec2) -> Self {
        Self { rect }
    }
}
