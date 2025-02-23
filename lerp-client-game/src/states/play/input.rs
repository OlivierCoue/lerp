use bevy::prelude::*;
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::ActionState};
use lightyear::{
    client::input::leafwing::InputSystemSet, prelude::client::Predicted,
    shared::replication::components::Controlled,
};
use lerp_common_game::prelude::*;

use crate::{common::AppState, states::play::cursor::HoverableEntityKind};

use super::cursor::CursorState;

#[derive(Resource)]
// TODO: Rework this quick and dirty resource to handle just pressed in FixedUpdate
struct FixedMouseState {
    left_button_pressed: bool,
    left_button_just_pressed: bool,
}

fn handle_mouse_click(
    mut fixed_mouse_state: ResMut<FixedMouseState>,
    cursor_state: Res<CursorState>,
    mouse_button_state: Res<ButtonInput<MouseButton>>,
    mut action_state_query: Query<
        &mut ActionState<PlayerActions>,
        (With<Player>, With<Predicted>, With<Controlled>),
    >,
) {
    let left_button_pressed = mouse_button_state.pressed(MouseButton::Left);

    let Ok(mut action_state) = action_state_query.get_single_mut() else {
        return;
    };

    // If Left click is releaded we set reset all actions bind to the left click
    if !left_button_pressed {
        fixed_mouse_state.left_button_pressed = false;
        fixed_mouse_state.left_button_just_pressed = false;
        action_state.set_axis_triple(&PlayerActions::PickupDroppedItemLocal, Vec3::ZERO);
        action_state.set_axis_triple(&PlayerActions::PickupDroppedItemRemote, Vec3::ZERO);
        action_state.release(&PlayerActions::SkillSlot1);
        return;
    } else if !fixed_mouse_state.left_button_pressed {
        fixed_mouse_state.left_button_pressed = true;
        fixed_mouse_state.left_button_just_pressed = true;
    } else if fixed_mouse_state.left_button_just_pressed {
        fixed_mouse_state.left_button_just_pressed = false;
        action_state.set_axis_triple(&PlayerActions::PickupDroppedItemLocal, Vec3::ZERO);
        action_state.set_axis_triple(&PlayerActions::PickupDroppedItemRemote, Vec3::ZERO);
    }

    // If the cursor is not over an interactable element, we trigger de default SkillSlot1
    let Some(entity_hover) = cursor_state.entity_hover else {
        action_state.press(&PlayerActions::SkillSlot1);
        return;
    };

    if !fixed_mouse_state.left_button_just_pressed {
        return;
    }

    // Trigger the correct action based on which interactable element the cursor is over
    match entity_hover.kind {
        HoverableEntityKind::DroppedItem => {
            action_state.set_axis_triple(
                &PlayerActions::PickupDroppedItemLocal,
                u64_to_vec3(entity_hover.local_entity.to_bits()),
            );
            action_state.set_axis_triple(
                &PlayerActions::PickupDroppedItemRemote,
                u64_to_vec3(entity_hover.remote_entity.to_bits()),
            );
        }
    }
}

fn handle_mouse_move(
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

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedMouseState {
            left_button_pressed: false,
            left_button_just_pressed: false,
        });
        app.add_systems(
            FixedPreUpdate,
            (handle_mouse_click, handle_mouse_move)
                // WARNING: chain() matter, create desync/rollback if removed
                .chain()
                .before(InputSystemSet::BufferClientInputs)
                .in_set(InputManagerSystem::ManualControl)
                .run_if(in_state(AppState::Play)),
        );
    }
}
