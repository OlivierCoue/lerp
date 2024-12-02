use bevy_ecs::prelude::*;
use rust_common::collisions::ColliderShape;

#[derive(Component)]
pub struct ColliderMvt {
    pub shape: ColliderShape,
}
impl ColliderMvt {
    pub fn new(shape: ColliderShape) -> Self {
        Self { shape }
    }
}
