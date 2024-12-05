use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::settings::*;

pub fn setup_map(mut commands: Commands) {
    println!("[setup_map]");

    for row in 0..100 {
        for col in 0..100 {
            let cart_coord = Vec3::new(
                col as f32 * ENTITY_SIZE - 1600.,
                row as f32 * ENTITY_SIZE - 1600.,
                0.,
            );

            let is_border = (row == 0) || (row == 99) || (col == 0) || (col == 99);
            let is_obstacle = row % 6 == 0 && col % 6 == 0;

            if is_obstacle && !is_border {
                commands.spawn((
                    Position::from_xy(cart_coord.x, cart_coord.y),
                    RigidBody::Static,
                    Collider::rectangle(ENTITY_SIZE - 1., ENTITY_SIZE - 1.),
                ));
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
