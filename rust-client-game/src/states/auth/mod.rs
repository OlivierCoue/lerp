use crate::common::*;
use crate::ui::*;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_simple_text_input::*;

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.25, 0.25, 0.25);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_COLOR_PLACEHOLDER: Color = Color::srgb(0.5, 0.5, 0.5);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Component)]
struct AuthSceneTag;

#[derive(Component)]
enum ButtonAction {
    Login,
    Exit,
}

#[derive(Component)]
struct TextInputUsername;

#[derive(Component)]
struct TextInputPassword;

fn auth_scene_setup(mut commands: Commands) {
    println!("[auth_scene_setup]");

    commands.spawn((AuthSceneTag, Camera2d::default()));
    commands.spawn((AuthSceneTag, Text("Authentication Scene".to_string())));
    commands
        .spawn((
            AuthSceneTag,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Interaction::None,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextInputUsername,
                BorderColor(BORDER_COLOR_ACTIVE.into()),
                BackgroundColor(BACKGROUND_COLOR.into()),
                FocusPolicy::Block,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                TextInput,
                TextInputTextFont(TextFont {
                    font_size: 20.,
                    ..default()
                }),
                TextInputTextColor(TextColor(TEXT_COLOR)),
                TextInputSettings {
                    retain_on_submit: true,
                    ..default()
                },
                TextInputPlaceholder {
                    value: "Username".to_string(),
                    text_color: Some(TextColor(TEXT_COLOR_PLACEHOLDER)),
                    text_font: Some(TextFont {
                        font_size: 20.,
                        ..default()
                    }),
                },
                TextInputInactive(true),
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                TextInputPassword,
                BorderColor(BORDER_COLOR_ACTIVE.into()),
                BackgroundColor(BACKGROUND_COLOR.into()),
                FocusPolicy::Block,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                TextInput,
                TextInputTextFont(TextFont {
                    font_size: 20.,
                    ..default()
                }),
                TextInputTextColor(TextColor(TEXT_COLOR)),
                TextInputSettings {
                    retain_on_submit: true,
                    mask_character: Some('*'),
                },
                TextInputPlaceholder {
                    value: "Password".to_string(),
                    text_color: Some(TextColor(TEXT_COLOR_PLACEHOLDER)),
                    text_font: Some(TextFont {
                        font_size: 20.,
                        ..default()
                    }),
                },
                TextInputInactive(true),
            ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonAction::Login,
                    Button,
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON.into()),
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(Text("Login".to_string()));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonAction::Exit,
                    Button,
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON.into()),
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(Text("Exit".to_string()));
                });
        });
}

fn text_input_focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                }
            }
        }
    }
}

fn auth_scene_logic(
    mut app_exit_events: EventWriter<AppExit>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

#[allow(clippy::type_complexity)]
fn auth_scene_button_logic(
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    text_input_username_query: Query<&TextInputValue, With<TextInputUsername>>,
    text_input_password_query: Query<&TextInputValue, With<TextInputPassword>>,
) {
    for (interaction, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match *action {
                ButtonAction::Login => {
                    let username = text_input_username_query.get_single().unwrap();
                    let password = text_input_password_query.get_single().unwrap();
                    println!("Username: {}", username.0);
                    println!("Password: {}", password.0);
                    app_state.set(AppState::Lobby);
                }
                ButtonAction::Exit => {
                    app_exit_events.send(AppExit::Success);
                }
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn auth_scene_cleanup(mut commands: Commands, query: Query<Entity, With<AuthSceneTag>>) {
    println!("[auth_scene_cleanup]");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Auth), auth_scene_setup);
        app.add_systems(Update, auth_scene_logic.run_if(in_state(AppState::Auth)));
        app.add_systems(
            Update,
            auth_scene_button_logic.run_if(in_state(AppState::Auth)),
        );
        app.add_systems(Update, text_input_focus.run_if(in_state(AppState::Auth)));
        app.add_systems(OnExit(AppState::Auth), auth_scene_cleanup);
    }
}
