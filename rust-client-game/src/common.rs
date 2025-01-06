use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Setup,
    Auth,
    Lobby,
    Play,
}

pub fn cartesian_to_isometric(cart_x: f32, cart_y: f32) -> Vec2 {
    Vec2::new(
        cart_x + cart_y,         // X-axis in isometric space
        (cart_y - cart_x) / 2.0, // Y-axis in isometric space
    )
}

pub fn isometric_to_cartesian(iso_x: f32, iso_y: f32) -> Vec2 {
    Vec2::new(
        (iso_x - 2.0 * iso_y) / 2.0, // Cartesian X
        (iso_x + 2.0 * iso_y) / 2.0, // Cartesian Y
    )
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

#[derive(Clone, Copy)]
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
}
