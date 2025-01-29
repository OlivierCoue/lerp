use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::{
    map::map::Map, protocol::Player, shared::PIXEL_METER, utils::cartesian_to_isometric,
};

use crate::common::*;

/// Virual Z value to render entity above floor.
///
/// The value is in meter, so adding IsoZ(2.) will render the entity 2 meters above floor.
#[derive(Component)]
pub struct IsoZ(pub f32);

#[allow(unused_mut)]
pub fn sync_position_to_transform(
    mut query: Query<
        (&Position, &mut Transform, Option<&IsoZ>, Has<Player>),
        Or<(Added<Position>, Changed<Position>)>,
    >,
    render_config: Res<RenderConfig>,
    map: Res<Map>,
    mut gizmos: Gizmos,
) {
    for (position, mut transform, opt_izo_z, is_player) in query.iter_mut() {
        transform.translation = match render_config.mode {
            RenderMode::Iso => {
                let mut iso_coord =
                    cartesian_to_isometric(position.x, position.y).extend(transform.translation.z);

                let mut y_offset = 0.;

                if let Some(current_render_tiles) = map.get_render_tiles_from_position(position) {
                    if let Some(current_render_tile) = current_render_tiles.first() {
                        if is_player {
                            gizmos.line_2d(
                                current_render_tile.y_sort_boundaries[0],
                                current_render_tile.y_sort_boundaries[1],
                                Color::linear_rgb(1., 0., 0.),
                            );
                            gizmos.line_2d(
                                current_render_tile.y_sort_boundaries[1],
                                current_render_tile.y_sort_boundaries[2],
                                Color::linear_rgb(1., 0., 0.),
                            );
                        }

                        let tile_y_sort_boundary_at_x =
                            current_render_tile.get_y_sort_boundary_at_x(iso_coord.x);

                        let distance_to_tile_y_coundary =
                            (tile_y_sort_boundary_at_x - iso_coord.y).abs();

                        // If the current render tile have a greater Y than the current entity
                        // And the distance from it is less than 24
                        if tile_y_sort_boundary_at_x > iso_coord.y
                            && distance_to_tile_y_coundary < 24.
                        {
                            y_offset = -(24. - distance_to_tile_y_coundary);

                            if is_player {
                                gizmos.line_2d(
                                    Vec2::new(iso_coord.x, iso_coord.y),
                                    Vec2::new(iso_coord.x, iso_coord.y + y_offset),
                                    Color::linear_rgb(0., 0., 1.),
                                );
                            }
                        };
                    }
                } else {
                    y_offset = 16.
                }

                iso_coord.z = 1. + (1. - ((iso_coord.y + y_offset) / (map.map_px_size.y)));

                // Update y based on IsoZ to make entity "fly"
                // Is important to do it after the z computation, because we want to compute z based on the initial y
                // which is the projected y on the floor and actual y sort reference
                iso_coord.y += opt_izo_z.map_or(0., |iso_z| iso_z.0 * PIXEL_METER);

                iso_coord
            }
            RenderMode::Cart => Vec3::new(position.x, position.y, transform.translation.z),
        };
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
