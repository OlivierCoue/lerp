use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::projectile::Projectile;

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

// Channels

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
pub struct Player(pub ClientId);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Enemy;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MovementTargets(pub Vec<Vec2>);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MovementSpeed(pub f32);

// Inputs
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InputVec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum PlayerActions {
    Move,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    SkillSlot1,
    SkillSlot2,
    #[actionlike(DualAxis)]
    Cursor,
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

        app.register_component::<Player>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<Enemy>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<Projectile>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<MovementSpeed>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<MovementTargets>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_should_rollback(linear_velocity_should_rollback);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_should_rollback(position_should_rollback);
        // Channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
