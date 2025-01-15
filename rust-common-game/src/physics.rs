use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
}
