use std::time::Duration;

use avian2d::prelude::*;
use avian2d::sync::SyncPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;

use lightyear::prelude::*;
use lightyear::shared::config::Mode;

use crate::protocol::*;
use crate::settings::FIXED_TIMESTEP_HZ;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    Main,
    // apply physics steps
    Physics,
}

#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);

        // Physics
        //
        // we use Position and Rotation as primary source of truth, so no need to sync changes
        // from Transform->Pos, just Pos->Transform.
        app.insert_resource(avian2d::sync::SyncConfig {
            transform_to_position: false,
            position_to_transform: false,
        });
        // We change SyncPlugin to PostUpdate, because we want the visually interpreted values
        // synced to transform every time, not just when Fixed schedule runs.
        app.add_plugins(
            PhysicsPlugins::new(FixedUpdate)
                .build()
                .disable::<SyncPlugin>(),
        );
        // .add_plugins(SyncPlugin::new(PostUpdate));

        app.insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)));
        app.insert_resource(Gravity(Vec2::ZERO));

        app.configure_sets(
            FixedUpdate,
            (
                // make sure that any physics simulation happens after the Main SystemSet
                // (where we apply user's actions)
                (
                    PhysicsSet::Prepare,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Sync,
                )
                    .in_set(FixedSet::Physics),
                (FixedSet::Main, FixedSet::Physics).chain(),
            ),
        );
    }
}

pub fn shared_movement_behaviour(
    time: &Res<Time<Physics>>,
    position: &Position,
    mut velocity: Mut<LinearVelocity>,
    mut targets: Mut<Targets>,
) {
    let speed = 500.0;

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
            let max_distance = speed * time.delta_seconds();

            // If the next step overshoots the target, use reduced velocity
            if max_distance > distance_to_target {
                println!(
                    "Close to target, max_distance: {} distance_to_target: {}",
                    max_distance, distance_to_target
                );
                *velocity = LinearVelocity(direction * (distance_to_target / time.delta_seconds()));
            // Else go at max speed
            } else {
                *velocity = LinearVelocity((direction * speed).clamp_length_max(speed))
            }
        }
    }
}
