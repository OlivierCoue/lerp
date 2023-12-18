use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rand::Rng;

use crate::{
    game::{bundles::prelude::*, resources::prelude::*},
    utils::get_game_time,
};

use super::prelude::{GRID_SIZE_X_MAX, GRID_SIZE_X_MIN, GRID_SIZE_Y_MAX, GRID_SIZE_Y_MIN};

pub fn enemies_spawner(mut enemies_state: ResMut<EnemiesState>, mut command: Commands) {
    let current_game_time = get_game_time();
    if enemies_state.last_spawn_at_millis + enemies_state.spwan_every_millis < current_game_time {
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
