use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicationTarget, PreSpawnedPlayerObject};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBaseBundle {
    marker: Enemy,
    position: Position,
    health: Health,
    team: Team,
}
impl Default for EnemyBaseBundle {
    fn default() -> Self {
        Self {
            marker: Enemy,
            position: Position::default(),
            health: Health::new(ENEMY_BASE_HEALTH),
            team: Team::Enemy,
        }
    }
}

impl EnemyBaseBundle {
    pub fn new(position: &Vec2) -> Self {
        Self {
            position: Position(*position),
            ..default()
        }
    }
}

#[derive(Bundle)]
pub struct EnemyAliveBundle {
    marker: Alive,
    physics: PhysicsBundle,
    hittable: Hittable,
    hit_tracker: HitTracker,
    character_controller: CharacterController,
    movement_speed: MovementSpeed,
}
impl EnemyAliveBundle {
    pub fn init() -> Self {
        Self {
            marker: Alive,
            physics: Self::physics(),
            character_controller: CharacterController,
            movement_speed: MovementSpeed(ENEMY_BASE_MOVEMENT_SPEED),
            hittable: Hittable,
            hit_tracker: HitTracker::default(),
        }
    }
    pub fn physics() -> PhysicsBundle {
        PhysicsBundle {
            rigid_body: RigidBody::Kinematic,
            collider: Collider::circle(ENEMY_SIZE / 2.),
        }
    }
}

pub fn enemy_movement_behavior(
    map_grid: Res<Map>,
    flow_field: Res<FlowField>,
    mut query_enemies: Query<
        (&Position, &mut LinearVelocity, &MovementSpeed),
        (
            With<Enemy>,
            With<Alive>,
            Or<(
                With<Predicted>,
                With<PreSpawnedPlayerObject>,
                With<ReplicationTarget>,
            )>,
        ),
    >,
) {
    // Collect and sort enemies deterministically
    let mut enemies: Vec<_> = query_enemies.iter_mut().collect();
    enemies.sort_by(|(pos_a, _, _), (pos_b, _, _)| {
        pos_a
            .x
            .partial_cmp(&pos_b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                pos_a
                    .y
                    .partial_cmp(&pos_b.y)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    // Store enemy positions for separation
    let enemies_position: Vec<_> = enemies.iter().map(|(pos, _, _)| *pos).collect();

    let mut i: i32 = 0;
    #[allow(clippy::explicit_counter_loop)]
    for (enemy_position, mut enemy_velocity, movement_speed) in enemies {
        // Retrieve the flow field direction
        let flow_direction = flow_field.get_direction_from_position(&map_grid, enemy_position);

        // Scale flow field force to movement speed
        let flow_field_force = flow_direction.map_or(Vec2::ZERO, |d| {
            d.to_normalized_velocity() * movement_speed.0
        });

        // Separation behavior
        let mut separation_force = Vec2::ZERO;
        let separation_distance = 1.0 * PIXEL_METER;
        for other_position in &enemies_position {
            if other_position != &enemy_position {
                let diff = enemy_position.0 - other_position.0;
                let dist_sq = diff.length_squared();
                if dist_sq < separation_distance.powi(2) && dist_sq > 0.0 {
                    let force = diff / dist_sq.sqrt();
                    if !force.is_nan() {
                        separation_force += force;
                    }
                }
            }
        }

        // Scale separation force to avoid overpowering flow field
        let separation_force_scale = if i % 3 == 0 { 0.5 } else { 0.25 };
        separation_force =
            separation_force.normalize_or_zero() * movement_speed.0 * separation_force_scale;

        // Combine forces
        let combined_force = flow_field_force + separation_force;

        // Update velocity
        enemy_velocity.0 = combined_force.clamp_length_max(movement_speed.0);
        i += 1;
    }
}
