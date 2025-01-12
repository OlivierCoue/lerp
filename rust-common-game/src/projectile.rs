use avian2d::prelude::*;
use bevy::prelude::*;
use client::{Predicted, Rollback};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    protocol::{EnemyDTO, PlayerDTO, REPLICATION_GROUP},
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

pub fn on_spawn_projectile_event(
    identity: NetworkIdentity,
    tick_manager: Res<TickManager>,
    rollback: Option<Res<Rollback>>,
    mut commands: Commands,
    mut spawn_projectile_events: EventReader<SpawnProjectileEvent>,
) {
    for event in spawn_projectile_events.read() {
        let velocity = event.direction * PROJECTILE_BASE_MOVEMENT_SPEED;

        let projectile_entity = commands
            .spawn((
                Position::from_xy(event.from_position.x, event.from_position.y),
                Projectile,
                ProjectileData {
                    max_distance: 10. * PIXEL_METER,
                    distance_traveled: 0.,
                },
                RigidBody::Kinematic,
                Collider::circle(PROJECTILE_SIZE / 2.),
                LockedAxes::ROTATION_LOCKED,
                PreviousPosition(event.from_position),
                LinearVelocity(velocity),
                PreSpawnedPlayerObject::default_with_salt(
                    event.client_id.map_or(0, |c| c.to_bits()) as u64,
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

pub fn update_and_despawn_projectile(
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
                commands
                    .entity(entity)
                    .remove::<server::Replicate>()
                    .despawn();
            } else {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            previous_position.0 = current_position.0;
        }
    }
}

pub fn process_projectile_collisions(
    mut collision_event_reader: EventReader<Collision>,
    enemy_q: Query<&EnemyDTO>,
    projectile_q: Query<&Projectile>,
    wall_q: Query<&Wall>,
    mut commands: Commands,
    identity: NetworkIdentity,
) {
    // when A and B collide, it can be reported as one of:
    // * A collides with B
    // * B collides with A
    // which is why logic is duplicated twice here
    for Collision(contacts) in collision_event_reader.read() {
        if let Ok(_) = projectile_q.get(contacts.entity1) {
            let collide_with_wall = wall_q.get(contacts.entity2).is_ok();
            let collide_with_enemy = enemy_q.get(contacts.entity2).is_ok();

            // despawn the projectile if it hit a wall
            if collide_with_wall || collide_with_enemy {
                if identity.is_server() {
                    commands
                        .entity(contacts.entity1)
                        .remove::<server::Replicate>()
                        .despawn();
                } else {
                    commands.entity(contacts.entity1).despawn_recursive();
                }
            }

            // despawn the enemy if it get hit
            if collide_with_enemy {
                if identity.is_server() {
                    commands
                        .entity(contacts.entity2)
                        .remove::<server::Replicate>()
                        .despawn();
                } else {
                    commands.entity(contacts.entity2).despawn_recursive();
                }
            }
        }
        if let Ok(_) = projectile_q.get(contacts.entity2) {
            let collide_with_wall = wall_q.get(contacts.entity1).is_ok();
            let collide_with_enemy = enemy_q.get(contacts.entity1).is_ok();

            // despawn the projectile if it hit a wall
            if collide_with_wall || collide_with_enemy {
                if identity.is_server() {
                    commands
                        .entity(contacts.entity2)
                        .remove::<server::Replicate>()
                        .despawn();
                } else {
                    commands.entity(contacts.entity2).despawn_recursive();
                }
            }

            // despawn the enemy if it get hit
            if collide_with_enemy {
                if identity.is_server() {
                    commands
                        .entity(contacts.entity1)
                        .remove::<server::Replicate>()
                        .despawn();
                } else {
                    commands.entity(contacts.entity1).despawn_recursive();
                }
            }
        }
    }
}
