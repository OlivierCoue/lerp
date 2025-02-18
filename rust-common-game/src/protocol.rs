use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);
pub const LOOT_REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(2);

// Channels

#[derive(Channel)]
pub struct Channel1;

// Messages

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SpawnEnemies;

// Components

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerClient {
    pub client_id: ClientId,
    pub rtt: Duration,
    pub jitter: Duration,
    pub player_ref: Entity,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MovementTargets(pub Vec<Vec2>);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MovementSpeed(pub f32);

pub fn health_should_rollback(this: &Health, that: &Health) -> bool {
    if this.ne(that) {
        println!("rollback: health");
        return true;
    }
    false
}

pub fn mana_should_rollback(this: &Mana, that: &Mana) -> bool {
    if this.ne(that) {
        println!("rollback: mana");
        return true;
    }
    false
}

pub fn dead_should_rollback(this: &Dead, that: &Dead) -> bool {
    if this.ne(that) {
        println!("rollback: dead");
        return true;
    }
    false
}

pub fn position_should_rollback(this: &Position, that: &Position) -> bool {
    if this.ne(that) {
        println!("rollback: position");
        return true;
    }
    false
}

pub fn linear_velocity_should_rollback(this: &LinearVelocity, that: &LinearVelocity) -> bool {
    if this.ne(that) {
        println!("rollback: linear velocity");
        return true;
    }
    false
}

pub fn angular_velocity_should_rollback(this: &AngularVelocity, that: &AngularVelocity) -> bool {
    if this.ne(that) {
        println!("rollback: angular velocity");
        return true;
    }
    false
}

pub fn linear_velocity_correction(
    _: &LinearVelocity,
    other: &LinearVelocity,
    _: f32,
) -> LinearVelocity {
    *other
}

pub fn position_correction(_: &Position, other: &Position, _: f32) -> Position {
    *other
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Inputs
        app.add_plugins(LeafwingInputPlugin::<PlayerActions>::default());
        // Messages
        app.register_message::<SpawnEnemies>(ChannelDirection::ClientToServer);
        // Components
        app.register_component::<PlayerClient>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Simple);

        app.register_component::<Character>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<Projectile>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<SkillSlotMap>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<MovementSpeed>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<Health>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_should_rollback(health_should_rollback);

        app.register_component::<Mana>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_should_rollback(mana_should_rollback);

        app.register_component::<Dead>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_should_rollback(dead_should_rollback);

        app.register_component::<MovementTargets>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_should_rollback(linear_velocity_should_rollback);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_should_rollback(angular_velocity_should_rollback);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_should_rollback(position_should_rollback);

        // Server driven components
        app.register_component::<Loot>(ChannelDirection::ServerToClient);

        // Channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
