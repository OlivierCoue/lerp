use crate::common::*;
use crate::ui::text_input::create_text_input;
use crate::ui::*;
use bevy::prelude::*;
use bevy_simple_text_input::*;

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

    commands.spawn((AuthSceneTag, Camera2d));
    commands.spawn((AuthSceneTag, Text("Authentication Scene".to_string())));
    let container_entity = commands
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
        .id();

    let username_text_input_entity = create_text_input(
        &mut commands,
        TextInputUsername,
        "Username".to_string(),
        None,
    );

    let password_text_input_entity = create_text_input(
        &mut commands,
        TextInputPassword,
        "Password".to_string(),
        None,
    );

    commands
        .entity(container_entity)
        .add_children(&[username_text_input_entity, password_text_input_entity]);

    commands.entity(container_entity).with_children(|parent| {
        parent
            .spawn((
                ButtonAction::Login,
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
                parent.spawn(Text("Login".to_string()));
            });

        parent
            .spawn((
                ButtonAction::Exit,
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
                parent.spawn(Text("Exit".to_string()));
            });
    });
}

fn auth_scene_logic(
    mut app_exit_events: EventWriter<AppExit>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

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
        app.add_systems(OnExit(AppState::Auth), auth_scene_cleanup);
    }
}
