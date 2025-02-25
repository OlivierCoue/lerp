use bevy::{prelude::*, utils::HashMap};

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default());
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct TilePos(pub UVec2);

#[derive(Resource, Default)]
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TileKind {
    Wall,
    Water,
    Floor,
}

pub fn generate_map(map: &mut Map, size: UVec2) {
    map.reset();
    map.render_grid_size = size;

    for x in 0..map.render_grid_size.x {
        for y in 0..map.render_grid_size.y {
            let kind = if x == 0
                || x == map.render_grid_size.x - 1
                || y == 0
                || y == map.render_grid_size.y - 1
            {
                TileKind::Wall
            } else {
                TileKind::Floor
            };

            map.set_tile(TilePos(UVec2::new(x, y)), kind);
        }
    }

    map.set_tile(TilePos(UVec2::new(5, 5)), TileKind::Water);
}
