use bevy::prelude::*;
use lightyear::{prelude::client::*, shared::replication::components::Controlled};
use rust_common_game::protocol::*;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 20.;

// System to make the camera follow the player
pub fn camera_follow(
    time: Res<Time>,
    player_query: Query<&Transform, (With<PlayerDTO>, With<Predicted>, With<Controlled>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<PlayerDTO>)>,
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
