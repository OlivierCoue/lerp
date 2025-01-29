mod animation;
mod camera;
mod debug;
mod direction;
mod enemy;
pub mod map;
mod name_plate;
mod player;
mod projectile;

use crate::common::*;
use crate::states::play::camera::*;
use crate::states::play::debug::*;
use crate::states::play::map::*;
use crate::states::play::player::*;
use crate::NORMAL_BUTTON;

use animation::animate_sprite;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use bevy::render::camera::ScalingMode;
use bevy_transform_interpolation::TransformEasingSet;

use direction::update_direction;
use enemy::handle_new_enemy;
use flow_field::debug_render_flow_field;
use leafwing_input_manager::plugin::InputManagerSystem;
use lightyear::client::input::leafwing::InputSystemSet;
use lightyear::prelude::client::*;
use name_plate::handle_new_name_plate;
use name_plate::update_health_bar;
use name_plate::update_mana_bar;
use name_plate::update_skill_in_progress_bar;
use projectile::handle_new_projectile;
use projectile::handle_removed_projectile;

use rust_common_game::map::generate_map;
use rust_common_game::protocol::Channel1;
use rust_common_game::protocol::PlayerClient;
use rust_common_game::protocol::SpawnEnemies;
use rust_common_game::utils::CommonPlaySceneTag;

#[derive(Component)]
pub struct PlayerCamera;

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

#[derive(Component)]
enum ButtonAction {
    SpawnEnemies,
    CameraZoomIn,
    CameraZoomOut,
}

pub fn play_scene_setup(mut commands: Commands) {
    println!("[play_scene_setup]");

    commands.connect_client();
    commands.spawn((
        PlaySceneTag,
        PlayerCamera,
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 1280.,
                max_height: 720.,
            },
            ..OrthographicProjection::default_2d()
        },
    ));

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

    commands
        .spawn((
            PlaySceneTag,
            Node {
                width: Val::Percent(100.),
                align_items: AlignItems::End,
                justify_content: JustifyContent::End,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonAction::SpawnEnemies,
                    Button,
                    BorderColor(Color::BLACK),
                    BackgroundColor(NORMAL_BUTTON),
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((Text("Enemies".to_string()), TextFont::from_font_size(12.)));
                });
            parent
                .spawn((
                    ButtonAction::CameraZoomIn,
                    Button,
                    BorderColor(Color::BLACK),
                    BackgroundColor(NORMAL_BUTTON),
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((Text("Zoom +".to_string()), TextFont::from_font_size(12.)));
                });
            parent
                .spawn((
                    ButtonAction::CameraZoomOut,
                    Button,
                    BorderColor(Color::BLACK),
                    BackgroundColor(NORMAL_BUTTON),
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((Text("Zoom -".to_string()), TextFont::from_font_size(12.)));
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

fn play_scene_button_logic(
    mut connection_manager: ResMut<ConnectionManager>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut camera_query: Query<&mut OrthographicProjection, With<PlayerCamera>>,
) {
    for (interaction, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match action {
                ButtonAction::SpawnEnemies => {
                    let _ = connection_manager.send_message::<Channel1, _>(&SpawnEnemies);
                }
                ButtonAction::CameraZoomIn => {
                    if let Ok(mut ortho_proj) = camera_query.get_single_mut() {
                        ortho_proj.scale = (ortho_proj.scale - 0.5).max(1.)
                    };
                }
                ButtonAction::CameraZoomOut => {
                    if let Ok(mut ortho_proj) = camera_query.get_single_mut() {
                        ortho_proj.scale = (ortho_proj.scale + 0.5).min(5.)
                    };
                }
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn play_scene_cleanup(
    mut commands: Commands,
    query: Query<Entity, Or<(With<PlaySceneTag>, With<CommonPlaySceneTag>)>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    println!("[play_scene_cleanup]");
    commands.disconnect_client();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    chunk_manager.spawned_chunks.clear();
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
        app.insert_resource(ChunkManager::default());
        app.add_systems(
            OnEnter(AppState::Play),
            (play_scene_setup, (generate_map, render_flow_field).chain()),
        );
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
                play_scene_button_logic,
                handle_new_client,
                handle_new_player,
                handle_new_enemy,
                handle_new_projectile,
                handle_removed_projectile,
                handle_new_name_plate,
                update_health_bar,
                update_mana_bar,
                update_skill_in_progress_bar,
                (update_direction, animate_sprite).chain(),
                debug_draw_confirmed_entities,
                debug_undraw_confirmed_entities,
                debug_draw_colliders,
                debug_undraw_colliders,
                update_fps,
                update_ping,
                spawn_map_chunks_around_camera,
                despawn_outofrange_map_chunks,
            )
                .run_if(in_state(AppState::Play)),
        );
        app.add_systems(
            Update,
            (camera_draw_border).run_if(in_state(AppState::Play)),
        );

        app.add_systems(
            FixedUpdate,
            (update_rollback_state, debug_render_flow_field).run_if(in_state(AppState::Play)),
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
