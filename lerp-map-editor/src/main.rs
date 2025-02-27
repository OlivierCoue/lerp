use bevy::{
    input::mouse::MouseButton,
    input::mouse::MouseWheel,
    prelude::*,
    ui::Interaction,
    window::{PrimaryWindow, Window},
};
use lerp_map_gen::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MapGenPlugin))
        .add_systems(Startup, (setup, render_map).chain())
        .add_systems(Update, (zoom_camera, drag_camera, button_system))
        // .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut map: ResMut<Map>) {
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(15.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Spawn "),
                    TextFont {
                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
    map.generate_map(UVec2::new(50, 50));
}

fn render_map(mut commands: Commands, map: Res<Map>) {
    for x in 0..map.render_grid_size.x {
        for y in 0..map.render_grid_size.y {
            let color = match map.get_tile(&TilePos(UVec2::new(x, y))).unwrap() {
                TileKind::Floor => Color::srgb_u8(255, 255, 255),
                TileKind::Wall => Color::srgb_u8(255, 0, 0),
                TileKind::Water => Color::srgb_u8(0, 0, 255),
            };
            commands.spawn((
                Sprite::from_color(color, Vec2::new(10., 10.)),
                Transform::from_translation(Vec3::new(
                    x as f32 * 10. - (map.render_grid_size.x as f32 * 10. / 2.),
                    y as f32 * 10. - (map.render_grid_size.y as f32 * 10. / 2.),
                    0.,
                )),
            ));
        }
    }
}

fn zoom_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = primary_window_query.single();
    let mut projection = query.single_mut();

    for event in mouse_wheel_events.read() {
        let zoom_amount = event.y * 0.1; // Adjust sensitivity as needed

        projection.scale -= zoom_amount;
        projection.scale = projection.scale.max(0.1); // Prevent excessive zoom-in
    }
}

fn drag_camera(
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera2d>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut last_mouse_position: Local<Option<Vec2>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = primary_window_query.single();
    if mouse_buttons.pressed(MouseButton::Left) {
        if let Some(mouse_position) = window.cursor_position() {
            if let Some(last_position) = *last_mouse_position {
                let delta = mouse_position - last_position;

                for (mut transform, projection) in query.iter_mut() {
                    // Adjust delta for camera zoom
                    let adjusted_delta = delta * projection.scale;

                    transform.translation.x -= adjusted_delta.x;
                    transform.translation.y += adjusted_delta.y; // Invert Y for typical screen space
                }
            }
            *last_mouse_position = Some(mouse_position);
        }
    } else {
        *last_mouse_position = None; // Reset when not dragging
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut map: ResMut<Map>,
    mut commands: Commands,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                map.generate_bsp_floor(10, UVec2::new(5, 5));
                render_map(commands, map.as_ref());
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
