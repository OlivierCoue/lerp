use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::{
    map::{get_obstacles, MapGrid, MapNode, MapNodePos, MAP_SIZE},
    shared::PIXEL_METER,
    wall::Wall,
};

pub fn setup_map(mut commands: Commands, mut map_grid: ResMut<MapGrid>) {
    println!("[setup_map]");

    map_grid.map.clear();
    map_grid.size = MAP_SIZE;
    let obstacles = get_obstacles();

    for x in 0..MAP_SIZE.x {
        for y in 0..MAP_SIZE.y {
            let center_offest = PIXEL_METER / 2.0;
            let cart_coord = Vec3::new(
                x as f32 * PIXEL_METER - 1600. + center_offest,
                y as f32 * PIXEL_METER - 1600. + center_offest,
                0.,
            );

            let is_border = (x == 0) || (x == 99) || (y == 0) || (y == 99);
            let is_obstacle =
                !is_border && ((x % 6 == 0 && y % 6 == 0) || obstacles.contains(&UVec2::new(x, y)));

            map_grid.map.insert(
                MapNodePos(UVec2::new(x, y)),
                MapNode {
                    walkable: !is_border && !is_obstacle,
                },
            );

            if is_obstacle && !is_border {
                commands.spawn((
                    Wall,
                    Position::from_xy(cart_coord.x, cart_coord.y),
                    RigidBody::Static,
                    Collider::rectangle(PIXEL_METER, PIXEL_METER),
                ));
            }
        }
    }

    // // TOP
    commands.spawn((
        Wall,
        Position::from_xy(0., 1600. - PIXEL_METER / 2.),
        RigidBody::Static,
        Collider::rectangle(100. * PIXEL_METER, PIXEL_METER),
    ));
    // // BOTOM
    commands.spawn((
        Wall,
        Position::from_xy(0., 0. - 1600. + PIXEL_METER / 2.),
        RigidBody::Static,
        Collider::rectangle(100. * PIXEL_METER, PIXEL_METER),
    ));
    // // LEFT
    commands.spawn((
        Wall,
        Position::from_xy(-1600. + PIXEL_METER / 2., 0.),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, 100. * PIXEL_METER),
    ));
    // // RIGHT
    commands.spawn((
        Wall,
        Position::from_xy(1600. - PIXEL_METER / 2., 0.),
        RigidBody::Static,
        Collider::rectangle(PIXEL_METER, 100. * PIXEL_METER),
    ));
}
