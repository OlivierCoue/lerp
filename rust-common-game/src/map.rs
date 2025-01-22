use bevy::{
    math::{UVec2, Vec2},
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};

use crate::shared::PIXEL_METER;

/// Size of map (counts for tiles)
pub const MAP_SIZE: UVec2 = UVec2::new(100, 100);

/// Position a node/tile in the map grid
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash)]
pub struct MapNodePos(pub UVec2);
pub struct MapNode {
    pub walkable: bool,
}

#[derive(Resource, Default)]
pub struct MapGrid {
    pub map: HashMap<MapNodePos, MapNode>,
    pub size: UVec2,
}

pub fn world_position_to_map_node_pos(position: &Vec2) -> MapNodePos {
    let half_width = (MAP_SIZE.x as f32 * PIXEL_METER) / 2.;
    let half_height = (MAP_SIZE.y as f32 * PIXEL_METER) / 2.;

    // Add half of the map width/height to the position to get its absolute (positive) position
    // Example:
    // Map of 3000/3000 have an half width/height of 1500 and go from -1500 to 1500
    // Given a position of 0/0 position_abs will be 0 + 1500 / 0 + 1500 so 1500/1500
    let position_abs = position + Vec2::new(half_width, half_height);

    // Divide position_abs by the size of a tile to get the tile coordonates
    MapNodePos(UVec2::new(
        (position_abs.x / PIXEL_METER) as u32,
        (position_abs.y / PIXEL_METER) as u32,
    ))
}

pub fn get_obstacles() -> HashSet<UVec2> {
    let mut set = HashSet::new();
    set.insert(UVec2::new(60, 60));
    set.insert(UVec2::new(60, 61));
    set.insert(UVec2::new(60, 62));
    set.insert(UVec2::new(60, 63));
    set.insert(UVec2::new(60, 64));
    set.insert(UVec2::new(60, 65));

    set.insert(UVec2::new(61, 60));
    set.insert(UVec2::new(61, 61));
    set.insert(UVec2::new(61, 62));
    set.insert(UVec2::new(61, 63));
    set.insert(UVec2::new(61, 64));
    set.insert(UVec2::new(61, 65));

    set.insert(UVec2::new(62, 60));
    set.insert(UVec2::new(62, 61));
    set.insert(UVec2::new(62, 62));

    set.insert(UVec2::new(63, 60));
    set.insert(UVec2::new(64, 60));
    set.insert(UVec2::new(65, 60));
    set.insert(UVec2::new(66, 60));
    set.insert(UVec2::new(67, 60));
    set.insert(UVec2::new(68, 60));

    set.insert(UVec2::new(70, 60));
    set.insert(UVec2::new(70, 61));
    set.insert(UVec2::new(70, 62));
    set.insert(UVec2::new(70, 63));
    set.insert(UVec2::new(70, 64));
    set.insert(UVec2::new(70, 65));

    set.insert(UVec2::new(71, 60));
    set.insert(UVec2::new(71, 61));
    set.insert(UVec2::new(71, 62));
    set.insert(UVec2::new(71, 63));
    set.insert(UVec2::new(71, 64));
    set.insert(UVec2::new(71, 65));

    set.insert(UVec2::new(72, 65));
    set.insert(UVec2::new(73, 65));

    set
}
