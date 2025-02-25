use bevy::prelude::*;

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

#[derive(Component, Debug)]
pub struct Map {
    pub data: Vec<Vec<TileType>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TileType {
    Wall,
    Water,
    Floor,
}

fn setup_map(mut commands: Commands) {
    let generated_map = generate_map(20, 20);
    commands.spawn(Map {
        data: generated_map,
    });
}

fn generate_map(width: usize, height: usize) -> Vec<Vec<TileType>> {
    let mut map = vec![vec![TileType::Floor; width]; height];
    for y in 5..10 {
        for x in 5..10 {
            map[y][x] = TileType::Wall;
        }
    }

    map[0][0] = TileType::Water;
    map[height - 1][width - 1] = TileType::Water;
    map
}

pub fn access_map(query: Query<&Map>) {
    for map in query.iter() {
        if !map.data.is_empty() && !map.data[0].is_empty() {
            println!("Map data: {:?}", map.data);
        }
    }
}
