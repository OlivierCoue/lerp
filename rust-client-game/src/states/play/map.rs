use crate::common::*;
use crate::states::play::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

// const SPRITE_SCALE_FACTOR: f32 = 4.;

pub fn setup_map(
    mut commands: Commands,
    render_config: Res<RenderConfig>,
    asset_server: Res<AssetServer>,
) {
    println!("[setup_map]");

    for row in 0..100 {
        for col in 0..100 {
            let color = if (row + col) % 2 == 0 {
                Color::srgb_u8(166, 174, 191)
            } else {
                Color::srgb_u8(197, 211, 232)
            };

            let cart_coord = Vec3::new(
                col as f32 * ENTITY_SIZE - 1600.,
                row as f32 * ENTITY_SIZE - 1600.,
                0.,
            );

            let entity = commands
                .spawn((PlaySceneTag, Position::from_xy(cart_coord.x, cart_coord.y)))
                .id();

            let is_border = (row == 0) || (row == 99) || (col == 0) || (col == 99);
            let is_obstacle = row % 6 == 0 && col % 6 == 0;

            if is_border || is_obstacle {
                if !is_border {
                    commands.entity(entity).insert((
                        RigidBody::Static,
                        Collider::rectangle(ENTITY_SIZE - 1., ENTITY_SIZE - 1.),
                    ));
                }

                match render_config.mode {
                    RenderMode::Iso => commands.entity(entity).insert(SpriteBundle {
                        texture: asset_server.load("assets/stone_W.png"),
                        // transform: Transform::default().with_scale(Vec3::new(
                        //     1. / SPRITE_SCALE_FACTOR,
                        //     1. / SPRITE_SCALE_FACTOR,
                        //     1.,
                        // ))
                        sprite: Sprite {
                            anchor: Anchor::Custom(Vec2::new(0., -0.375)),
                            ..Default::default()
                        },
                        ..default()
                    }),
                    RenderMode::Cart => commands.entity(entity).insert(SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb_u8(255, 0, 0),
                            custom_size: Some(Vec2::new(ENTITY_SIZE, ENTITY_SIZE)),
                            ..default()
                        },
                        ..default()
                    }),
                };
            } else {
                match render_config.mode {
                    RenderMode::Iso => commands.entity(entity).insert(SpriteBundle {
                        texture: asset_server.load("assets/dirt_W.png"),
                        sprite: Sprite {
                            anchor: Anchor::Custom(Vec2::new(0., -0.375)),
                            ..Default::default()
                        },
                        ..default()
                    }),
                    RenderMode::Cart => commands.entity(entity).insert(SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(ENTITY_SIZE, ENTITY_SIZE)),
                            ..default()
                        },
                        ..default()
                    }),
                };
            }
        }
    }

    // TOP
    commands.spawn((
        Position::from_xy(0., 1600. - ENTITY_SIZE),
        RigidBody::Static,
        Collider::rectangle(100. * ENTITY_SIZE, ENTITY_SIZE),
    ));
    // BOTOM
    commands.spawn((
        Position::from_xy(0., 0. - 1600.),
        RigidBody::Static,
        Collider::rectangle(100. * ENTITY_SIZE, ENTITY_SIZE),
    ));
    // LEFT
    commands.spawn((
        Position::from_xy(-1600., 0.),
        RigidBody::Static,
        Collider::rectangle(ENTITY_SIZE, 100. * ENTITY_SIZE),
    ));
    // RIGHT
    commands.spawn((
        Position::from_xy(1600. - ENTITY_SIZE, 0.),
        RigidBody::Static,
        Collider::rectangle(ENTITY_SIZE, 100. * ENTITY_SIZE),
    ));
}
