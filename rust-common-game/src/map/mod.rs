use bevy::prelude::*;
use input::*;
use loader::load_map;
use map::Map;

pub mod input;
pub mod loader;
#[allow(clippy::module_inception)]
pub mod map;
pub mod tile_kind;

pub fn generate_map(mut commands: Commands, mut map_grid: ResMut<Map>) {
    load_map(&mut commands, &mut map_grid, create_large_map_input());
}
