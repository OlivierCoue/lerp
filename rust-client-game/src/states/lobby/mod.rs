use std::net::IpAddr;
use std::str::FromStr;

use crate::lightyear::get_client_net_config;
use crate::ui::text_input::create_text_input;
use crate::ui::*;
use crate::{common::*, network::tokio_task::TokioTasksRuntime};
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy_simple_text_input::TextInputValue;
use lightyear::client::config::ClientConfig;
use rust_common_game::http_api::HttpStartServerResponse;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct LobbySceneTag;

#[derive(Component)]
enum ButtonAction {
    Play,
    Logout,
    ToggleDebugShowCollider,
    ToggleDebugShowConfirmed,
    ToggleDebugShowFlowField,
    ToggleDebugShowYSortBoundaries,
}

#[derive(Component)]
struct TextInputServerAddress;

pub fn lobby_scene_setup(mut commands: Commands, debug_config: Res<DebugConfig>) {
    println!("[lobby_scene_setup]");

    commands.spawn((LobbySceneTag, Camera2d));
    commands.spawn((LobbySceneTag, Text("Lobby Scene".to_string())));
    let container = commands
        .spawn((
            LobbySceneTag,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .id();

    let text_input_server_address_entity = create_text_input(
        &mut commands,
        TextInputServerAddress,
        "Server Address".to_string(),
        Some("127.0.0.1".to_string()),
    );
    commands
        .entity(container)
        .add_children(&[text_input_server_address_entity]);

    commands.entity(container).with_children(|parent| {
        parent
            .spawn((
                ButtonAction::Play,
                Button,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(Text("Play".to_string()));
            });

        parent
            .spawn((
                ButtonAction::Logout,
                Button,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(Text("Logout".to_string()));
            });

        add_debug_option_checkbox(
            parent,
            "Show collider",
            ButtonAction::ToggleDebugShowCollider,
            debug_config.show_colliders,
        );
        add_debug_option_checkbox(
            parent,
            "Show confirmed",
            ButtonAction::ToggleDebugShowConfirmed,
            debug_config.show_confirmed_entities,
        );
        add_debug_option_checkbox(
            parent,
            "Show flow field",
            ButtonAction::ToggleDebugShowFlowField,
            debug_config.show_flow_field,
        );
        add_debug_option_checkbox(
            parent,
            "Show Y sort boundaries",
            ButtonAction::ToggleDebugShowYSortBoundaries,
            debug_config.show_y_sort_boundaries,
        );
    });
}

fn add_debug_option_checkbox(
    parent: &mut ChildBuilder,
    title: &str,
    button_action: ButtonAction,
    default_state: bool,
) {
    parent
        .spawn(Node {
            width: Val::Px(150.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((Text(title.to_string()), TextFont::from_font_size(12.)));
            parent.spawn((
                button_action,
                Button,
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                BorderColor(Color::BLACK),
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                Checkbox {
                    checked: default_state,
                },
            ));
        });
}

pub fn lobby_scene_logic(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Auth);
    }
}

fn lobby_scene_button_logic(
    tokio_runtime: ResMut<TokioTasksRuntime>,
    mut lightyear_client_config: ResMut<ClientConfig>,
    mut app_state: ResMut<NextState<AppState>>,
    mut debug_config: ResMut<DebugConfig>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, Option<&mut Checkbox>),
        (Changed<Interaction>, With<Button>),
    >,
    text_input_server_address_query: Query<&TextInputValue, With<TextInputServerAddress>>,
) {
    for (interaction, action, checkbox) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match action {
                ButtonAction::Play => {
                    let server_address = text_input_server_address_query.get_single().unwrap();
                    let Ok(server_address) = IpAddr::from_str(server_address.0.as_str()) else {
                        println!("Invalid server address");
                        break;
                    };

                    tokio_runtime.spawn_background_task(move |mut ctx| async move {
                        let response = reqwest::Client::new()
                            .post(format!("http://{}:4000/server/start", server_address))
                            .send()
                            .await
                            .unwrap()
                            .json::<HttpStartServerResponse>()
                            .await
                            .unwrap();
                        info!(
                            "Server: uuid: {} port: {}",
                            response.instance_uuid, response.instance_port
                        );

                        ctx.run_on_main_thread(move |ctx| {
                            let mut lightyear_client_config = ctx
                                .world
                                .get_resource_mut::<ClientConfig>()
                                .expect("Lightyear ClientConfig resource not initialized");
                            lightyear_client_config.net =
                                get_client_net_config(server_address, response.instance_port);

                            let mut app_state = ctx
                                .world
                                .get_resource_mut::<NextState<AppState>>()
                                .expect("AppState state not initialized");

                            app_state.set(AppState::Play);
                        })
                        .await;
                    });
                }
                ButtonAction::Logout => {
                    app_state.set(AppState::Auth);
                }
                ButtonAction::ToggleDebugShowCollider => {
                    if let Some(mut checkbox) = checkbox {
                        checkbox.checked = !checkbox.checked;
                        debug_config.show_colliders = checkbox.checked;
                    };
                }
                ButtonAction::ToggleDebugShowConfirmed => {
                    if let Some(mut checkbox) = checkbox {
                        checkbox.checked = !checkbox.checked;
                        debug_config.show_confirmed_entities = checkbox.checked;
                    };
                }
                ButtonAction::ToggleDebugShowFlowField => {
                    if let Some(mut checkbox) = checkbox {
                        checkbox.checked = !checkbox.checked;
                        debug_config.show_flow_field = checkbox.checked;
                    };
                }
                ButtonAction::ToggleDebugShowYSortBoundaries => {
                    if let Some(mut checkbox) = checkbox {
                        checkbox.checked = !checkbox.checked;
                        debug_config.show_y_sort_boundaries = checkbox.checked;
                    };
                }
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn lobby_scene_cleanup(mut commands: Commands, query: Query<Entity, With<LobbySceneTag>>) {
    println!("[lobby_scene_cleanup]");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Lobby), lobby_scene_setup);
        app.add_systems(Update, lobby_scene_logic.run_if(in_state(AppState::Lobby)));
        app.add_systems(
            Update,
            lobby_scene_button_logic.run_if(in_state(AppState::Lobby)),
        );
        app.add_systems(OnExit(AppState::Lobby), lobby_scene_cleanup);
    }
}
