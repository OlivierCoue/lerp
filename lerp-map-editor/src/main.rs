use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                focused: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
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
) {
    commands.spawn(Camera2d);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        Sprite::from_atlas_image(
            asset_server.load("sprite_du_cul.png"),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 4,
            },
        ),
        // Sprite::from_image(asset_server.load("branding/icon.png")),
        // Sprite::from_color(),
        Transform::from_xyz(100., 0., 0.),
        Direction::Up,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_secs(),
            Direction::Down => transform.translation.y -= 150. * time.delta_secs(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}
