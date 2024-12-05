use bevy::prelude::*;
use lightyear::{prelude::client::*, shared::replication::components::Controlled};
use rust_common_game::protocol::*;

// System to make the camera follow the player
#[allow(clippy::type_complexity)]
pub fn camera_follow(
    // time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, With<Predicted>, With<Controlled>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in &mut camera_query {
            // let damping = 40.0; // Higher values = faster snap, lower = smoother

            // // Smoothly interpolate camera position using exponential damping
            // camera_transform.translation.x += (player_transform.translation.x
            //     - camera_transform.translation.x)
            //     * damping
            //     * time.delta_seconds();
            // camera_transform.translation.y += (player_transform.translation.y
            //     - camera_transform.translation.y)
            //     * damping
            //     * time.delta_seconds();

            camera_transform.translation = player_transform.translation;
        }
    }
}
