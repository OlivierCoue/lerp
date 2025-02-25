use bevy::{
    input::mouse::MouseButton,
    input::mouse::MouseWheel,
    prelude::*,
    window::{PrimaryWindow, Window},
};
use lerp_map_gen::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MapGenPlugin))
        .add_systems(Startup, (setup, render_map).chain())
        .add_systems(Update, (zoom_camera, drag_camera))
        // .add_systems(Startup, setup)
        // .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut map: ResMut<Map>,
) {
    commands.spawn(Camera2d);
    generate_map(&mut map, UVec2::new(100, 200));
    // let layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 5, 1, None, None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // commands.spawn((
    //     Sprite::from_atlas_image(
    //         asset_server.load("sprite_du_cul.png"),
    //         TextureAtlas {
    //             layout: texture_atlas_layout,
    //             index: 4,
    //         },
    //     ),
    //     // Sprite::from_image(asset_server.load("branding/icon.png")),
    //     // Sprite::from_color(),
    //     Transform::from_xyz(100., 0., 0.),
    //     Direction::Up,
    // ));
}

fn render_map(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    map: Res<Map>,
) {
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
