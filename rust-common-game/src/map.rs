use avian2d::prelude::{Collider, Position, RigidBody};
use bevy::{
    math::{UVec2, Vec2},
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};

use crate::{shared::PIXEL_METER, utils::CommonPlaySceneTag, wall::Wall};

/// Size of map (counts for tiles)
pub const MAP_SIZE: UVec2 = UVec2::new(100, 100);

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MapNodeKind {
    Floor,
    Obtacle,
    Wall,
}
impl MapNodeKind {
    pub fn is_walkable(&self) -> bool {
        *self == Self::Floor
    }
}

/// Position a node/tile in the map grid
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash)]
pub struct MapNodePos(pub UVec2);
pub struct MapNode {
    pub kind: MapNodeKind,
    pub walkable: bool,
}

#[derive(Resource, Default)]
pub struct MapGrid {
    pub map: HashMap<MapNodePos, MapNode>,
    pub size: UVec2,
}
impl MapGrid {
    pub fn get_node_xy(&self, x: u32, y: u32) -> Option<&MapNode> {
        self.map.get(&MapNodePos(UVec2::new(x, y)))
    }
    pub fn get_node_uvec2(&self, uvec2: UVec2) -> Option<&MapNode> {
        self.map.get(&MapNodePos(uvec2))
    }
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

pub fn generate_map(mut commands: Commands, mut map_grid: ResMut<MapGrid>) {
    map_grid.map.clear();
    map_grid.size = MAP_SIZE;
    let obstacles = get_obstacles();

    let width_px = map_grid.size.x as f32 * PIXEL_METER;
    let height_px = map_grid.size.y as f32 * PIXEL_METER;

    let node_center_offest = PIXEL_METER / 2.0;

    for x in 0..map_grid.size.x {
        for y in 0..map_grid.size.y {
            let cart_coord = Vec2::new(
                x as f32 * PIXEL_METER - (width_px / 2.0) + node_center_offest,
                y as f32 * PIXEL_METER - (height_px / 2.0) + node_center_offest,
            );

            let kind = if (x == 0)
                || (x == (map_grid.size.x - 1))
                || (y == 0)
                || (y == map_grid.size.y - 1)
            {
                MapNodeKind::Wall
            } else if (x % 6 == 0 && y % 6 == 0) || obstacles.contains(&UVec2::new(x, y)) {
                MapNodeKind::Obtacle
            } else {
                MapNodeKind::Floor
            };

            map_grid.map.insert(
                MapNodePos(UVec2::new(x, y)),
                MapNode {
                    kind,
                    walkable: kind.is_walkable(),
                },
            );

            if !kind.is_walkable() {
                commands.spawn((
                    CommonPlaySceneTag,
                    Wall,
                    Position::from_xy(cart_coord.x, cart_coord.y),
                    RigidBody::Static,
                    Collider::rectangle(PIXEL_METER, PIXEL_METER),
                ));
            }
        }
    }

    // // TOP
    commands.spawn((
        CommonPlaySceneTag,
        Wall,
        Position::from_xy(0., (height_px / 2.0) - node_center_offest),
        RigidBody::Static,
        Collider::rectangle(width_px, PIXEL_METER),
    ));
    // // BOTOM
    commands.spawn((
        CommonPlaySceneTag,
        Wall,
        Position::from_xy(0., -(height_px / 2.0) + node_center_offest),
        RigidBody::Static,
        Collider::rectangle(width_px, PIXEL_METER),
    ));
    // // LEFT
    commands.spawn((
        CommonPlaySceneTag,
        Wall,
        Position::from_xy(-(width_px / 2.0) + node_center_offest, 0.),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, height_px),
    ));
    // // RIGHT
    commands.spawn((
        CommonPlaySceneTag,
        Wall,
        Position::from_xy(width_px / 2.0 - node_center_offest, 0.),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, height_px),
    ));
}
