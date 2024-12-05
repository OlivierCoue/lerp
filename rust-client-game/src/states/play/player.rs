use crate::states::play::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_transform_interpolation::TranslationInterpolation;
use leafwing_input_manager::prelude::*;

use lightyear::shared::replication::components::Controlled;
use rust_common_game::character_controller::*;
use rust_common_game::protocol::*;
use rust_common_game::shared::*;

// System create the player
#[allow(clippy::type_complexity)]
pub fn handle_new_player(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<
        (Entity, Has<Controlled>),
        (Or<(Added<Predicted>, Added<Interpolated>)>, With<Player>),
    >,
) {
    for (entity, is_controlled) in player_query.iter_mut() {
        println!(
            "[handle_new_player] New Player with id: {}",
            connection.id()
        );

        commands.entity(entity).insert((
            PlaySceneTag,
            RigidBody::Kinematic,
            CharacterController,
            Collider::circle(ENTITY_SIZE / 2.),
            LockedAxes::ROTATION_LOCKED,
            TranslationInterpolation,
            SpriteBundle {
                texture: asset_server.load("assets/gear-sorceress.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(0., -0.20)),
                    ..default()
                },
                ..default()
            },
        ));

        if is_controlled {
            commands
                .entity(entity)
                .insert((InputMap::new([(PlayerActions::Stop, KeyCode::KeyS)])
                    .with(PlayerActions::Move, MouseButton::Left),));
        }
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
    interpolated_q: Query<&Targets, (With<Player>, With<Interpolated>)>,
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

    // Interpolated
    for targets in interpolated_q.iter() {
        if let Some(target) = targets.0.first() {
            gizmos.circle_2d(
                apply_render_mode(&render_config, target),
                12.,
                Color::linear_rgb(0., 1., 1.),
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

#[allow(clippy::type_complexity)]
pub fn sync_cursor_poisition(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    render_config: Res<RenderConfig>,
    mut action_state_query: Query<
        &mut ActionState<PlayerActions>,
        (With<Player>, With<Predicted>, With<Controlled>),
    >,
) {
    let (camera, camera_transform) = camera_query.single();

    let Ok(winodw) = windows.get_single() else {
        return;
    };

    let Some(screen_cursor_position) = winodw.cursor_position() else {
        return;
    };

    let Some(world_cursor_position) =
        camera.viewport_to_world_2d(camera_transform, screen_cursor_position)
    else {
        return;
    };

    let actual_world_cursor_position = match render_config.mode {
        RenderMode::Iso => isometric_to_cartesian(world_cursor_position.x, world_cursor_position.y),
        RenderMode::Cart => world_cursor_position,
    };

    let Ok(mut action_state) = action_state_query.get_single_mut() else {
        return;
    };

    let action = action_state.dual_axis_data_mut_or_default(&PlayerActions::Cursor);
    action.fixed_update_pair = actual_world_cursor_position;
}

#[allow(clippy::type_complexity)]
pub fn set_player_target(
    mut query: Query<
        (&ActionState<PlayerActions>, &mut Targets),
        (With<Player>, With<Predicted>, With<Controlled>),
    >,
) {
    for (action, mut targets) in query.iter_mut() {
        if action.pressed(&PlayerActions::Move) {
            let Some(cursor_position) = action.dual_axis_data(&PlayerActions::Cursor) else {
                println!("cursor_position not set skipping");
                return;
            };

            *targets = Targets(vec![Vec2::new(
                cursor_position.pair.x,
                cursor_position.pair.y,
            )])
        }
    }
}
