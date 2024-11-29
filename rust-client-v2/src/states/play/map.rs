use crate::states::play::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn setup_map(mut commands: Commands) {
    println!("[setup_map]");

    // Spawn the grid as individual tiles (using sprites)
    for row in 0..100 {
        for col in 0..100 {
            let color = if (row + col) % 2 == 0 {
                Color::srgb_u8(166, 174, 191)
            } else {
                Color::srgb_u8(197, 211, 232)
            };

            if (row % 6 == 0 && col % 6 == 0)
                || (row == 0)
                || (row == 99)
                || (col == 0)
                || (col == 99)
            {
                let physics_parent = (
                    PlaySceneTag,
                    RigidBody::Static,
                    Restitution::new(1.0),
                    Friction::new(0.0),
                    Collider::rectangle(ENTITY_SIZE, ENTITY_SIZE),
                    Transform::from_xyz(
                        col as f32 * ENTITY_SIZE - 1600.0, // Center grid horizontally
                        row as f32 * ENTITY_SIZE - 1600.0, // Center grid vertically
                        0.0,                               // Grid at z = 0
                    ),
                );

                let render_child = (
                    PlaySceneTag,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb_u8(255, 0, 0),
                            custom_size: Some(Vec2::new(ENTITY_SIZE, ENTITY_SIZE)), // Each grid tile is 32x32
                            ..default()
                        },
                        transform: Transform::default(),
                        ..default()
                    },
                );

                spawn_physics_render_pair(&mut commands, physics_parent, render_child);
            } else {
                // commands.spawn((
                //     PlaySceneTag,
                //     SpriteBundle {
                //         sprite: Sprite {
                //             color,
                //             custom_size: Some(Vec2::new(ENTITY_SIZE, ENTITY_SIZE)), // Each grid tile is 32x32
                //             ..default()
                //         },
                //         transform: Transform::from_xyz(
                //             col as f32 * ENTITY_SIZE - 1600.0, // Center grid horizontally
                //             row as f32 * ENTITY_SIZE - 1600.0, // Center grid vertically
                //             0.0,                               // Grid at z = 0
                //         ),
                //         ..default()
                //     },
                // ));
            }
        }
    }
}
