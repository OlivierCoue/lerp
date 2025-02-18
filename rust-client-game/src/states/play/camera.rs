use bevy::prelude::*;
use bevy_transform_interpolation::TransformEasingSet;
use lightyear::{prelude::client::*, shared::replication::components::Controlled};
use rust_common_game::prelude::*;

use crate::common::AppState;

#[derive(Component)]
pub struct PlayerCamera;

pub const CAMERA_VIEWPORT_SIZE: Vec2 = Vec2::new(1280., 720.);

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 20.;

fn camera_draw_border(
    mut gizmos: Gizmos,
    camera_query: Query<(&Transform, &OrthographicProjection), With<PlayerCamera>>,
) {
    for (transform, ortho_proj) in camera_query.iter() {
        if ortho_proj.scale != 1. {
            gizmos.rect_2d(
                transform.translation.xy(),
                CAMERA_VIEWPORT_SIZE,
                Color::linear_rgb(1., 0., 0.),
            );
        }
    }
}
// System to make the camera follow the player
fn camera_follow(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, With<Predicted>, With<Controlled>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<PlayerCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in &mut camera_query {
            let Vec3 { x, y, .. } = player_transform.translation;
            let direction = Vec3::new(x, y, camera_transform.translation.z);

            // Applies a smooth effect to camera movement using stable interpolation
            // between the camera position and the player position on the x and y axes.
            camera_transform.translation.smooth_nudge(
                &direction,
                CAMERA_DECAY_RATE,
                time.delta_secs(),
            );
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (camera_draw_border).run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            PostUpdate,
            camera_follow
                .before(TransformSystem::TransformPropagate)
                .after(TransformEasingSet::UpdateEasingTick)
                .run_if(in_state(AppState::Play)),
        );
    }
}
