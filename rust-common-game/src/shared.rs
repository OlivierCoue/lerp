use avian2d::prelude::*;
use avian2d::sync::SyncPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;

use crate::character_controller::CharacterControllerPlugin;
use crate::protocol::*;
use crate::settings::FIXED_TIMESTEP_HZ;

#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);

        // Physics
        //
        // we use Position and Rotation as primary source of truth, so no need to sync changes with SyncPlugin
        app.add_plugins(
            PhysicsPlugins::new(FixedUpdate)
                .build()
                .disable::<SyncPlugin>(),
        );
        app.insert_resource(avian2d::sync::SyncConfig {
            transform_to_position: false,
            position_to_transform: false,
            transform_to_collider_scale: false,
        });
        app.insert_resource(Time::<Fixed>::from_hz(FIXED_TIMESTEP_HZ));
        app.insert_resource(Gravity(Vec2::ZERO));

        app.add_plugins(CharacterControllerPlugin);
    }
}

pub fn shared_movement_behaviour(
    time: &Res<Time<Physics>>,
    position: &Position,
    movement_speed: &MovementSpeed,
    mut velocity: Mut<LinearVelocity>,
    mut targets: Mut<Targets>,
) {
    if let Some(target) = targets.0.first() {
        let to_target: Vec2 = *target - position.0;
        let distance_to_target = to_target.length();

        // If close enough to the target, stop movement
        if distance_to_target <= 1e-4 {
            velocity.0 = Vec2::ZERO;
            targets.0.clear();
        } else {
            // Calculate direction to the target
            let direction = to_target.normalize_or_zero();
            // Compute movement distance based on speed and delta time
            let max_distance = movement_speed.0 * time.delta_secs();

            // If the next step overshoots the target, use reduced velocity
            if max_distance > distance_to_target {
                *velocity = LinearVelocity(direction * (distance_to_target / time.delta_secs()));
            // Else go at max speed
            } else {
                *velocity = LinearVelocity(
                    (direction * movement_speed.0).clamp_length_max(movement_speed.0),
                )
            }
        }
    }
}
