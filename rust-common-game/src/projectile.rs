use avian2d::prelude::*;
use bevy::prelude::*;
use client::{Predicted, PredictionDespawnCommandsExt};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    protocol::{Enemy, REPLICATION_GROUP},
    shared::{PIXEL_METER, PROJECTILE_BASE_MOVEMENT_SPEED, PROJECTILE_SIZE},
    wall::Wall,
};

#[derive(Component, Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Projectile;

#[derive(Event)]
pub struct SpawnProjectileEvent {
    pub client_id: Option<ClientId>,
    pub from_position: Vec2,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct ProjectileData {
    pub max_distance: f32,
    pub distance_traveled: f32,
}

#[derive(Component)]
pub struct PreviousPosition(pub Vec2);

#[derive(Component)]
pub struct EntityPhysics;

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub marker: EntityPhysics,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    marker: Projectile,
    data: ProjectileData,
    physics: PhysicsBundle,
    position: Position,
    previous_position: PreviousPosition,
    linear_velocity: LinearVelocity,
}
impl ProjectileBundle {
    pub fn new(position: &Vec2, linear_velocity: &Vec2) -> Self {
        Self {
            marker: Projectile,
            data: ProjectileData {
                max_distance: 10. * PIXEL_METER,
                distance_traveled: 0.,
            },
            physics: Self::physics(),
            position: Position::from_xy(position.x, position.y),
            previous_position: PreviousPosition(*position),
            linear_velocity: LinearVelocity(*linear_velocity),
        }
    }
    pub fn physics() -> PhysicsBundle {
        PhysicsBundle {
            marker: EntityPhysics,
            rigid_body: RigidBody::Kinematic,
            collider: Collider::circle(PROJECTILE_SIZE / 2.),
        }
    }
}

pub fn on_spawn_projectile_event(
    identity: NetworkIdentity,
    mut commands: Commands,
    mut spawn_projectile_events: EventReader<SpawnProjectileEvent>,
) {
    for event in spawn_projectile_events.read() {
        let linear_velocity = event.direction * PROJECTILE_BASE_MOVEMENT_SPEED;

        let projectile_entity = commands
            .spawn((
                ProjectileBundle::new(&event.from_position, &linear_velocity),
                PreSpawnedPlayerObject::default_with_salt(
                    event.client_id.map_or(0, |c| c.to_bits()),
                ),
            ))
            .id();

        if identity.is_server() {
            commands.entity(projectile_entity).insert((Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::All,
                    interpolation: NetworkTarget::None,
                },
                target: ReplicationTarget {
                    target: NetworkTarget::All,
                },
                controlled_by: ControlledBy {
                    target: NetworkTarget::None,
                    ..default()
                },
                group: REPLICATION_GROUP,
                ..default()
            },));
        }
    }
}

pub fn process_projectile_distance(
    identity: NetworkIdentity,
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
                With<Predicted>,
                With<PreSpawnedPlayerObject>,
                With<ReplicationTarget>,
            )>,
        ),
    >,
) {
    for (entity, mut previous_position, current_position, mut projectile_data) in query.iter_mut() {
        let distance_traveled = previous_position.0.distance(current_position.0);
        projectile_data.distance_traveled += distance_traveled;

        if projectile_data.distance_traveled >= projectile_data.max_distance {
            if identity.is_server() {
                commands.entity(entity).despawn();
            } else {
                commands.entity(entity).prediction_despawn();
            }
        } else {
            previous_position.0 = current_position.0;
        }
    }
}

pub fn process_projectile_collisions(
    mut collision_event_reader: EventReader<Collision>,
    enemy_q: Query<&Enemy>,
    projectile_q: Query<(&Projectile, &ProjectileData)>,
    wall_q: Query<&Wall>,
    mut commands: Commands,
    identity: NetworkIdentity,
) {
    // when A and B collide, it can be reported as one of:
    // * A collides with B
    // * B collides with A
    // which is why logic is duplicated twice here
    for Collision(contacts) in collision_event_reader.read() {
        if projectile_q.get(contacts.entity1).is_ok() {
            let collide_with_wall = wall_q.get(contacts.entity2).is_ok();
            let collide_with_enemy = enemy_q.get(contacts.entity2).is_ok();

            // despawn the projectile if it hit a wall
            if collide_with_wall || collide_with_enemy {
                if identity.is_server() {
                    commands.entity(contacts.entity1).despawn();
                } else {
                    commands.entity(contacts.entity1).prediction_despawn();
                }
            }

            // despawn the enemy if it get hit
            if collide_with_enemy {
                if identity.is_server() {
                    commands.entity(contacts.entity2).despawn();
                } else {
                    commands.entity(contacts.entity2).prediction_despawn();
                }
            }
        }

        if projectile_q.get(contacts.entity2).is_ok() {
            let collide_with_wall = wall_q.get(contacts.entity1).is_ok();
            let collide_with_enemy = enemy_q.get(contacts.entity1).is_ok();

            // despawn the projectile if it hit a wall
            if collide_with_wall || collide_with_enemy {
                if identity.is_server() {
                    commands.entity(contacts.entity2).despawn();
                } else {
                    commands.entity(contacts.entity2).prediction_despawn();
                }
            }

            // despawn the enemy if it get hit
            if collide_with_enemy {
                if identity.is_server() {
                    commands.entity(contacts.entity1).despawn();
                } else {
                    commands.entity(contacts.entity1).prediction_despawn();
                }
            }
        }
    }
}
