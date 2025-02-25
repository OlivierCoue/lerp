use avian2d::prelude::*;
use bevy::prelude::*;

use crate::prelude::*;

#[derive(Component, Default)]
pub struct Wall;

#[derive(Bundle, Default)]
pub struct WallBundle {
    pub wall: Wall,
    pub not_networked: NotNetworked,
    pub hittable: Hittable,
    pub position: Position,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}
impl WallBundle {
    pub fn new(position: Position, collider: Collider) -> Self {
        Self {
            position,
            collider,
            rigid_body: RigidBody::Static,
            ..default()
        }
    }
}
