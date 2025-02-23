use avian2d::prelude::Position;
use bevy::{prelude::*, utils::HashMap};

use crate::{
    shared::{NAV_TILE_SIZE, RENDER_TILE_SIZE, RENDER_TO_NAV_TILE_MULTI},
    utils::cartesian_to_isometric,
};

use super::tile_kind::{RenderTileFloorKind, RenderTileWallKind};

/// Coordonates of a tile in the nav map
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash)]
pub struct NavTileCoord(pub UVec2);

/// Coordonates of a tile in the render map
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash)]
pub struct RenderTileCoord(pub UVec2);

pub struct NavTile {
    pub walkable: bool,
}

pub struct RenderTileFloor {
    pub kind: RenderTileFloorKind,
    pub none_walkable_nav_tiles: Vec<IVec2>,
}
impl RenderTileFloor {
    pub fn new(kind: RenderTileFloorKind) -> Self {
        Self {
            kind,
            none_walkable_nav_tiles: vec![],
        }
    }
}

pub struct RenderTileWall {
    pub kind: RenderTileWallKind,
    pub y_sort_boundaries: [Vec2; 3],
    pub none_walkable_nav_tiles: Vec<IVec2>,
}
impl RenderTileWall {
    pub fn new(
        map_size_px: &Vec2,
        render_tile_pos: &RenderTileCoord,
        kind: RenderTileWallKind,
    ) -> Self {
        let iso_offset = cartesian_to_isometric(
            (render_tile_pos.x as f32) * RENDER_TILE_SIZE - (map_size_px.x / 2.0),
            (render_tile_pos.y as f32) * RENDER_TILE_SIZE - (map_size_px.y / 2.0),
        );

        Self {
            kind,
            y_sort_boundaries: kind.y_sort_boundaries_with_offset(iso_offset),
            none_walkable_nav_tiles: kind.none_walkable_nav_tiles(),
        }
    }

    pub fn get_y_sort_boundary_at_x(&self, iso_x: f32) -> f32 {
        let [a_left, a_middle, a_right] = &self.y_sort_boundaries;

        // Determine which segment of the boundary to use based on x

        if iso_x < a_left.x {
            // Use the left boundary of A
            a_left.y
        } else if iso_x > a_right.x {
            // Use the right boundary of A
            a_right.y
        } else if iso_x <= a_middle.x {
            // Interpolate between left and middle
            a_left.y + (iso_x - a_left.x) / (a_middle.x - a_left.x) * (a_middle.y - a_left.y)
        } else {
            // Interpolate between middle and right
            a_middle.y + (iso_x - a_middle.x) / (a_right.x - a_middle.x) * (a_right.y - a_middle.y)
        }
    }
}

#[derive(Resource, Default)]
pub struct Map {
    pub nav_map: HashMap<NavTileCoord, NavTile>,
    pub nav_map_size: UVec2,

    pub render_map_wall: HashMap<RenderTileCoord, Vec<RenderTileWall>>,
    pub render_map_floor: HashMap<RenderTileCoord, RenderTileFloor>,
    pub render_map_size: UVec2,

    pub map_px_size: Vec2,
    pub map_px_half_size: Vec2,

    nav_tile_px_offset: Vec2,

    pub player_spawn_position: Vec2,
}
impl Map {
    pub fn reset(&mut self, render_map_size: UVec2) {
        if render_map_size.x % 10 != 0 {
            panic!(
                "Invalid map x size, it must be a multiple of 10 but got: {}",
                render_map_size.x
            );
        }
        if render_map_size.y % 10 != 0 {
            panic!(
                "Invalid map y size, it must be a multiple of 10 but got: {}",
                render_map_size.y
            );
        }
        self.render_map_wall.clear();
        self.render_map_size = render_map_size;

        self.nav_map.clear();
        self.nav_map_size = self.render_map_size * RENDER_TO_NAV_TILE_MULTI;

        self.map_px_size = Vec2::new(
            self.render_map_size.x as f32 * RENDER_TILE_SIZE,
            self.render_map_size.y as f32 * RENDER_TILE_SIZE,
        );
        self.map_px_half_size = self.map_px_size / 2.;

        self.nav_tile_px_offset = self.map_px_half_size - NAV_TILE_SIZE / 2.0;
    }

    pub fn add_tile_floor(&mut self, kind: RenderTileFloorKind, render_tile_pos: UVec2) {
        let render_map_tile_floor = RenderTileFloor::new(kind);
        self.render_map_floor.insert(
            RenderTileCoord(UVec2::new(render_tile_pos.x, render_tile_pos.y)),
            render_map_tile_floor,
        );
    }

    pub fn add_tile_wall(&mut self, kind: RenderTileWallKind, render_tile_pos: UVec2) {
        let render_map_tile = RenderTileWall::new(
            &self.map_px_size,
            &RenderTileCoord(UVec2::new(render_tile_pos.x, render_tile_pos.y)),
            kind,
        );

        for none_walkable_nav_tile in &render_map_tile.none_walkable_nav_tiles {
            self.nav_map.insert(
                NavTileCoord(UVec2::new(
                    ((render_tile_pos.x * RENDER_TO_NAV_TILE_MULTI) as i32
                        + none_walkable_nav_tile.x) as u32,
                    ((render_tile_pos.y * RENDER_TO_NAV_TILE_MULTI) as i32
                        + none_walkable_nav_tile.y) as u32,
                )),
                NavTile { walkable: false },
            );
        }

        let key = RenderTileCoord(UVec2::new(render_tile_pos.x, render_tile_pos.y));
        if let Some(exist) = self.render_map_wall.get_mut(&key) {
            exist.push(render_map_tile);
        } else {
            self.render_map_wall.insert(key, vec![render_map_tile]);
        }
    }

    pub fn get_nav_tile(&self, uvec2: UVec2) -> Option<&NavTile> {
        self.nav_map.get(&NavTileCoord(uvec2))
    }

    pub fn get_render_tile_wall(&self, uvec2: UVec2) -> Option<&Vec<RenderTileWall>> {
        self.render_map_wall.get(&RenderTileCoord(uvec2))
    }

    pub fn get_render_tile_floor(&self, uvec2: UVec2) -> Option<&RenderTileFloor> {
        self.render_map_floor.get(&RenderTileCoord(uvec2))
    }

    pub fn position_to_nav_map_tile_coord(&self, position: &Position) -> NavTileCoord {
        // Add half of the map width/height to the position to get its absolute (positive) position
        let position_abs = position.0 + self.map_px_half_size;

        // Divide position_abs by the size of a nav tile to get the tile coordonates
        NavTileCoord(UVec2::new(
            (position_abs.x / NAV_TILE_SIZE) as u32,
            (position_abs.y / NAV_TILE_SIZE) as u32,
        ))
    }

    pub fn position_to_render_map_tile_coord(&self, position: &Position) -> RenderTileCoord {
        // Add half of the map width/height to the position to get its absolute (positive) position
        let position_abs = position.0 + self.map_px_half_size;

        // Divide position_abs by the size of a render tile to get the tile coordonates
        RenderTileCoord(UVec2::new(
            (position_abs.x / RENDER_TILE_SIZE) as u32,
            (position_abs.y / RENDER_TILE_SIZE) as u32,
        ))
    }

    pub fn get_nav_tile_from_position(&self, position: &Position) -> Option<&NavTile> {
        self.nav_map
            .get(&self.position_to_nav_map_tile_coord(position))
    }

    pub fn get_render_tiles_from_position(
        &self,
        position: &Position,
    ) -> Option<&Vec<RenderTileWall>> {
        self.render_map_wall
            .get(&self.position_to_render_map_tile_coord(position))
    }
}
