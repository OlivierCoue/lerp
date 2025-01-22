use crate::common::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct LobbySceneTag;

#[derive(Component)]
enum ButtonAction {
    Play(RenderMode),
    Logout,
    ToggleDebugShowCollider,
    ToggleDebugShowConfirmed,
    ToggleDebugShowFlowField,
}

pub fn lobby_scene_setup(mut commands: Commands, debug_config: Res<DebugConfig>) {
    println!("[lobby_scene_setup]");

    commands.spawn((LobbySceneTag, Camera2d));
    commands.spawn((LobbySceneTag, Text("Lobby Scene".to_string())));
    commands
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
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonAction::Play(RenderMode::Iso),
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
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonAction::Play(RenderMode::Cart),
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
                    parent.spawn(Text("Play Debug".to_string()));
                });
        })
        .with_children(|parent| {
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
        })
        .with_children(|parent| {
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
                    parent.spawn((
                        Text("Show collider".to_string()),
                        TextFont::from_font_size(12.),
                    ));
                    parent.spawn((
                        ButtonAction::ToggleDebugShowCollider,
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
                            checked: debug_config.show_colliders,
                        },
                    ));
                });
        })
        .with_children(|parent| {
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
                    parent.spawn((
                        Text("Show confirmed".to_string()),
                        TextFont::from_font_size(12.),
                    ));
                    parent.spawn((
                        ButtonAction::ToggleDebugShowConfirmed,
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
                            checked: debug_config.show_confirmed_entities,
                        },
                    ));
                });
        })
        .with_children(|parent| {
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
                    parent.spawn((
                        Text("Show flow field".to_string()),
                        TextFont::from_font_size(12.),
                    ));
                    parent.spawn((
                        ButtonAction::ToggleDebugShowFlowField,
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
                            checked: debug_config.show_flow_field,
                        },
                    ));
                });
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
    mut app_state: ResMut<NextState<AppState>>,
    mut debug_config: ResMut<DebugConfig>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, Option<&mut Checkbox>),
        (Changed<Interaction>, With<Button>),
    >,
    mut render_config: ResMut<RenderConfig>,
) {
    for (interaction, action, checkbox) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match action {
                ButtonAction::Play(render_mode) => {
                    render_config.mode = *render_mode;
                    app_state.set(AppState::Play);
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
