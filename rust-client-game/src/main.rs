use bevy::{prelude::*, sprite::Wireframe2dPlugin};
use bevy_simple_text_input::TextInputPlugin;
use common::*;
use states::auth::*;
use states::lobby::*;
use states::play::*;
use ui::*;

mod common;
mod states;
mod ui;

fn setup() {
    println!("Setup")
}

fn transition_to_auth_scene(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Auth);
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();

        app.add_systems(OnEnter(AppState::Setup), setup);
        app.add_systems(
            Update,
            transition_to_auth_scene.run_if(in_state(AppState::Setup)),
        );
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin, TextInputPlugin))
        .add_plugins((UiPlugin, SetupPlugin, AuthPlugin, LobbyPlugin, PlayPlugin))
        .insert_resource(RenderConfig {
            mode: RenderMode::Iso,
        })
        .run();
}
