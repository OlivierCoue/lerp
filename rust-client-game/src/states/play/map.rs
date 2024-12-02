use crate::common::*;
use crate::states::play::*;
use avian2d::prelude::*;
use bevy::prelude::*;

const SPRITE_SCALE_FACTOR: f32 = 4.;

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

            if (row % 6 == 0 && col % 6 == 0)
                || (row == 0)
                || (row == 99)
                || (col == 0)
                || (col == 99)
            {
                commands.entity(entity).insert((
                    RigidBody::Static,
                    Restitution::new(1.0),
                    Friction::new(0.0),
                    Collider::rectangle(ENTITY_SIZE, ENTITY_SIZE),
                ));

                match render_config.mode {
                    RenderMode::Iso => commands.entity(entity).insert(SpriteBundle {
                        texture: asset_server.load("assets/stone_W.png"),
                        transform: Transform::default().with_scale(Vec3::new(
                            1. / SPRITE_SCALE_FACTOR,
                            1. / SPRITE_SCALE_FACTOR,
                            1.,
                        )),
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
                        transform: Transform::default().with_scale(Vec3::new(
                            1. / SPRITE_SCALE_FACTOR,
                            1. / SPRITE_SCALE_FACTOR,
                            1.,
                        )),
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
}
