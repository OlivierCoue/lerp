use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::{
    client::Predicted, server::ReplicationTarget, NetworkIdentity, PreSpawnedPlayerObject,
};
use serde::{Deserialize, Serialize};

use crate::shared::PROJECTILE_SIZE;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Projectile;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectileData {
    pub max_distance: f32,
    pub distance_traveled: f32,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PreviousPosition(pub Vec2);

#[derive(Bundle)]
pub struct ProjectileDTOBundle {
    pub marker: Projectile,
    pub position: Position,
    pub linear_velocity: LinearVelocity,
}

impl ProjectileDTOBundle {
    pub fn build(position: Position, linear_velocity: LinearVelocity) -> Self {
        Self {
            marker: Projectile,
            position,
            linear_velocity,
        }
    }
}

#[derive(Bundle)]
pub struct ProjectilePhysicsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
}

impl ProjectilePhysicsBundle {
    pub fn build() -> Self {
        Self {
            rigid_body: RigidBody::Kinematic,
            collider: Collider::circle(PROJECTILE_SIZE / 2.),
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn move_projectiles(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut PreviousPosition,
            &mut Position,
            &mut ProjectileData,
        ),
        (
            With<Projectile>,
            Or<(
                // move predicted bullets
                With<Predicted>,
                // move server entities
                With<ReplicationTarget>,
                // move prespawned bullets
                With<PreSpawnedPlayerObject>,
            )>,
        ),
    >,
) {
    for (entity, mut previous_position, current_position, mut projectile_data) in query.iter_mut() {
        let distance_traveled = previous_position.0.distance(current_position.0);
        projectile_data.distance_traveled += distance_traveled;

        if projectile_data.distance_traveled >= projectile_data.max_distance {
            commands.entity(entity).despawn_recursive();
        } else {
            previous_position.0 = current_position.0;
        }
    }
}
