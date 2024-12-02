mod camera;
mod map;
mod player;
use crate::common::*;
use crate::states::play::camera::*;
use crate::states::play::map::*;
use crate::states::play::player::*;

use bevy::prelude::*;

use bevy_transform_interpolation::TransformEasingSet;

use lightyear::client::input::native::InputSystemSet;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use rust_common_game::settings::*;
use rust_common_game::shared::*;

#[derive(Component)]
pub struct PlaySceneTag;

pub fn play_scene_setup(mut commands: Commands) {
    println!("[play_scene_setup]");

    commands.connect_client();
    commands.spawn((PlaySceneTag, Camera2dBundle::default()));
    commands.spawn((
        PlaySceneTag,
        TextBundle::from_section("Play Scene", TextStyle::default()),
    ));
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

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Play), (play_scene_setup, setup_map));
        app.add_systems(OnExit(AppState::Play), play_scene_cleanup);

        app.add_systems(
            FixedPreUpdate,
            buffer_input.in_set(InputSystemSet::BufferInputs),
        );

        app.add_systems(
            Update,
            (
                play_scene_logic,
                handle_new_player,
                draw_confirmed_player,
                draw_predicted_target,
            )
                .run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            FixedUpdate,
            (movement, set_player_target)
                .chain()
                .in_set(FixedSet::Main)
                .run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            PostUpdate,
            camera_follow
                .before(TransformSystem::TransformPropagate)
                .after(TransformEasingSet)
                .run_if(in_state(AppState::Play)),
        );
    }
}
