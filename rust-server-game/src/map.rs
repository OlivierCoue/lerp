use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::shared::PIXEL_METER;

pub fn setup_map(mut commands: Commands) {
    println!("[setup_map]");

    for row in 0..100 {
        for col in 0..100 {
            let center_offest = PIXEL_METER / 2.0;
            let cart_coord = Vec3::new(
                col as f32 * PIXEL_METER - 1600. + center_offest,
                row as f32 * PIXEL_METER - 1600. + center_offest,
                0.,
            );

            let is_border = (row == 0) || (row == 99) || (col == 0) || (col == 99);
            let is_obstacle = row % 6 == 0 && col % 6 == 0;

            if is_obstacle && !is_border {
                commands.spawn((
                    Position::from_xy(cart_coord.x, cart_coord.y),
                    RigidBody::Static,
                    Collider::rectangle(PIXEL_METER, PIXEL_METER),
                ));
            }
        }
    }

    // // TOP
    commands.spawn((
        Position::from_xy(0., 1600. - PIXEL_METER / 2.),
        RigidBody::Static,
        Collider::rectangle(100. * PIXEL_METER, PIXEL_METER),
    ));
    // // BOTOM
    commands.spawn((
        Position::from_xy(0., 0. - 1600. + PIXEL_METER / 2.),
        RigidBody::Static,
        Collider::rectangle(100. * PIXEL_METER, PIXEL_METER),
    ));
    // // LEFT
    commands.spawn((
        Position::from_xy(-1600. + PIXEL_METER / 2., 0.),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, 100. * PIXEL_METER),
    ));
    // // RIGHT
    commands.spawn((
        Position::from_xy(1600. - PIXEL_METER / 2., 0.),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, 100. * PIXEL_METER),
    ));
}
