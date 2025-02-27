use bevy::{
    input::mouse::MouseButton,
    input::mouse::MouseWheel,
    prelude::*,
    ui::Interaction,
    window::{PrimaryWindow, Window},
};
use lerp_map_gen::*;
use rand::Rng;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const MAP_SIZE: UVec2 = UVec2::new(50, 50);

#[derive(Event, Default)]
struct RenderMapEvent;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MapGenPlugin))
        .add_event::<RenderMapEvent>()
        .add_systems(Startup, (setup).chain())
        .add_systems(
            Update,
            (
                zoom_camera,
                drag_camera,
                button_system,
                render_map.run_if(on_event::<RenderMapEvent>),
            ),
        )
        // .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component, Clone, Copy)]
enum ButtonAction {
    ResetMap,
    GenerateBSP,
    WeightedRandomSplit,
    SliceAndDice,
}
#[derive(Component, Default)]
struct Tile {}
fn setup(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut render_map_event: EventWriter<RenderMapEvent>,
) {
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(20.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
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
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    ButtonAction::ResetMap,
                ))
                .with_child((
                    Text::new("Reset"),
                    TextFont {
                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    ButtonAction::GenerateBSP,
                ))
                .with_child((
                    Text::new("GenerateBSP "),
                    TextFont {
                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    ButtonAction::WeightedRandomSplit,
                ))
                .with_child((
                    Text::new("Weithted Random Split "),
                    TextFont {
                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    ButtonAction::SliceAndDice,
                ))
                .with_child((
                    Text::new("Slide and dice "),
                    TextFont {
                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
    map.generate_map(MAP_SIZE);
    render_map_event.send(RenderMapEvent);
}

fn render_map(mut commands: Commands, map: Res<Map>, mut sprite_query: Query<Entity, With<Tile>>) {
    // Clear existing map sprites
    for entity in sprite_query.iter_mut() {
        commands.entity(entity).despawn();
    }
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
                Tile::default(),
            ));
        }
    }
    println!("Map Rendered")
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
            &ButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut map: ResMut<Map>,
    mut render_map_event: EventWriter<RenderMapEvent>,
) {
    let mut rng = rand::rng();

    for (interaction, mut color, mut border_color, button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
                match *button_action {
                    ButtonAction::GenerateBSP => {
                        map.generate_bsp_floor(rng.random_range(6..10), UVec2::new(5, 5));
                    }
                    ButtonAction::ResetMap => {
                        map.generate_map(MAP_SIZE);
                    }
                    ButtonAction::WeightedRandomSplit => {
                        map.generate_weighted_random_split(UVec2::new(
                            rng.random_range(30..100),
                            rng.random_range(30..100),
                        ));
                    }
                    ButtonAction::SliceAndDice => {
                        map.generate_slice_and_dice(UVec2::new(
                            rng.random_range(30..100),
                            rng.random_range(30..100),
                        ));
                    }
                }
                render_map_event.send(RenderMapEvent);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
