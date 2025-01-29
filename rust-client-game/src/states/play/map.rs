use crate::states::play::*;

use bevy::{prelude::*, sprite::Anchor, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use rust_common_game::{
    map::map::Map,
    shared::RENDER_TILE_SIZE,
    utils::{cartesian_to_isometric, isometric_to_cartesian},
};

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

const CHUNK_SIZE: u32 = 5;
const CHUNK_SIZE_VEC: UVec2 = UVec2::new(CHUNK_SIZE, CHUNK_SIZE);
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE_VEC.x * 2,
    y: CHUNK_SIZE_VEC.y * 2,
};

/// Size of a tile in the grid
pub const MAP_TILE_GRID_SIZE: Vec2 = Vec2::new(160.0, 80.0);
/// Size of a tile the atlas/img of floor
pub const MAP_TILE_IMG_SIZE_FLOOR: Vec2 = Vec2::new(160.0, 80.0);
/// Size of a tile the atlas/img of wall
pub const MAP_TILE_IMG_SIZE_WALL: Vec2 = Vec2::new(160.0, 320.0);

#[derive(Component)]
pub struct TileMapFloorChunk {
    pub position: IVec2,
    pub walls: Vec<Entity>,
}

#[derive(Component)]
pub struct TileMapFlowField;

#[derive(Component)]
pub struct TileWall;

#[derive(Component)]
pub struct TileFlowField;

fn spawn_map_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    map_grid: &Map,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE_VEC.into());

    let offset_x = map_grid.render_map_size.x as i32 / CHUNK_SIZE as i32 / 2 + chunk_pos.x;
    let offset_y = map_grid.render_map_size.y as i32 / CHUNK_SIZE as i32 / 2 + chunk_pos.y;
    // Chunk is oob, stopping here
    if offset_x < 0 || offset_y < 0 {
        return;
    }

    let floor_tile_indexes = [
        4, 5, 6, 7, 9, 10, 11, 14, 15, 17, 17, 17, 17, 17, 17, 17, 17,
    ];
    let mut rng = rand::rng();

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
    let mut walls = Vec::new();

    for chunk_x in 0..CHUNK_SIZE_VEC.x {
        for chunk_y in 0..CHUNK_SIZE_VEC.y {
            let x = offset_x as u32 * CHUNK_SIZE + chunk_x;
            let y = offset_y as u32 * CHUNK_SIZE + chunk_y;

            // Spawn floor if any
            if let Some(_map_tile_floor) = map_grid.get_render_tile_floor(UVec2::new(x, y)) {
                let tile_pos = TilePos {
                    x: chunk_x,
                    y: chunk_y,
                };
                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(
                            *floor_tile_indexes.choose(&mut rng).unwrap(),
                        ),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, tile_entity);
            }

            let cart_coord = (Vec2::new(x as f32, y as f32) * RENDER_TILE_SIZE)
                - map_grid.map_px_half_size
                + RENDER_TILE_SIZE / 2.;
            let iso_coord = cartesian_to_isometric(cart_coord.x, cart_coord.y);

            // Spawn wall if any
            if let Some(map_nodes) = map_grid.get_render_tile_wall(UVec2::new(x, y)) {
                for map_node in map_nodes {
                    let z = 1. + (1. - ((iso_coord.y) / (map_grid.map_px_size.y)));
                    let wall_entity = commands.spawn((
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
                    walls.push(wall_entity.id());
                }
            }
        }
    }

    let grid_size = TilemapGridSize {
        x: MAP_TILE_GRID_SIZE.x,
        y: MAP_TILE_GRID_SIZE.y,
    };

    let iso_coord = cartesian_to_isometric(
        chunk_pos.x as f32 * CHUNK_SIZE as f32 * RENDER_TILE_SIZE + RENDER_TILE_SIZE / 2.,
        chunk_pos.y as f32 * CHUNK_SIZE as f32 * RENDER_TILE_SIZE + RENDER_TILE_SIZE / 2.,
    );

    let transform = Transform::from_translation(Vec3::new(iso_coord.x, iso_coord.y, 0.0));
    let texture_handle: Handle<Image> =
        asset_server.load("assets/iso-tileset-floor-cata-160x80.png");
    commands.entity(tilemap_entity).insert((
        PlaySceneTag,
        TileMapFloorChunk {
            position: chunk_pos,
            walls,
        },
        TilemapBundle {
            grid_size,
            size: CHUNK_SIZE_VEC.into(),
            map_type: TilemapType::Isometric(IsoCoordSystem::Diamond),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: TilemapTileSize {
                x: MAP_TILE_IMG_SIZE_FLOOR.x,
                y: MAP_TILE_IMG_SIZE_FLOOR.y,
            },
            transform,
            render_settings: TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..default()
            },
            ..default()
        },
    ));
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    (isometric_to_cartesian(
        camera_pos.x / CHUNK_SIZE as f32 / RENDER_TILE_SIZE,
        camera_pos.y / CHUNK_SIZE as f32 / RENDER_TILE_SIZE,
    ))
    .as_ivec2()
}

pub fn spawn_map_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    map_grid: Res<Map>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 3)..(camera_chunk_pos.y + 3) {
            for x in (camera_chunk_pos.x - 3)..(camera_chunk_pos.x + 3) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_map_chunk(
                        &mut commands,
                        &asset_server,
                        IVec2::new(x, y),
                        &map_grid,
                        &mut texture_atlas_layouts,
                    );
                }
            }
        }
    }
}

pub fn despawn_outofrange_map_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &TileMapFloorChunk)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for camera_transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&camera_transform.translation.xy());
        for (entity, chunk) in chunks_query.iter() {
            let distance = (camera_chunk_pos.x - chunk.position.x)
                .abs()
                .max((camera_chunk_pos.y - chunk.position.y).abs());
            if distance > 3 {
                chunk_manager.spawned_chunks.remove(&chunk.position);
                commands.entity(entity).despawn_recursive();
                for wall_entity in &chunk.walls {
                    commands.entity(*wall_entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn render_flow_field(
    debug_config: Res<DebugConfig>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_grid: Res<Map>,
) {
    if !debug_config.show_flow_field {
        return;
    }

    println!("[render_flow_field]");

    let tile_map_size = TilemapSize {
        x: map_grid.nav_map_size.x,
        y: map_grid.nav_map_size.y,
    };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tile_map_size);

    for x in 0..map_grid.nav_map_size.x {
        for y in 0..map_grid.nav_map_size.y {
            let tile_pos = TilePos::new(x, y);

            let tile_entity = commands
                .spawn((
                    PlaySceneTag,
                    TileFlowField,
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(8),
                        ..Default::default()
                    },
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let grid_size = TilemapGridSize { x: 32., y: 16. };

    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    commands.entity(tilemap_entity).insert((
        PlaySceneTag,
        TileMapFlowField,
        TilemapBundle {
            grid_size,
            map_type,
            size: tile_map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(
                asset_server.load("assets/iso-tileset-flow-field-32x16.png"),
            ),
            tile_size: TilemapTileSize { x: 32., y: 16. },
            transform: get_tilemap_center_transform(&tile_map_size, &grid_size, &map_type, 3.0),
            ..Default::default()
        },
    ));
}
