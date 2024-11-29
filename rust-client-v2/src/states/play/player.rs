use crate::states::play::*;
use avian2d::prelude::*;
use bevy::{input::mouse::MouseButtonInput, prelude::*};

#[derive(Component)]
pub struct PlayerPhysics;

#[derive(Component)]
pub struct PlayerRender;

#[derive(Component)]
pub struct Targets(Vec<Vec2>);

#[derive(Event)]
pub struct LeftClickEvent {
    world_position: Vec2,
}

pub const ENTITY_SIZE: f32 = 32.0;

// System create the player
pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("[setup_player]");

    // let color = Color::srgb_u8(100, 13, 95);

    let physics_parent = (
        PlaySceneTag,
        PlayerPhysics,
        Targets(Vec::new()),
        RigidBody::Dynamic,
        Collider::circle(ENTITY_SIZE / 2.0),
        LockedAxes::ROTATION_LOCKED,
        Restitution::new(1.0),
        Friction::new(0.0),
    );
    let render_child = (
        PlaySceneTag,
        PlayerRender,
        SpriteBundle {
            texture: asset_server.load("assets/gear-sorceress.png"),
            // sprite: Sprite {
            //     color,

            //     custom_size: Some(Vec2::new(ENTITY_SIZE, ENTITY_SIZE)),
            //     ..default()
            // },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
    );
    spawn_physics_render_pair(&mut commands, physics_parent, render_child);
}

// System to make the camera follow the player
pub fn camera_follow(
    player_query: Query<&Transform, With<PlayerRender>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<PlayerRender>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in &mut camera_query {
            // Update camera position to follow the player
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

pub fn movement(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Targets, &mut LinearVelocity)>,
) {
    let speed = 200000.0;
    for (transform, mut targets, mut velocity) in &mut query {
        if let Some(target) = targets.0.first() {
            let current_position = Vec2::new(transform.translation.x, transform.translation.y);

            let to_target: Vec2 = *target - current_position;
            let distance_to_target = to_target.length();

            if distance_to_target <= 1e-4 {
                velocity.0 = Vec2::ZERO;
                targets.0.clear();
            } else {
                let direction = (*target - current_position).normalize_or_zero();
                let max_distance = speed * time.delta_seconds();

                // Clamp movement to not overshoot the target
                let movement_distance =
                    max_distance.min(distance_to_target * 10000.0 * time.delta_seconds());
                let desired_velocity = direction * movement_distance;

                velocity.0 = desired_velocity;
            }
        }
    }
}

pub fn capture_world_click(
    mut mouse_button_events: EventReader<MouseButtonInput>, // Listen for mouse button input
    camera_query: Query<(&Camera, &GlobalTransform)>, // Camera query to convert screen space to world space
    windows: Query<&Window>,
    mut left_click_events: EventWriter<LeftClickEvent>,
    render_config: Res<RenderConfig>,
) {
    // Check if the left mouse button is pressed
    for event in &mut mouse_button_events.read() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            let (camera, camera_transform) = camera_query.single();

            let Some(cursor_position) = windows.single().cursor_position() else {
                return;
            };

            // Calculate a world position based on the cursor's position.
            let Some(world_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            else {
                return;
            };

            left_click_events.send(LeftClickEvent {
                world_position: match render_config.mode {
                    RenderMode::Iso => isometric_to_cartesian(world_position.x, world_position.y),
                    RenderMode::Cart => world_position,
                },
            });
        }
    }
}

pub fn set_player_target(
    mut left_click_events: EventReader<LeftClickEvent>,
    mut query: Query<&mut Targets, With<PlayerPhysics>>,
) {
    let Ok(mut targets) = query.get_single_mut() else {
        return;
    };

    for event in &mut left_click_events.read() {
        *targets = Targets(vec![event.world_position])
    }
}

#[allow(clippy::type_complexity, unused_mut)]
pub fn sync_transform_physics_with_render(
    physics_parents: Query<
        (&Transform, &RenderChild),
        (Changed<Transform>, Without<PhysicsParent>),
    >,
    mut render_children: Query<&mut Transform, With<PhysicsParent>>,
    render_config: Res<RenderConfig>,
) {
    physics_parents
        .par_iter()
        .for_each(|(parent_transform, children)| {
            if let Ok(mut child_transform) = unsafe { render_children.get_unchecked(children.0) } {
                child_transform.translation = match render_config.mode {
                    RenderMode::Iso => cartesian_to_isometric(
                        parent_transform.translation.x,
                        parent_transform.translation.y,
                    )
                    .extend(child_transform.translation.z),
                    RenderMode::Cart => Vec3::new(
                        parent_transform.translation.x,
                        parent_transform.translation.y,
                        child_transform.translation.z,
                    ),
                };
            }
        });
}
