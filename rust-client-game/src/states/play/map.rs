use crate::states::play::*;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rust_common_game::map::{MapGrid, MapNodeKind};

/// Size of a tile in the grid
pub const MAP_TILE_GRID_SIZE: Vec2 = Vec2::new(64.0, 32.0);
/// Size of a tile the atlas/img of floor
pub const MAP_TILE_IMG_SIZE_FLOOR: Vec2 = Vec2::new(64.0, 32.0);
/// Size of a tile the atlas/img of wall
pub const MAP_TILE_IMG_SIZE_WALL: Vec2 = Vec2::new(64.0, 96.0);

#[derive(Component)]
pub struct TileMapFloor;

#[derive(Component)]
pub struct TileFloor;

#[derive(Component)]
pub struct TileWall;

#[derive(Component)]
pub struct TileObstacle;

pub fn render_map(mut commands: Commands, asset_server: Res<AssetServer>, map_grid: Res<MapGrid>) {
    println!("[render_map]");

    let map_size = TilemapSize {
        x: map_grid.size.x,
        y: map_grid.size.y,
    };

    let tilemap_floor_entity = commands.spawn_empty().id();
    let tilemap_wall_entity = commands.spawn_empty().id();

    let mut tile_storage_floor = TileStorage::empty(map_size);
    let mut tile_storage_wall = TileStorage::empty(map_size);

    for x in 0..map_grid.size.x {
        for y in 0..map_grid.size.y {
            // All nodes should exits for x/y in range of 0..size.x : 0..size.y
            let map_node = map_grid.get_node_xy(x, y).unwrap();

            let tile_pos = TilePos::new(x, y);
            let mut tile_entity = commands.spawn((PlaySceneTag,));

            let tile_storage = match map_node.kind {
                MapNodeKind::Floor => {
                    tile_entity.insert((
                        TileFloor,
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_floor_entity),
                            texture_index: TileTextureIndex(15),
                            ..Default::default()
                        },
                    ));

                    &mut tile_storage_floor
                }
                MapNodeKind::Wall => {
                    tile_entity.insert((
                        TileWall,
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_wall_entity),
                            texture_index: TileTextureIndex(1),
                            ..Default::default()
                        },
                    ));

                    &mut tile_storage_wall
                }
                MapNodeKind::Obtacle => {
                    tile_entity.insert((
                        TileObstacle,
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_wall_entity),
                            texture_index: TileTextureIndex(2),
                            ..Default::default()
                        },
                    ));

                    &mut tile_storage_wall
                }
            };

            tile_storage.set(&tile_pos, tile_entity.id());
        }
    }

    let grid_size = TilemapGridSize {
        x: MAP_TILE_GRID_SIZE.x,
        y: MAP_TILE_GRID_SIZE.y,
    };

    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    // Layer 0 (Floor)
    commands.entity(tilemap_floor_entity).insert((
        PlaySceneTag,
        TileMapFloor,
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage_floor,
            texture: TilemapTexture::Single(
                asset_server.load("assets/iso-tileset-grass-flow-64x32.png"),
            ),
            tile_size: TilemapTileSize {
                x: MAP_TILE_IMG_SIZE_FLOOR.x,
                y: MAP_TILE_IMG_SIZE_FLOOR.y,
            },
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
    ));

    // Layer 1 (Wall/Obstacle) with y sorting enabled
    commands.entity(tilemap_wall_entity).insert((
        PlaySceneTag,
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage_wall,
            texture: TilemapTexture::Single(asset_server.load("assets/iso-tileset-wall-64x32.png")),
            tile_size: TilemapTileSize {
                x: MAP_TILE_IMG_SIZE_WALL.x,
                y: MAP_TILE_IMG_SIZE_WALL.y,
            },
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
            render_settings: TilemapRenderSettings {
                render_chunk_size: UVec2::new(3, 1),
                y_sort: true,
            },

            ..Default::default()
        },
    ));
}
