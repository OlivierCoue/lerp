use bevy_ecs::prelude::*;
use rust_common::collisions::ColliderShape;

#[derive(Component)]
pub struct ColliderMvt {
    pub shape: ColliderShape,
    pub reversed: bool,
}
impl ColliderMvt {
    pub fn new(shape: ColliderShape, reversed: bool) -> Self {
        Self { shape, reversed }
    }
}
