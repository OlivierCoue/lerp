use bevy::{prelude::*, utils::HashMap};
use rand::{rng, Rng};
/// Number of pixels per one meter
pub const PIXEL_METER: f32 = 32.;

pub const NAV_TILE_SIZE: f32 = PIXEL_METER / 2.;
pub const RENDER_TO_NAV_TILE_MULTI: u32 = 5;
pub const RENDER_TILE_SIZE: f32 = NAV_TILE_SIZE * RENDER_TO_NAV_TILE_MULTI as f32;

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default());
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct TilePos(pub UVec2);

#[derive(Resource, Default, Clone)]
pub struct Map {
    render_grid: HashMap<TilePos, TileKind>,
    pub render_grid_size: UVec2,
}
impl Map {
    pub fn reset(&mut self) {
        self.render_grid.clear();
        self.render_grid_size = UVec2::ZERO;
    }

    pub fn set_tile(&mut self, pos: TilePos, kind: TileKind) {
        self.render_grid.insert(pos, kind);
    }

    pub fn get_tile(&self, pos: &TilePos) -> Option<&TileKind> {
        self.render_grid.get(pos)
    }
    pub fn generate_bsp_floor(&mut self, iterations: u32, min_size: UVec2) {
        let mut rng = rng();

        let mut regions = vec![(UVec2::ZERO, self.render_grid_size)];

        for _ in 0..iterations {
            let mut new_regions = Vec::new();
            for (start, size) in regions {
                if size.x < min_size.x || size.y < min_size.y {
                    new_regions.push((start, size));
                    continue;
                }

                let axis = rng.random_range(0..2);
                let first_region;
                let second_region;

                if axis == 0 {
                    let max_threshold = size.x - min_size.x;
                    if max_threshold <= min_size.x {
                        new_regions.push((start, size));
                        continue;
                    }

                    let threshold = rng.random_range(min_size.x..=max_threshold);
                    first_region = (start, UVec2::new(threshold, size.y));
                    second_region = (
                        UVec2::new(start.x + threshold, start.y),
                        UVec2::new(size.x - threshold, size.y),
                    );
                } else {
                    let max_threshold = size.y - min_size.y;
                    if max_threshold <= min_size.y {
                        new_regions.push((start, size));
                        continue;
                    }
                    let threshold = rng.random_range(min_size.y..=max_threshold);
                    first_region = (start, UVec2::new(size.x, threshold));
                    second_region = (
                        UVec2::new(start.x, start.y + threshold),
                        UVec2::new(size.x, size.y - threshold),
                    );
                }
                if rng.random_bool(0.5) {
                    new_regions.push(first_region);
                    new_regions.push(second_region);
                } else {
                    new_regions.push(second_region);
                    new_regions.push(first_region);
                }
            }
            regions = new_regions;
        }
        for (start, size) in regions.last() {
            for x in start.x..(start.x + size.x) {
                for y in start.y..(start.y + size.y) {
                    self.set_tile(TilePos(UVec2::new(x, y)), TileKind::Floor);
                }
            }
        }
    }
    pub fn generate_map(&mut self, size: UVec2) {
        self.reset();
        self.render_grid_size = size;
        // map.generate_bsp_floor(5, UVec2::new(10, 10));

        for x in 0..self.render_grid_size.x {
            for y in 0..self.render_grid_size.y {
                // let kind = if x == 0
                //     || x == map.render_grid_size.x - 1
                //     || y == 0
                //     || y == map.render_grid_size.y - 1
                // {
                //     TileKind::Wall
                // } else {
                //     TileKind::Floor
                // };

                self.set_tile(TilePos(UVec2::new(x, y)), TileKind::Wall);
            }
        }
        // map.set_tile(TilePos(UVec2::new(5, 5)), TileKind::Water);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TileKind {
    Wall,
    Water,
    Floor,
}
