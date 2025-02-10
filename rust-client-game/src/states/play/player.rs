use crate::states::play::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::PreSpawnedPlayerObject;
use lightyear::shared::replication::components::Controlled;
use rust_common_game::prelude::*;

pub fn handle_new_client(
    mut client_query: Query<
        (Entity, &PlayerClient),
        (Added<Predicted>, With<PlayerClient>, With<Controlled>),
    >,
) {
    for (_, player_client) in client_query.iter_mut() {
        println!(
            "[handle_new_client] New client with id: {}",
            player_client.client_id
        );
    }
}

#[derive(Component)]
pub struct PlayerRender;

pub fn handle_new_player(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, Has<Controlled>),
        (
            Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>,
            With<Player>,
            Without<PlayerRender>,
        ),
    >,
) {
    for (entity, controlled) in player_query.iter_mut() {
        println!("[handle_new_player] New Player");
        commands.entity(entity).insert(PlayerRender);

        if controlled {
            commands.entity(entity).insert(
                InputMap::new([
                    (PlayerActions::MoveUp, KeyCode::KeyW),
                    (PlayerActions::MoveDown, KeyCode::KeyS),
                    (PlayerActions::MoveLeft, KeyCode::KeyA),
                    (PlayerActions::MoveRight, KeyCode::KeyD),
                    (PlayerActions::SkillSlot3, KeyCode::KeyE),
                ])
                .with(PlayerActions::SkillSlot1, MouseButton::Left)
                .with(PlayerActions::SkillSlot2, MouseButton::Right),
            );
        }
    }
}

pub fn sync_cursor_poisition(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
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

    let Ok(world_cursor_position) =
        camera.viewport_to_world_2d(camera_transform, screen_cursor_position)
    else {
        return;
    };

    let actual_world_cursor_position = isometric_to_cartesian(
        world_cursor_position.x,
        world_cursor_position.y - 1. * PIXEL_METER,
    );

    let Ok(mut action_state) = action_state_query.get_single_mut() else {
        return;
    };

    action_state.set_axis_pair(&PlayerActions::Cursor, actual_world_cursor_position);
}
