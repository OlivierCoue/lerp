use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::shared::PIXEL_METER;

use crate::common::*;

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
