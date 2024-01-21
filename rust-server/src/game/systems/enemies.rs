use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rand::Rng;

use crate::{
    game::{
        bundles::prelude::*, resources::prelude::*, Position, UpdateVelocityTargetWithPathFinder,
    },
    utils::get_game_time,
};

use super::prelude::{GRID_SIZE_X_MAX, GRID_SIZE_X_MIN, GRID_SIZE_Y_MAX, GRID_SIZE_Y_MIN};

pub fn enemies_spawner(mut enemies_state: ResMut<EnemiesState>, mut command: Commands) {
    if !enemies_state.is_enable() {
        return;
    }

    let current_game_time = get_game_time();
    if enemies_state.last_spawn_at_millis == 0
        || enemies_state.last_spawn_at_millis + enemies_state.spwan_every_millis < current_game_time
    {
        enemies_state.last_spawn_at_millis = current_game_time;
        let random = rand::thread_rng().gen_range(GRID_SIZE_X_MIN..GRID_SIZE_X_MAX - 1.0);
        let position_current = match rand::thread_rng().gen_range(0..4) {
            0 => Vector2::new(GRID_SIZE_X_MIN, random),
            1 => Vector2::new(GRID_SIZE_X_MAX, random),
            2 => Vector2::new(random, GRID_SIZE_Y_MIN),
            3 => Vector2::new(random, GRID_SIZE_Y_MAX),
            _ => panic!("Unexpected value"),
        };
        command.spawn(EnemyBundle::new(position_current));
    }
}

#[allow(clippy::type_complexity)]
pub fn enemies_ai(
    mut query_enemies: Query<(Entity, &mut Enemie, &Position), (With<Enemie>, Without<Player>)>,
    query_players: Query<&Position, (With<Player>, Without<Enemie>)>,
    mut writer_update_velocity_target_with_pathfinder: EventWriter<
        UpdateVelocityTargetWithPathFinder,
    >,
) {
    let aggro_range = 500.0;
    for (enemy_entity, mut enemy, enemy_position) in &mut query_enemies {
        let current_game_time = get_game_time();
        if enemy.last_action_at_millis != 0
            && enemy.last_action_at_millis + 1000 > current_game_time
        {
            continue;
        }

        enemy.last_action_at_millis = current_game_time;
        let mut opt_closest_player_location = None;
        let mut closest_player_distance = 0.0;

        for player_position in &query_players {
            let player_distance = enemy_position.current.distance_to(player_position.current);
            if player_distance <= aggro_range
                && (opt_closest_player_location.is_none()
                    || player_distance < closest_player_distance)
            {
                opt_closest_player_location = Some(player_position.current);
                closest_player_distance = player_distance;
            }
        }

        if let Some(closest_player_location) = opt_closest_player_location {
            writer_update_velocity_target_with_pathfinder.send(UpdateVelocityTargetWithPathFinder {
                entity: enemy_entity,
                target: closest_player_location,
            })
        }
    }
}
