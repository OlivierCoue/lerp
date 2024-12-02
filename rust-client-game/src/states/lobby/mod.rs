use crate::common::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct LobbySceneTag;

#[derive(Component)]
enum ButtonAction {
    Play(RenderMode),
    Logout,
}

pub fn lobby_scene_setup(mut commands: Commands) {
    println!("[lobby_scene_setup]");

    commands.spawn((LobbySceneTag, Camera2dBundle::default()));
    commands.spawn((
        LobbySceneTag,
        TextBundle::from_section("Lobby Scene", TextStyle::default()),
    ));
    commands
        .spawn((
            LobbySceneTag,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonAction::Play(RenderMode::Iso),
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Play", TextStyle::default()));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonAction::Play(RenderMode::Cart),
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Play Debug", TextStyle::default()));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonAction::Logout,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Logout", TextStyle::default()));
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

#[allow(clippy::type_complexity)]
fn lobby_scene_button_logic(
    mut app_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut render_config: ResMut<RenderConfig>,
) {
    for (interaction, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match action {
                ButtonAction::Play(render_mode) => {
                    render_config.mode = *render_mode;
                    app_state.set(AppState::Play);
                }
                ButtonAction::Logout => {
                    app_state.set(AppState::Auth);
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
