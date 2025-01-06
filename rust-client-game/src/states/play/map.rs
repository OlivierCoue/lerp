use crate::common::*;
use crate::states::play::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rust_common_game::shared::PIXEL_METER;

// const SPRITE_SCALE_FACTOR: f32 = 4.;

pub fn setup_map(
    mut commands: Commands,
    render_config: Res<RenderConfig>,
    asset_server: Res<AssetServer>,
) {
    println!("[setup_map]");

    // Load the first sprite atlas
    let texture_handle = asset_server.load("assets/iso-tileset-grass-64x32.png");
    let map_size = TilemapSize { x: 100, y: 100 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    for row in 0..100 {
        for col in 0..100 {
            let tile_pos = TilePos { x: row, y: col };

            let color = if (row + col) % 2 == 0 {
                Color::srgb_u8(166, 174, 191)
            } else {
                Color::srgb_u8(197, 211, 232)
            };

            let center_offest = PIXEL_METER / 2.0;
            let cart_coord = Vec3::new(
                col as f32 * PIXEL_METER - 1600. + center_offest,
                row as f32 * PIXEL_METER - 1600. + center_offest,
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

            let is_border = (row == 0) || (row == 99) || (col == 0) || (col == 99);
            let is_obstacle = row % 6 == 0 && col % 6 == 0;

            if is_border || is_obstacle {
                if !is_border {
                    commands.entity(entity).insert((
                        RigidBody::Static,
                        Collider::rectangle(PIXEL_METER, PIXEL_METER),
                    ));
                }

                match render_config.mode {
                    RenderMode::Iso => {
                        let tile_entity = commands
                            .spawn((
                                PlaySceneTag,
                                TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(tilemap_entity),
                                    texture_index: TileTextureIndex(1),
                                    ..Default::default()
                                },
                            ))
                            .id();
                        tile_storage.set(&tile_pos, tile_entity);
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
                                TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(tilemap_entity),
                                    texture_index: TileTextureIndex(15),
                                    ..Default::default()
                                },
                            ))
                            .id();
                        tile_storage.set(&tile_pos, tile_entity);
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

    let tilemap_transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);
    commands.entity(tilemap_entity).insert((
        PlaySceneTag,
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: tilemap_transform,
            ..Default::default()
        },
    ));

    // // TOP
    commands.spawn((
        PlaySceneTag,
        Visibility::default(),
        Position::from_xy(0., 1600. - PIXEL_METER / 2.),
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(100. * PIXEL_METER, PIXEL_METER),
    ));
    // // BOTOM
    commands.spawn((
        PlaySceneTag,
        Visibility::default(),
        Position::from_xy(0., 0. - 1600. + PIXEL_METER / 2.),
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(100. * PIXEL_METER, PIXEL_METER),
    ));
    // // LEFT
    commands.spawn((
        PlaySceneTag,
        Visibility::default(),
        Position::from_xy(-1600. + PIXEL_METER / 2., 0.),
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, 100. * PIXEL_METER),
    ));
    // // RIGHT
    commands.spawn((
        PlaySceneTag,
        Visibility::default(),
        Position::from_xy(1600. - PIXEL_METER / 2., 0.),
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, 100. * PIXEL_METER),
    ));
}
