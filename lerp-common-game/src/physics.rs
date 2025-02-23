use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Bundle, Clone)]
pub struct PhysicsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
}
