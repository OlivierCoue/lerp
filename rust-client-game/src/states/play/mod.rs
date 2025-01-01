mod camera;
mod debug;
mod map;
mod player;

use crate::common::*;
use crate::states::play::camera::*;
use crate::states::play::debug::*;
use crate::states::play::map::*;
use crate::states::play::player::*;

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use bevy_transform_interpolation::TransformEasingSet;

use leafwing_input_manager::plugin::InputManagerSystem;
use lightyear::prelude::client::*;
use rust_common_game::settings::*;

#[derive(Component)]
pub struct PlaySceneTag;

#[derive(Component)]
pub struct FpsDisplayTag;

pub fn play_scene_setup(mut commands: Commands) {
    println!("[play_scene_setup]");

    commands.connect_client();
    commands.spawn((PlaySceneTag, Camera2d));

    commands
        .spawn((
            PlaySceneTag,
            Node {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(Text("Play Scene".to_string()));
            parent.spawn((
                FpsDisplayTag,
                Text("FPS: 0".to_string()),
                TextColor(Color::linear_rgb(0., 1., 0.)),
            ));
        });
}

pub fn play_scene_logic(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Lobby);
    }
}

pub fn play_scene_cleanup(mut commands: Commands, query: Query<Entity, With<PlaySceneTag>>) {
    println!("[play_scene_cleanup]");
    commands.disconnect_client();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsDisplayTag>>,
) {
    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
    {
        for mut text in &mut query {
            text.0 = format!("FPS: {:.1}", fps);
        }
    }
}

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Play), (play_scene_setup, setup_map));
        app.add_systems(OnExit(AppState::Play), play_scene_cleanup);

        app.add_systems(
            PreUpdate,
            sync_cursor_poisition
                .in_set(InputManagerSystem::ManualControl)
                .run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            Update,
            (
                play_scene_logic,
                handle_new_player,
                animate_sprite,
                debug_draw_confirmed_entities,
                // draw_predicted_target,
                debug_draw_colliders,
                update_fps,
            )
                .run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            FixedUpdate,
            (movement, set_player_target)
                .chain()
                .run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            PostUpdate,
            camera_follow
                .before(TransformSystem::TransformPropagate)
                .after(TransformEasingSet::UpdateEasingTick)
                .run_if(in_state(AppState::Play)),
        );
    }
}
