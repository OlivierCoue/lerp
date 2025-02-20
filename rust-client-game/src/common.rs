use bevy::prelude::*;
use rust_common_game::utils::cartesian_to_isometric;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Setup,
    Auth,
    Lobby,
    Play,
}

pub fn cartesian_to_isometric_radius(r: f32) -> Vec2 {
    let sqrt_2 = 2.0f32.sqrt();
    Vec2::new(r * sqrt_2, r * (sqrt_2 / 2.0))
}

pub fn cartesian_to_isometric_vec2(v: &Vec2) -> Vec2 {
    cartesian_to_isometric(v.x, v.y)
}

#[derive(Resource)]
pub struct DebugConfig {
    pub show_colliders: bool,
    pub show_confirmed_entities: bool,
    pub show_flow_field: bool,
    pub show_y_sort_boundaries: bool,
}

pub const Z_FLOOR: f32 = 0.;
pub const Z_OBJECT_ON_FLOOR: f32 = 10.;
pub const Z_DEFAULT: f32 = 20.;
pub const Z_ITEM_DROPPED_NAME_PLATE: f32 = 30.;
pub const Z_DEBUG: f32 = 40.;
