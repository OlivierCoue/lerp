use std::time::Duration;

use avian2d::prelude::*;
use bevy::{prelude::*, time::Timer};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use rust_common_game::{
    character_controller::CharacterController, protocol::*, settings::ENTITY_SIZE,
};

use crate::AutoMove;

const ENEMY_MAX_COUNT: u32 = 50;

#[derive(Resource)]
pub struct EnemyState {
    pub timer: Timer,
    pub count: u32,
}

fn spaw_enemy(mut commands: Commands, time: Res<Time>, mut enemy_state: ResMut<EnemyState>) {
    enemy_state.timer.tick(time.delta());

    if enemy_state.timer.finished() && enemy_state.count < ENEMY_MAX_COUNT {
        enemy_state.count += 1;
        let player = (
            Player {
                client_id: ClientId::Netcode(999999999),
            },
            Enemy,
            MovementTargets(Vec::new()),
            RigidBody::Kinematic,
            CharacterController,
            Collider::circle(ENTITY_SIZE / 2.0 / 2.),
            LockedAxes::ROTATION_LOCKED,
            MovementSpeed(150.),
            Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::None,
                    interpolation: NetworkTarget::All,
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
            },
        );
        commands.spawn(player);
    }
}

#[allow(clippy::type_complexity)]
fn enemy_movement_behavior(
    mut query_enemies: Query<(&Position, &mut LinearVelocity, &MovementSpeed), With<Enemy>>,
    query_players: Query<&mut Position, (With<Player>, Without<Enemy>, Without<AutoMove>)>,
) {
    let mut nearest_player_position_opt = None;
    for player_position in &query_players {
        nearest_player_position_opt = Some(&player_position.0);
    }

    let Some(nearest_player_position) = nearest_player_position_opt else {
        return;
    };

    // Store all enemies position for later use
    let enemies_position: Vec<Position> = query_enemies
        .iter()
        .map(|(position, _, _)| *position)
        .collect();

    // Loop over enemies in make them move toward the player
    for (enemy_position, mut enemy_velocity, movement_speed) in &mut query_enemies {
        let distance_to_player = enemy_position.distance(*nearest_player_position);
        if distance_to_player > 25. {
            let target_pos = nearest_player_position;

            // SEEK behavior
            let desired_velocity = (target_pos - enemy_position.0).normalize() * movement_speed.0;

            // ARRIVE behavior (adjust speed near the target)
            let distance = (target_pos - enemy_position.0).length();
            let slowing_radius = ENTITY_SIZE; // Adjust as needed
            let adjusted_speed = if distance < slowing_radius {
                movement_speed.0 * (distance / slowing_radius)
            } else {
                movement_speed.0
            };
            let arrived_velocity = desired_velocity.clamp_length(0.0, adjusted_speed);

            // SEPARATION behavior
            let mut separation_force = Vec2::ZERO;
            let separation_distance = 1. * ENTITY_SIZE; // Adjust based on scale (e.g., 4x object size)
            for other_position in enemies_position.iter() {
                if other_position != enemy_position {
                    let diff = enemy_position.0 - other_position.0;
                    let dist_sq = diff.length_squared();
                    if dist_sq < separation_distance.powi(2) && dist_sq > 0.0 {
                        // Separation force scaling
                        let strength = 20.0; // Experiment with this value
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

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyState {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            count: 0,
        });
        app.add_systems(FixedUpdate, (spaw_enemy, enemy_movement_behavior));
    }
}
