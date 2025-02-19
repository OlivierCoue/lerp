use avian2d::prelude::*;
use avian2d::sync::SyncPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use lightyear::prelude::client::is_in_rollback;

use crate::prelude::*;

/// Number of pixels per one meter
pub const PIXEL_METER: f32 = 32.;

pub const NAV_TILE_SIZE: f32 = PIXEL_METER / 2.;
pub const RENDER_TO_NAV_TILE_MULTI: u32 = 5;
pub const RENDER_TILE_SIZE: f32 = NAV_TILE_SIZE * RENDER_TO_NAV_TILE_MULTI as f32;

/// Diameter of a player collider
pub const PLAYER_SIZE: f32 = 16.;
/// Diameter of an enemy collider
pub const ENEMY_SIZE: f32 = 16.;
/// Diameter of a projectile collider
pub const PROJECTILE_SIZE: f32 = 8.;

pub const PLAYER_BASE_MOVEMENT_SPEED: f32 = 8. * PIXEL_METER;
pub const ENEMY_BASE_MOVEMENT_SPEED: f32 = 5. * PIXEL_METER;
pub const PROJECTILE_BASE_MOVEMENT_SPEED: f32 = 30. * PIXEL_METER;

pub const PLAYER_PICKUP_RADIUS: f32 = PIXEL_METER;

pub const PLAYER_BASE_HEALTH: f32 = 100.;
pub const ENEMY_BASE_HEALTH: f32 = 20.;

pub const PLAYER_BASE_MANA: f32 = 100.;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameSimulationSet {
    ApplyPassiveEffects,
    RegisterInputs,
    TriggerSkills,
    ExcecuteSkills,
    SpawnSkills,
    RegisterHitEvents,
    ConsumeHitEvents,
    Others,
}

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
        app.add_plugins((CharacterControllerPlugin, InputPlugin, LootPlugin));

        app.insert_resource(avian2d::sync::SyncConfig {
            transform_to_position: false,
            position_to_transform: false,
            transform_to_collider_scale: false,
        });
        app.insert_resource(Time::<Fixed>::from_hz(FIXED_TIMESTEP_HZ));
        app.insert_resource(Gravity(Vec2::ZERO));
        app.insert_resource(SkillDb::default());
        app.insert_resource(Map::default());
        app.insert_resource(FlowField::default());

        app.add_event::<HitEvent>();
        app.add_event::<TriggerSkillEvent>();
        app.add_event::<ExcecuteSkillEvent>();

        app.configure_sets(
            FixedUpdate,
            (
                GameSimulationSet::Others,
                GameSimulationSet::ApplyPassiveEffects,
                GameSimulationSet::RegisterInputs,
                GameSimulationSet::TriggerSkills,
                GameSimulationSet::ExcecuteSkills,
                GameSimulationSet::RegisterHitEvents,
                GameSimulationSet::ConsumeHitEvents,
            )
                .chain(),
        );

        app.add_systems(
            FixedUpdate,
            (
                (
                    (update_flow_field.run_if(not(is_in_rollback)),),
                    enemy_movement_behavior,
                )
                    .chain(),
                process_projectile_distance,
            )
                .in_set(GameSimulationSet::Others),
        );

        app.add_systems(
            FixedUpdate,
            (
                mana_regeneration,
                progress_skill_cooldown_timers.run_if(not(is_in_rollback)),
            )
                .in_set(GameSimulationSet::ApplyPassiveEffects),
        );

        app.add_systems(
            FixedUpdate,
            (
                on_trigger_skill_event.run_if(on_event::<TriggerSkillEvent>),
                progress_skill_in_progress_timers,
            )
                .chain()
                .in_set(GameSimulationSet::TriggerSkills),
        );

        app.add_systems(
            FixedUpdate,
            on_execute_skill_projectile_event
                .run_if(on_event::<ExcecuteSkillEvent>)
                .in_set(GameSimulationSet::ExcecuteSkills),
        );

        app.add_systems(
            FixedUpdate,
            (process_projectile_collisions).in_set(GameSimulationSet::RegisterHitEvents),
        );

        app.add_systems(
            FixedUpdate,
            (
                on_hit_event.run_if(on_event::<HitEvent>),
                set_character_local,
                set_character_life_state,
            )
                .chain()
                .in_set(GameSimulationSet::ConsumeHitEvents),
        );
    }
}
