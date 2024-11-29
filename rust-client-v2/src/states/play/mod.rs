mod map;
mod player;
use crate::common::*;
use crate::states::play::map::*;
use crate::states::play::player::*;
use avian2d::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct PlaySceneTag;

pub fn play_scene_setup(mut commands: Commands) {
    println!("[play_scene_setup]");

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

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Play), play_scene_setup);
        app.add_systems(Update, play_scene_logic.run_if(in_state(AppState::Play)));
        app.add_systems(OnExit(AppState::Play), play_scene_cleanup);

        app.add_systems(OnEnter(AppState::Play), setup_map);
        app.add_systems(OnEnter(AppState::Play), setup_player);
        app.add_systems(
            PostUpdate,
            sync_transform_physics_with_render
                .after(PhysicsSet::Sync)
                .before(camera_follow)
                .run_if(in_state(AppState::Play)),
        );
        app.add_systems(
            PostUpdate,
            camera_follow
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate)
                .run_if(in_state(AppState::Play)),
        );
        app.add_systems(Update, movement.run_if(in_state(AppState::Play)));
        app.add_systems(Update, capture_world_click.run_if(in_state(AppState::Play)));
        app.add_systems(Update, set_player_target.run_if(in_state(AppState::Play)));

        app.add_event::<LeftClickEvent>();
    }
}
