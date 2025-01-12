use avian2d::prelude::*;
use bevy::prelude::*;

use crate::common::*;

#[allow(unused_mut)]
pub fn sync_position_to_transform(
    mut query: Query<(&Position, &mut Transform), Or<(Added<Position>, Changed<Position>)>>,
    render_config: Res<RenderConfig>,
) {
    query.par_iter_mut().for_each(|(position, mut transform)| {
        transform.translation = match render_config.mode {
            RenderMode::Iso => {
                cartesian_to_isometric(position.x, position.y).extend(transform.translation.z)
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
