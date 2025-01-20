use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, PreSpawnedPlayerObject};

use super::{apply_render_mode, RenderConfig};

#[derive(Component)]
pub struct Direction(pub usize);

pub fn update_direction(
    mut commands: Commands,
    render_config: Res<RenderConfig>,
    mut q: Query<
        (Entity, &LinearVelocity, Option<&mut Direction>),
        Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>,
    >,
) {
    for (entity, linear_velocity, current_direction) in &mut q {
        let renderered_velocity = apply_render_mode(&render_config, &linear_velocity.0);

        if renderered_velocity.length_squared() != 0.0 {
            // Calculate the angle in radians and normalize to [0, 2π]
            let angle = renderered_velocity.y.atan2(renderered_velocity.x); // atan2(y, x) gives the angle relative to the X-axis
            let mut angle_deg = angle.to_degrees(); // Convert to degrees
            angle_deg = (angle_deg + 180.0).rem_euclid(360.0); // Normalize negative angles to [0, 360]

            let adjusted_angle = 360. - ((angle_deg + 270.) % 360.0);

            // Map the adjusted angle to one of 16 directions
            let sector_size = 360.0 / 16.0; // Each direction covers 22.5 degrees
            let direction_index =
                ((adjusted_angle + (sector_size / 2.0)) / sector_size).floor() as usize % 16;

            if let Some(mut current_direction) = current_direction {
                current_direction.0 = direction_index;
            } else {
                commands.entity(entity).insert(Direction(direction_index));
            }
        } else if current_direction.is_none() {
            commands.entity(entity).insert(Direction(0));
        }
    }
}
