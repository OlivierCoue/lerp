use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::{
    map::{MapGrid, MAP_SIZE},
    protocol::Player,
    shared::PIXEL_METER,
};
use std::cmp::Ordering;

use crate::common::*;

#[derive(Component)]
pub struct IsoZ(pub f32);

#[allow(unused_mut)]
pub fn sync_position_to_transform(
    mut query: Query<
        (&Position, &mut Transform, Option<&IsoZ>, Has<Player>),
        Or<(Added<Position>, Changed<Position>)>,
    >,
    render_config: Res<RenderConfig>,
    map_grid: Res<MapGrid>,
    mut gizmos: Gizmos,
) {
    let mut t = 0;
    for (position, mut transform, opt_izo_z, is_player) in query.iter_mut() {
        // query
        //     .par_iter_mut()
        //     .for_each(|(position, mut transform, opt_izo_z, is_player)| {
        transform.translation = match render_config.mode {
            RenderMode::Iso => {
                let mut coord =
                    cartesian_to_isometric(position.x, position.y).extend(transform.translation.z);

                let mut offset = 0.;

                if let Some(render_nodes) = map_grid.get_render_node_from_world_position(position) {
                    if let Some(render_node) = render_nodes.get(0) {
                        if is_player {
                            gizmos.line_2d(
                                render_node.boundaries[0],
                                render_node.boundaries[1],
                                Color::linear_rgb(1., 0., 0.),
                            );
                            gizmos.line_2d(
                                render_node.boundaries[1],
                                render_node.boundaries[2],
                                Color::linear_rgb(1., 0., 0.),
                            );
                        }
                        let (at_y, order) = render_node.compare_y(&coord.xy());
                        let distance_to_y = (at_y - coord.y).abs();

                        if distance_to_y < 24. {
                            match order {
                                Ordering::Greater => offset = -(24. - distance_to_y),
                                Ordering::Less => {}
                                Ordering::Equal => {}
                            }
                            if is_player {
                                gizmos.line_2d(
                                    Vec2::new(coord.x, coord.y),
                                    Vec2::new(coord.x, coord.y + offset),
                                    Color::linear_rgb(0., 0., 1.),
                                );
                            }
                        };
                    }
                } else {
                    offset = 16.
                }

                // Aply the same y sorting as the one in ECS TileMap lib:
                // https://github.com/StarArawn/bevy_ecs_tilemap/blob/8ccbd6bff829643a9c6f20ad25367234179886b9/src/render/material.rs#L483
                coord.z = 1. + (1. - ((coord.y + offset) / (MAP_SIZE.y as f32 * 80.)));

                // Update y base on IsoZ to make element "fly"
                // Is important to do it after the z computation, because we want to compute z based on the initial y
                // which is the projeted y on the floor and actual y sort reference
                coord.y += opt_izo_z.map_or(0., |iso_z| iso_z.0 * PIXEL_METER);

                coord
            }
            RenderMode::Cart => Vec3::new(position.x, position.y, transform.translation.z),
        };
        // });
    }
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPostUpdate,
            sync_position_to_transform
                .before(TransformSystem::TransformPropagate)
                .run_if(in_state(AppState::Play)),
        );
    }
}
