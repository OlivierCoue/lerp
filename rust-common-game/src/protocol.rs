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

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Deref)]
pub struct MovementTarget(pub Vec2);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MovementSpeed(pub f32);

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
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Mana>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Dead>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<MovementTarget>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        // Server driven components
        app.register_component::<ItemDropped>(ChannelDirection::ServerToClient);

        // Channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
