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
        cart_x - cart_y,         // X-axis in isometric space
        (cart_x + cart_y) / 2.0, // Y-axis in isometric space
    )
}

pub fn isometric_to_cartesian(iso_x: f32, iso_y: f32) -> Vec2 {
    Vec2::new(
        (2.0 * iso_y + iso_x) / 2.0, // Cartesian X
        (2.0 * iso_y - iso_x) / 2.0, // Cartesian Y
    )
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
