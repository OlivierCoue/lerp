use avian2d::prelude::*;
use bevy::prelude::*;
use client::{Predicted, PredictionDespawnCommandsExt};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    enemy::Enemy,
    hit::{HitEvent, HitSource},
    physics::PhysicsBundle,
    protocol::{Player, REPLICATION_GROUP},
    shared::{PIXEL_METER, PROJECTILE_BASE_MOVEMENT_SPEED, PROJECTILE_SIZE},
    skill::*,
    utils::xor_u64s,
    wall::Wall,
};

#[derive(Component, Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Projectile;

#[derive(Component)]
pub struct ProjectileData {
    pub skill_source: Entity,
    pub max_distance: f32,
    pub distance_traveled: f32,
}

#[derive(Component, Default)]
pub struct PreviousPosition(pub Vec2);

#[derive(Bundle)]
pub struct ProjectileBundle {
    marker: Projectile,
    data: ProjectileData,
    hit_source: HitSource,
    physics: PhysicsBundle,
    position: Position,
    previous_position: PreviousPosition,
    linear_velocity: LinearVelocity,
    skill_instance_hash: SkillInstanceHash,
}
impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            marker: Projectile,
            data: ProjectileData {
                skill_source: Entity::PLACEHOLDER,
                max_distance: 0.,
                distance_traveled: 0.,
            },
            hit_source: HitSource,
            physics: Self::physics(),
            position: Position::default(),
            previous_position: PreviousPosition::default(),
            linear_velocity: LinearVelocity::default(),
            skill_instance_hash: SkillInstanceHash::default(),
        }
    }
}
impl ProjectileBundle {
    pub fn new(
        position: &Vec2,
        linear_velocity: &Vec2,
        skill_source: Entity,
        skill_instance_hash: u64,
    ) -> Self {
        Self {
            data: ProjectileData {
                skill_source,
                max_distance: 10. * PIXEL_METER,
                distance_traveled: 0.,
            },
            physics: Self::physics(),
            position: Position::from_xy(position.x, position.y),
            previous_position: PreviousPosition(*position),
            linear_velocity: LinearVelocity(*linear_velocity),
            skill_instance_hash: SkillInstanceHash(skill_instance_hash),
            ..default()
        }
    }
    pub fn from_protocol() -> Self {
        Self { ..default() }
    }
    pub fn physics() -> PhysicsBundle {
        PhysicsBundle {
            rigid_body: RigidBody::Kinematic,
            collider: Collider::circle(PROJECTILE_SIZE / 2.),
        }
    }
}

pub fn on_execute_skill_projectile_event(
    identity: NetworkIdentity,
    mut commands: Commands,
    mut excecute_skill_ev: EventReader<ExcecuteSkillEvent>,
    skill_projectile_q: Query<(Entity, &SkillProjectile, Option<&SkillDamageOnHit>), With<Skill>>,
    initiator_q: Query<(&Position, Option<&Player>), Without<Skill>>,
) {
    for event in excecute_skill_ev.read() {
        // Try to retrieve the skill data from the query.
        // If it does not exist, then this skill is not a projectile and will be ignored
        let Ok((skill_entity, skill_projectile, skill_damage_on_hit)) =
            skill_projectile_q.get(event.skill)
        else {
            continue;
        };

        let Ok((initiator_position, initiator_player)) = initiator_q.get(event.initiator) else {
            println!("[on_execute_skill_projectile_event] Cannot find initiator entity");
            continue;
        };

        let directions = generate_fan_projectile_directions(
            initiator_position.0,
            event.target,
            skill_projectile.count.ceil() as u32,
            15.,
        );

        let mut projectile_nb = 0;
        for direction in directions {
            projectile_nb += 1;
            let linear_velocity = direction * PROJECTILE_BASE_MOVEMENT_SPEED;

            // Create base Projectile
            let projectile_entity = commands
                .spawn((
                    ProjectileBundle::new(
                        &initiator_position.0,
                        &linear_velocity,
                        skill_entity,
                        event.skill_instance_hash,
                    ),
                    PreSpawnedPlayerObject::new(xor_u64s(&[
                        event.skill_instance_hash,
                        projectile_nb,
                    ])),
                ))
                .id();

            // Add optional components based on skill data
            if let Some(skill_damage_on_hit) = skill_damage_on_hit {
                commands.entity(projectile_entity).insert((DamageOnHit {
                    value: skill_damage_on_hit.value,
                },));
            }

            if skill_projectile.pierce_count > 0 {
                commands.entity(projectile_entity).insert((Pierce {
                    count: skill_projectile.pierce_count,
                },));
            }

            // Setup replication if we are on the server
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
}

fn generate_fan_projectile_directions(
    from: Vec2,
    target: Vec2,
    count: u32,
    angle: f32,
) -> Vec<Vec2> {
    if count == 0 {
        return Vec::new();
    }

    // Calculate the straight direction (normalized)
    let direction = (target - from).normalize();

    // Convert the angle to radians
    let angle_rad = angle.to_radians();

    // Calculate the initial rotation angle (to center the fan)
    let total_angle = angle_rad * (count as f32 - 1.0);
    let start_angle = -total_angle / 2.0;

    // Helper to rotate a vector by a given angle
    let rotate = |v: Vec2, angle: f32| {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
    };

    // Generate directions
    (0..count)
        .map(|i| {
            let current_angle = start_angle + angle_rad * i as f32;
            rotate(direction, current_angle)
        })
        .collect()
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
    mut hit_events: EventWriter<HitEvent>,
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
        if !contacts.collision_started() {
            continue;
        }

        if let Ok((_, projectile_data)) = projectile_q.get(contacts.entity1) {
            let collide_with_wall = wall_q.get(contacts.entity2).is_ok();
            let collide_with_enemy = enemy_q.get(contacts.entity2).is_ok();

            // despawn the projectile if it hit a wall
            if collide_with_wall {
                if identity.is_server() {
                    commands.entity(contacts.entity1).despawn();
                } else {
                    commands.entity(contacts.entity1).prediction_despawn();
                }
            }

            if collide_with_enemy {
                hit_events.send(HitEvent {
                    source: contacts.entity1,
                    skill: projectile_data.skill_source,
                    target: contacts.entity2,
                });
            }
        }

        if let Ok((_, projectile_data)) = projectile_q.get(contacts.entity2) {
            let collide_with_wall = wall_q.get(contacts.entity1).is_ok();
            let collide_with_enemy = enemy_q.get(contacts.entity1).is_ok();

            // despawn the projectile if it hit a wall
            if collide_with_wall {
                if identity.is_server() {
                    commands.entity(contacts.entity2).despawn();
                } else {
                    commands.entity(contacts.entity2).prediction_despawn();
                }
            }

            // despawn the enemy if it get hit
            if collide_with_enemy {
                hit_events.send(HitEvent {
                    source: contacts.entity2,
                    skill: projectile_data.skill_source,
                    target: contacts.entity1,
                });
            }
        }
    }
}
