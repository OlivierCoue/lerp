use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_simple_text_input::TextInputPlugin;

use common::*;
use lightyear::*;
use states::auth::*;
use states::lobby::*;
use states::play::*;
use ui::*;
use utils::*;

mod common;
mod lightyear;
mod states;
mod ui;
mod utils;

fn setup() {
    println!("Setup!")
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
        .add_plugins((
            // Deps
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Immediate, // Disable VSync and keep high FPS
                        focused: true,

                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,bevy_time=war".to_string(),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin,
            TextInputPlugin,
            TilemapPlugin,
            ShapePlugin,
            // Internal
            UtilsPlugin,
            UiPlugin,
            SetupPlugin,
            LightyearPlugin,
            AuthPlugin,
            LobbyPlugin,
            PlayPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RenderConfig {
            mode: RenderMode::Iso,
        })
        .insert_resource(DebugConfig {
            show_colliders: true,
            show_confirmed_entities: true,
            show_flow_field: false,
        })
        .run();
}
