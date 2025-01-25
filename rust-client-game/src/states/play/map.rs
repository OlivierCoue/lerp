use crate::states::play::*;

use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_tilemap::prelude::*;
use rand::seq::SliceRandom;
use rust_common_game::{map::MapGrid, shared::RENDER_TILE_SIZE};

/// Size of a tile in the grid
pub const MAP_TILE_GRID_SIZE: Vec2 = Vec2::new(160.0, 80.0);
/// Size of a tile the atlas/img of floor
pub const MAP_TILE_IMG_SIZE_FLOOR: Vec2 = Vec2::new(160.0, 80.0);
/// Size of a tile the atlas/img of wall
pub const MAP_TILE_IMG_SIZE_WALL: Vec2 = Vec2::new(160.0, 320.0);

#[derive(Component)]
pub struct TileMapFloor;

#[derive(Component)]
pub struct TileFloor;

#[derive(Component)]
pub struct TileWall;

pub fn render_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_grid: Res<MapGrid>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("[render_map]");

    let tile_map_size = TilemapSize {
        x: map_grid.render_map_size.x,
        y: map_grid.render_map_size.y,
    };

    let tilemap_floor_entity = commands.spawn_empty().id();
    let mut tile_storage_floor = TileStorage::empty(tile_map_size);

    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(
        UVec2::new(
            MAP_TILE_IMG_SIZE_WALL.x as u32,
            MAP_TILE_IMG_SIZE_WALL.y as u32,
        ),
        29,
        1,
        None,
        None,
    );

    let wall_atlas_layout = texture_atlas_layouts.add(layout.clone());
    let wall_texture: Handle<Image> = asset_server.load("assets/iso-tileset-wall-cata-160x80.png");

    let floor_tile_indexes = [
        4, 5, 6, 7, 9, 10, 11, 14, 15, 17, 17, 17, 17, 17, 17, 17, 17,
    ];
    let mut rng = rand::thread_rng();

    for x in 0..map_grid.render_map_size.x {
        for y in 0..map_grid.render_map_size.y {
            let tile_pos = TilePos::new(x, y);

            let floor_tile_entity = commands
                .spawn((
                    PlaySceneTag,
                    TileFloor,
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_floor_entity),
                        texture_index: TileTextureIndex(
                            *floor_tile_indexes.choose(&mut rng).unwrap(),
                        ),
                        ..Default::default()
                    },
                ))
                .id();
            tile_storage_floor.set(&tile_pos, floor_tile_entity);

            let iso_coord = cartesian_to_isometric(
                (x as f32 * RENDER_TILE_SIZE) - map_grid.map_px_size.x / 2. + RENDER_TILE_SIZE / 2.,
                (y as f32 * RENDER_TILE_SIZE) - map_grid.map_px_size.x / 2. + RENDER_TILE_SIZE / 2.,
            );

            if let Some(map_nodes) = map_grid.get_render_node_xy(x, y) {
                for map_node in map_nodes {
                    let z = 1. + (1. - ((iso_coord.y) / (map_grid.render_map_size.y as f32 * 80.)));
                    commands.spawn((
                        PlaySceneTag,
                        TileWall,
                        Sprite {
                            image: wall_texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: wall_atlas_layout.clone(),
                                index: map_node.kind.atlas_index(),
                            }),
                            anchor: Anchor::Custom(Vec2::new(
                                0.0,
                                -(0.5 - (1. / (MAP_TILE_IMG_SIZE_WALL.y / RENDER_TILE_SIZE)) / 2.),
                            )),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(iso_coord.x, iso_coord.y, z)),
                    ));
                }
            }
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
            size: tile_map_size,
            storage: tile_storage_floor,
            texture: TilemapTexture::Single(
                asset_server.load("assets/iso-tileset-floor-cata-160x80.png"),
            ),
            tile_size: TilemapTileSize {
                x: MAP_TILE_IMG_SIZE_FLOOR.x,
                y: MAP_TILE_IMG_SIZE_FLOOR.y,
            },
            transform: get_tilemap_center_transform(&tile_map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
    ));
}
