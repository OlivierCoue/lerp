use bevy_ecs::prelude::*;

#[derive(Resource, Clone)]
pub struct AreaConfig {
    pub area_width: f32,
    pub area_height: f32,
    pub walkable_x: Vec<u32>,
    pub walkable_y: Vec<u32>,
}
