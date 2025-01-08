mod animation;
mod camera;
mod debug;
mod enemy;
mod map;
mod player;
mod projectile;

use crate::common::*;
use crate::states::play::camera::*;
use crate::states::play::debug::*;
use crate::states::play::map::*;
use crate::states::play::player::*;

use animation::animate_sprite;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use bevy_transform_interpolation::TransformEasingSet;

use enemy::handle_new_enemy;
use leafwing_input_manager::plugin::InputManagerSystem;
use lightyear::client::input::leafwing::InputSystemSet;
use lightyear::prelude::client::*;
use projectile::handle_new_projectile;
use rust_common_game::protocol::PlayerClient;

#[derive(Component, Default)]
pub struct PlaySceneTag;

#[derive(Component)]
pub struct FpsDisplayTag;

#[derive(Component)]
pub struct PingDisplayTag;

#[derive(Component)]
pub struct RollBackHistoric(pub [bool; 100]);

#[derive(Component)]
pub struct RollbackStateLineItem;

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
            parent.spawn((
                PingDisplayTag,
                Text("PING: 0".to_string()),
                TextColor(Color::linear_rgb(0., 1., 0.)),
            ));
            parent
                .spawn((
                    RollBackHistoric([false; 100]),
                    Node {
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    for _ in 0..100 {
                        parent.spawn((
                            RollbackStateLineItem,
                            Node {
                                width: Val::Px(3.0),
                                height: Val::Px(10.0),
                                ..default()
                            },
                            BackgroundColor(Color::linear_rgb(0., 1., 0.)),
                        ));
                    }
                });
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

pub fn update_ping(
    query_client: Query<&PlayerClient, (With<Predicted>, Without<PingDisplayTag>)>,
    mut query_text: Query<&mut Text, With<PingDisplayTag>>,
) {
    for mut text in &mut query_text {
        for client in &query_client {
            text.0 = format!("PING: {}", client.rtt.as_millis());
        }
    }
}

pub fn update_rollback_state(
    rollback: Res<Rollback>,
    mut query_container: Query<(&mut RollBackHistoric, &Children), Without<RollbackStateLineItem>>,
    mut query_line_item: Query<&mut BackgroundColor, With<RollbackStateLineItem>>,
) {
    for (mut rollback_historic, children) in &mut query_container {
        for i in 0..99 {
            rollback_historic.0[i] = rollback_historic.0[i + 1];
        }
        rollback_historic.0[99] = rollback.is_rollback();

        for (i, &is_rollback) in rollback_historic.0.iter().enumerate() {
            let child_id = children.get(i).unwrap();
            let mut background_color = query_line_item.get_mut(*child_id).unwrap();
            background_color.0 = if is_rollback {
                Color::linear_rgb(1., 0., 0.)
            } else {
                Color::srgb(0.15, 0.15, 0.15)
            };
        }
    }
}

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Play), (play_scene_setup, setup_map));
        app.add_systems(OnExit(AppState::Play), play_scene_cleanup);

        app.add_systems(
            FixedPreUpdate,
            sync_cursor_poisition
                .before(InputSystemSet::BufferClientInputs)
                .in_set(InputManagerSystem::ManualControl)
                .run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            Update,
            (
                play_scene_logic,
                handle_new_client,
                handle_new_player,
                handle_new_enemy,
                handle_new_projectile,
                animate_sprite,
                debug_draw_confirmed_entities,
                debug_undraw_confirmed_entities,
                debug_draw_colliders,
                update_fps,
                update_ping,
            )
                .run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            FixedUpdate,
            (update_rollback_state).run_if(in_state(AppState::Play)),
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
