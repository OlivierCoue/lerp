use core::f32;

use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicationTarget};
use serde::{Deserialize, Serialize};

use crate::{
    character_controller::CharacterController,
    health::Health,
    hit::{HitTracker, Hittable},
    physics::PhysicsBundle,
    protocol::*,
    shared::{ENEMY_BASE_HEALTH, ENEMY_BASE_MOVEMENT_SPEED, ENEMY_SIZE, PIXEL_METER},
};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Enemy,
    physics: PhysicsBundle,
    position: Position,
    character_controller: CharacterController,
    movement_speed: MovementSpeed,
    health: Health,
    hittable: Hittable,
    hit_tracker: HitTracker,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            marker: Enemy,
            physics: Self::physics(),
            position: Position::default(),
            character_controller: CharacterController,
            movement_speed: MovementSpeed(ENEMY_BASE_MOVEMENT_SPEED),
            health: Health::new(ENEMY_BASE_HEALTH),
            hittable: Hittable,
            hit_tracker: HitTracker::default(),
        }
    }
}
impl EnemyBundle {
    pub fn new(position: &Vec2) -> Self {
        Self {
            position: Position(*position),
            ..default()
        }
    }
    pub fn from_protocol() -> Self {
        Self { ..default() }
    }
    pub fn physics() -> PhysicsBundle {
        PhysicsBundle {
            rigid_body: RigidBody::Kinematic,
            collider: Collider::circle(ENEMY_SIZE / 2.),
        }
    }
}

pub fn enemy_movement_behavior(
    mut query_enemies: Query<
        (&Position, &mut LinearVelocity, &MovementSpeed),
        (With<Enemy>, Or<(With<Predicted>, With<ReplicationTarget>)>),
    >,
    query_players: Query<
        &mut Position,
        (
            With<Player>,
            Without<Enemy>,
            Or<(With<Predicted>, With<ReplicationTarget>)>,
        ),
    >,
) {
    // Collect and sort players deterministically
    let mut player_positions: Vec<_> = query_players.iter().collect();
    player_positions.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });

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

    // Process each enemy
    for (enemy_position, mut enemy_velocity, movement_speed) in enemies {
        // Retrieve the position of the nearest player
        let mut nearest_player_position = None;
        let mut nearest_player_distance = f32::MAX;
        for player_position in &player_positions {
            let distance_to_player = enemy_position.distance(player_position.0);
            if distance_to_player < nearest_player_distance {
                nearest_player_distance = distance_to_player;
                nearest_player_position = Some(player_position)
            }
        }

        let Some(nearest_player_position) = nearest_player_position else {
            continue;
        };

        nearest_player_distance = ((nearest_player_distance * 1000.0).round() as i32 / 1000) as f32;

        if nearest_player_distance > 25. {
            let target_pos = nearest_player_position.0;

            // Seek behavior
            let desired_velocity = (target_pos - enemy_position.0).normalize() * movement_speed.0;

            // Arrive behavior
            let distance = (target_pos - enemy_position.0).length();
            let slowing_radius = PIXEL_METER;
            let adjusted_speed = if distance < slowing_radius {
                movement_speed.0 * (distance / slowing_radius)
            } else {
                movement_speed.0
            };
            let arrived_velocity = desired_velocity.clamp_length(0.0, adjusted_speed);

            // Separation behavior
            let mut separation_force = Vec2::ZERO;
            let separation_distance = 1.0 * PIXEL_METER;
            for other_position in &enemies_position {
                if other_position != &enemy_position {
                    let diff = enemy_position.0 - other_position.0;
                    let dist_sq = diff.length_squared();
                    if dist_sq < separation_distance.powi(2) && dist_sq > 0.0 {
                        let strength = PIXEL_METER;
                        separation_force += (diff / dist_sq.sqrt()) * strength;
                    }
                }
            }

            // Combine behaviors
            let steering = (arrived_velocity - enemy_velocity.0) + separation_force;
            enemy_velocity.0 += steering.clamp_length_max(movement_speed.0);
        } else {
            enemy_velocity.0 = Vec2::ZERO;
        }
    }
}
