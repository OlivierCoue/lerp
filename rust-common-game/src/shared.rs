use avian2d::prelude::*;
use avian2d::sync::SyncPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;

use crate::character_controller::CharacterControllerPlugin;

use crate::enemy::enemy_movement_behavior;
use crate::hit::HitEvent;
use crate::input::{handle_input_move_wasd, handle_input_skill_slot};
use crate::projectile::{
    on_spawn_projectile_event, process_projectile_collisions, process_projectile_distance,
    SpawnProjectileEvent,
};
use crate::protocol::*;
use crate::settings::FIXED_TIMESTEP_HZ;

/// Number of pixels per one meter
pub const PIXEL_METER: f32 = 32.;
/// Diameter of a player collider
pub const PLAYER_SIZE: f32 = 32.;
/// Diameter of an enemy collider
pub const ENEMY_SIZE: f32 = 16.;
/// Diameter of a projectile collider
pub const PROJECTILE_SIZE: f32 = 8.;

pub const PLAYER_BASE_MOVEMENT_SPEED: f32 = 8. * PIXEL_METER;
pub const ENEMY_BASE_MOVEMENT_SPEED: f32 = 5. * PIXEL_METER;
pub const PROJECTILE_BASE_MOVEMENT_SPEED: f32 = 30. * PIXEL_METER;

pub const PLAYER_BASE_HEALTH: f32 = 100.;
pub const ENEMY_BASE_HEALTH: f32 = 20.;

#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);
        app.add_plugins(
            PhysicsPlugins::new(FixedUpdate)
                .with_length_unit(PIXEL_METER)
                .build()
                .disable::<SyncPlugin>(),
        );
        app.add_plugins(CharacterControllerPlugin);

        app.insert_resource(avian2d::sync::SyncConfig {
            transform_to_position: false,
            position_to_transform: false,
            transform_to_collider_scale: false,
        });
        app.insert_resource(Time::<Fixed>::from_hz(FIXED_TIMESTEP_HZ));
        app.insert_resource(Gravity(Vec2::ZERO));

        app.add_event::<SpawnProjectileEvent>();
        app.add_event::<HitEvent>();

        app.add_systems(
            FixedUpdate,
            (
                handle_input_move_wasd,
                handle_input_skill_slot,
                enemy_movement_behavior,
                process_projectile_distance,
                process_projectile_collisions,
            )
                .chain(),
        );

        app.add_systems(
            FixedUpdate,
            on_spawn_projectile_event
                .run_if(on_event::<SpawnProjectileEvent>)
                .after(handle_input_skill_slot),
        );
    }
}
