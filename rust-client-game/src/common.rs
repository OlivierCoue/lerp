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

pub fn apply_render_mode(render_config: &RenderConfig, position: &Vec2) -> Vec2 {
    match render_config.mode {
        RenderMode::Iso => cartesian_to_isometric(position.x, position.y),
        RenderMode::Cart => *position,
    }
}

pub fn apply_render_mode_radius(render_config: &RenderConfig, r: f32) -> Vec2 {
    match render_config.mode {
        RenderMode::Iso => cartesian_to_isometric_radius(r),
        RenderMode::Cart => Vec2::splat(r),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Iso,
    Cart,
}

#[derive(Resource)]
pub struct RenderConfig {
    pub mode: RenderMode,
}

#[derive(Resource)]
pub struct DebugConfig {
    pub show_colliders: bool,
    pub show_confirmed_entities: bool,
    pub show_flow_field: bool,
}
