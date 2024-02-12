use crate::game::area_gen::Shape;
use bevy_ecs::prelude::*;
use rust_common::proto::TileType;

#[derive(Resource)]
pub struct AreaConfig {
    pub area_width: f32,
    pub area_height: f32,
    pub walkable_x: Vec<u32>,
    pub walkable_y: Vec<u32>,
    pub oob_polygons: Vec<Shape>,
    pub oob_tile_type: TileType,
}
