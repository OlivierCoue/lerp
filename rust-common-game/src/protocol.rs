use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lightyear::{
    client::components::ComponentSyncMode, utils::avian2d::angular_velocity,
    utils::avian2d::linear_velocity, utils::avian2d::position,
};
use serde::{Deserialize, Serialize};

use crate::projectile::Projectile;

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

// Components

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerClient {
    pub client_id: ClientId,
    pub rtt: Duration,
    pub jitter: Duration,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player;

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
    Stop,
    #[actionlike(DualAxis)]
    Cursor,
}

pub fn position_should_rollback(this: &Position, that: &Position) -> bool {
    let distance = this.distance(that.0);
    distance > 1.
}

pub fn velocity_should_rollback(this: &LinearVelocity, that: &LinearVelocity) -> bool {
    let distance = this.distance(that.0);
    distance > 500.
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Inputs
        app.add_plugins(LeafwingInputPlugin::<PlayerActions>::default());

        // Components
        app.register_component::<PlayerClient>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Simple);

        app.register_component::<Player>(ChannelDirection::ServerToClient)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<Enemy>(ChannelDirection::ServerToClient)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<Projectile>(ChannelDirection::ServerToClient)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<MovementSpeed>(ChannelDirection::ServerToClient)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<MovementTargets>(ChannelDirection::ServerToClient)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(linear_velocity::lerp);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(angular_velocity::lerp);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp);
    }
}
