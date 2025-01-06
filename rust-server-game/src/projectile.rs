use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::projectile::*;

#[derive(Bundle)]
pub struct ProjectileServerBundle {
    pub dto: ProjectileDTOBundle,
    pub physics: ProjectilePhysicsBundle,
}
impl ProjectileServerBundle {
    pub fn build(position: Position, linear_velocity: LinearVelocity) -> Self {
        Self {
            dto: ProjectileDTOBundle::build(position, linear_velocity),
            physics: ProjectilePhysicsBundle::build(),
        }
    }
}
