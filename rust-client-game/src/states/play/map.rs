use crate::common::*;
use crate::states::play::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rust_common_game::map::{get_obstacles, MapGrid, MapNode, MapNodePos, MAP_SIZE};
use rust_common_game::shared::PIXEL_METER;
use rust_common_game::wall::Wall;

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

pub fn setup_map(
    mut commands: Commands,
    render_config: Res<RenderConfig>,
    asset_server: Res<AssetServer>,
    mut map_grid: ResMut<MapGrid>,
) {
    println!("[setup_map]");

    let texture_handle_floor = asset_server.load("assets/iso-tileset-grass-flow-64x32.png");
    let texture_handle_wall = asset_server.load("assets/iso-tileset-wall-64x32.png");

    let map_size = TilemapSize {
        x: MAP_SIZE.x,
        y: MAP_SIZE.y,
    };

    let tilemap_floor_entity = commands.spawn_empty().id();
    let tilemap_wall_entity = commands.spawn_empty().id();
    let mut tile_storage_floor = TileStorage::empty(map_size);
    let mut tile_storage_wall = TileStorage::empty(map_size);

    let grid_size = TilemapGridSize {
        x: MAP_TILE_GRID_SIZE.x,
        y: MAP_TILE_GRID_SIZE.y,
    };
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    let obstacles = get_obstacles();

    map_grid.map.clear();
    map_grid.size = MAP_SIZE;

    for x in 0..100 {
        for y in 0..100 {
            let tile_pos = TilePos { x, y };

            let color = if (x + y) % 2 == 0 {
                Color::srgb_u8(166, 174, 191)
            } else {
                Color::srgb_u8(197, 211, 232)
            };

            let center_offest = PIXEL_METER / 2.0;
            let cart_coord = Vec3::new(
                x as f32 * PIXEL_METER - 1600. + center_offest,
                y as f32 * PIXEL_METER - 1600. + center_offest,
                0.,
            );

            let entity = commands
                .spawn((
                    PlaySceneTag,
                    Position::from_xy(cart_coord.x, cart_coord.y),
                    Transform::default(),
                    Visibility::default(),
                ))
                .id();

            let is_border = (x == 0) || (x == 99) || (y == 0) || (y == 99);
            let is_obstacle = !is_border
                && ((x % 6 == 0 && y % 6 == 0)
                    || obstacles.contains(&UVec2::new(tile_pos.x, tile_pos.y)));

            map_grid.map.insert(
                MapNodePos(UVec2::new(tile_pos.x, tile_pos.y)),
                MapNode {
                    walkable: !is_border && !is_obstacle,
                },
            );

            if is_border || is_obstacle {
                if !is_border {
                    commands.entity(entity).insert((
                        Wall,
                        RigidBody::Static,
                        Collider::rectangle(PIXEL_METER, PIXEL_METER),
                    ));
                }
                match render_config.mode {
                    RenderMode::Iso => {
                        let tile_entity_floor = commands
                            .spawn((
                                PlaySceneTag,
                                TileFloor,
                                TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(tilemap_floor_entity),
                                    texture_index: TileTextureIndex(1),
                                    ..Default::default()
                                },
                            ))
                            .id();
                        tile_storage_floor.set(&tile_pos, tile_entity_floor);

                        let texture_index = if is_obstacle { 2 } else { 1 };
                        let tile_entity_wall = commands
                            .spawn((
                                PlaySceneTag,
                                TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(tilemap_wall_entity),
                                    texture_index: TileTextureIndex(texture_index),
                                    ..Default::default()
                                },
                            ))
                            .id();
                        tile_storage_wall.set(&tile_pos, tile_entity_wall);
                    }
                    RenderMode::Cart => {
                        commands.entity(entity).insert(Sprite {
                            color: Color::srgb_u8(255, 0, 0),
                            custom_size: Some(Vec2::new(PIXEL_METER, PIXEL_METER)),
                            ..default()
                        });
                    }
                };
            } else {
                match render_config.mode {
                    RenderMode::Iso => {
                        let tile_entity = commands
                            .spawn((
                                PlaySceneTag,
                                TileFloor,
                                TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(tilemap_floor_entity),
                                    texture_index: TileTextureIndex(15),
                                    ..Default::default()
                                },
                            ))
                            .id();
                        tile_storage_floor.set(&tile_pos, tile_entity);
                    }
                    RenderMode::Cart => {
                        commands.entity(entity).insert(Sprite {
                            color,
                            custom_size: Some(Vec2::new(PIXEL_METER, PIXEL_METER)),
                            ..default()
                        });
                    }
                };
            }
        }
    }

    let tilemap_transform_floor =
        get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);
    commands.entity(tilemap_floor_entity).insert((
        PlaySceneTag,
        TileMapFloor,
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage_floor,
            texture: TilemapTexture::Single(texture_handle_floor),
            tile_size: TilemapTileSize {
                x: MAP_TILE_IMG_SIZE_FLOOR.x,
                y: MAP_TILE_IMG_SIZE_FLOOR.y,
            },
            transform: tilemap_transform_floor,
            ..Default::default()
        },
    ));
    let tilemap_transform_wall =
        get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0);
    commands.entity(tilemap_wall_entity).insert((
        PlaySceneTag,
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage_wall,
            texture: TilemapTexture::Single(texture_handle_wall),
            tile_size: TilemapTileSize {
                x: MAP_TILE_IMG_SIZE_WALL.x,
                y: MAP_TILE_IMG_SIZE_WALL.y,
            },
            transform: tilemap_transform_wall,
            render_settings: TilemapRenderSettings {
                render_chunk_size: UVec2::new(3, 1),
                y_sort: true,
            },

            ..Default::default()
        },
    ));

    // // TOP
    commands.spawn((
        Wall,
        PlaySceneTag,
        Visibility::default(),
        Position::from_xy(0., 1600. - PIXEL_METER / 2.),
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(100. * PIXEL_METER, PIXEL_METER),
    ));
    // // BOTOM
    commands.spawn((
        Wall,
        PlaySceneTag,
        Visibility::default(),
        Position::from_xy(0., 0. - 1600. + PIXEL_METER / 2.),
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(100. * PIXEL_METER, PIXEL_METER),
    ));
    // // LEFT
    commands.spawn((
        Wall,
        PlaySceneTag,
        Visibility::default(),
        Position::from_xy(-1600. + PIXEL_METER / 2., 0.),
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, 100. * PIXEL_METER),
    ));
    // // RIGHT
    commands.spawn((
        Wall,
        PlaySceneTag,
        Visibility::default(),
        Position::from_xy(1600. - PIXEL_METER / 2., 0.),
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, 100. * PIXEL_METER),
    ));
}
