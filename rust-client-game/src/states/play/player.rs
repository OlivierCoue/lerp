use crate::states::play::*;
use avian2d::prelude::*;
use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_transform_interpolation::TranslationInterpolation;
use lightyear::shared::replication::components::Controlled;
use rust_common_game::protocol::*;
use rust_common_game::shared::*;

// System create the player
#[allow(clippy::type_complexity)]
pub fn handle_new_player(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(Entity, Has<Controlled>), (Added<Predicted>, With<Player>)>,
) {
    for (entity, is_controlled) in player_query.iter_mut() {
        println!(
            "[handle_new_player] New Player with id: {}",
            connection.id()
        );

        commands.entity(entity).insert((
            PlaySceneTag,
            RigidBody::Dynamic,
            Collider::circle(ENTITY_SIZE / 2.0),
            LockedAxes::ROTATION_LOCKED,
            Restitution::new(1.0),
            Friction::new(0.0),
            TranslationInterpolation,
            SpriteBundle {
                texture: asset_server.load("assets/gear-sorceress.png"),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
        ));
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn draw_confirmed_player(
    mut gizmos: Gizmos,
    confirmed_q: Query<&Position, (With<Player>, With<Confirmed>)>,
    render_config: Res<RenderConfig>,
) {
    for position in confirmed_q.iter() {
        gizmos.circle_2d(
            apply_render_mode(&render_config, &position.0),
            15.,
            Color::linear_rgb(1., 0., 0.),
        );
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn draw_predicted_target(
    mut gizmos: Gizmos,
    confirmed_q: Query<&Targets, (With<Player>, With<Confirmed>)>,
    predicted_q: Query<&Targets, (With<Player>, With<Predicted>)>,
    render_config: Res<RenderConfig>,
) {
    // Predicted
    for targets in predicted_q.iter() {
        if let Some(target) = targets.0.first() {
            gizmos.circle_2d(
                apply_render_mode(&render_config, target),
                15.,
                Color::linear_rgb(0., 0., 1.),
            );
        }
    }

    // Confirmed
    for targets in confirmed_q.iter() {
        if let Some(target) = targets.0.first() {
            gizmos.circle_2d(
                apply_render_mode(&render_config, target),
                12.,
                Color::linear_rgb(0., 1., 0.),
            );
        }
    }
}

pub fn movement(
    time: Res<Time<Physics>>,
    mut query: Query<(&Position, &mut Targets, &mut LinearVelocity), With<Predicted>>,
) {
    for (position, targets, velocity) in &mut query {
        shared_movement_behaviour(&time, position, velocity, targets);
    }
}

pub(crate) fn buffer_input(
    tick_manager: Res<TickManager>,
    mut input_manager: ResMut<InputManager<Inputs>>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    camera_query: Query<(&Camera, &GlobalTransform)>, // Camera query to convert screen space to world space
    windows: Query<&Window>,
    render_config: Res<RenderConfig>,
) {
    let tick = tick_manager.tick();

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

            let target = match render_config.mode {
                RenderMode::Iso => isometric_to_cartesian(world_position.x, world_position.y),
                RenderMode::Cart => world_position,
            };
            input_manager.add_input(
                Inputs::Target(InputVec2 {
                    x: target.x,
                    y: target.y,
                }),
                tick,
            )
        }
    }
}

pub fn set_player_target(
    mut input_reader: EventReader<InputEvent<Inputs>>,
    mut query: Query<&mut Targets, (With<Player>, With<Predicted>)>,
) {
    for input in input_reader.read() {
        if let Some(Inputs::Target(target)) = input.input() {
            let Ok(mut targets) = query.get_single_mut() else {
                return;
            };
            *targets = Targets(vec![Vec2::new(target.x, target.y)])
        }
    }
}
