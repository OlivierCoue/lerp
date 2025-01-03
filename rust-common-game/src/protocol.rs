use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lightyear::{
    client::components::ComponentSyncMode,
    utils::avian2d::{linear_velocity, position},
};
use serde::{Deserialize, Serialize};

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

// Components

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player {
    pub client_id: ClientId,
}

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
    Stop,
    #[actionlike(DualAxis)]
    Cursor,
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Inputs
        app.add_plugins(LeafwingInputPlugin::<PlayerActions>::default());

        // Components
        app.register_component::<Player>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<Enemy>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<MovementSpeed>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<MovementTargets>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_correction_fn(position::lerp);
    }
}
