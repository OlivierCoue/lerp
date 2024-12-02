use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::client::components::ComponentSyncMode;

use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

// Components

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player {
    pub client_id: ClientId,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Targets(pub Vec<Vec2>);

// Inputs
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InputVec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Inputs {
    Target(InputVec2),
    Delete,
    Spawn,
    /// NOTE: we NEED to provide a None input so that the server can distinguish between lost input packets and 'None' inputs
    None,
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Inputs
        app.add_plugins(InputPlugin::<Inputs>::default());

        // Components
        app.register_component::<Player>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<Targets>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);
    }
}
