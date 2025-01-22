use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::{map::MAP_SIZE, shared::PIXEL_METER};

use crate::{common::*, map::MAP_TILE_IMG_SIZE_WALL};

#[derive(Component)]
pub struct IsoZ(pub f32);

#[allow(unused_mut)]
pub fn sync_position_to_transform(
    mut query: Query<
        (&Position, &mut Transform, Option<&IsoZ>),
        Or<(Added<Position>, Changed<Position>)>,
    >,
    render_config: Res<RenderConfig>,
) {
    query
        .par_iter_mut()
        .for_each(|(position, mut transform, opt_izo_z)| {
            transform.translation = match render_config.mode {
                RenderMode::Iso => {
                    let mut coord = cartesian_to_isometric(position.x, position.y)
                        .extend(transform.translation.z);

                    // Aply the same y sorting as the one in ECS TileMap lib:
                    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/8ccbd6bff829643a9c6f20ad25367234179886b9/src/render/material.rs#L483
                    coord.z =
                        1. + (1. - ((coord.y) / (MAP_SIZE.y as f32 * MAP_TILE_IMG_SIZE_WALL.y)));

                    // Update y base on IsoZ to make element "fly"
                    // Is important to do it after the z computation, because we want to compute z based on the initial y
                    // which is the projeted y on the floor and actual y sort reference
                    coord.y += opt_izo_z.map_or(0., |iso_z| iso_z.0 * PIXEL_METER);

                    coord
                }
                RenderMode::Cart => Vec3::new(position.x, position.y, transform.translation.z),
            };
        });
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
