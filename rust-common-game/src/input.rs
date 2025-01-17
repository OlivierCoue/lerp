use avian2d::prelude::*;
use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::{prelude::ActionState, Actionlike};
use lightyear::{
    inputs::leafwing::input_buffer::InputBuffer,
    prelude::{
        client::{Predicted, Rollback},
        server::ReplicationTarget,
        TickManager,
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    protocol::*,
    skill::{SkillName, SkillsAvailable, TriggerSkillEvent},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InputVec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum PlayerActions {
    Move,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    SkillSlot1,
    SkillSlot2,
    #[actionlike(DualAxis)]
    Cursor,
}
impl PlayerActions {
    /// You could use the `strum` crate to derive this automatically!
    pub fn variants() -> impl Iterator<Item = PlayerActions> {
        use PlayerActions::*;
        [SkillSlot1, SkillSlot2].iter().copied()
    }
}

#[derive(
    Component,
    Serialize,
    Deserialize,
    Debug,
    Default,
    PartialEq,
    Eq,
    Clone,
    Reflect,
    Deref,
    DerefMut,
)]
pub struct SkillSlotMap {
    map: HashMap<PlayerActions, SkillName>,
}

pub fn shared_handle_move_click_behavior(
    action: &ActionState<PlayerActions>,
    mut targets: Mut<MovementTargets>,
) {
    if action.pressed(&PlayerActions::Move) {
        let Some(cursor_position) = action.dual_axis_data(&PlayerActions::Cursor) else {
            println!("cursor_position not set skipping");
            return;
        };

        *targets = MovementTargets(vec![Vec2::new(
            cursor_position.pair.x,
            cursor_position.pair.y,
        )])
    }
}

pub fn handle_input_move_wasd(
    tick_manager: Res<TickManager>,
    rollback: Option<Res<Rollback>>,
    mut player_query: Query<
        (
            &ActionState<PlayerActions>,
            &InputBuffer<PlayerActions>,
            &mut LinearVelocity,
            &MovementSpeed,
        ),
        (With<Player>, Or<(With<Predicted>, With<ReplicationTarget>)>),
    >,
) {
    let tick = rollback
        .as_ref()
        .map(|rb| tick_manager.tick_or_rollback_tick(rb))
        .unwrap_or(tick_manager.tick());

    for (action, buffer, mut linear_velocity, movement_speed) in player_query.iter_mut() {
        let action = if buffer.get(tick).is_some() {
            action
        } else {
            &ActionState::default()
        };

        let up = action.pressed(&PlayerActions::MoveUp);
        let down = action.pressed(&PlayerActions::MoveDown);
        let left = action.pressed(&PlayerActions::MoveLeft);
        let right = action.pressed(&PlayerActions::MoveRight);

        // Adjust directions for isometric mapping
        let mut direction = Vec2::ZERO;

        if up {
            direction += Vec2::new(-1.0, 1.0); // Up-left
        }
        if down {
            direction += Vec2::new(1.0, -1.0); // Down-right
        }
        if left {
            direction += Vec2::new(-1.0, -1.0); // Down-left
        }
        if right {
            direction += Vec2::new(1.0, 1.0); // Up-right
        }

        direction = direction.clamp_length_max(1.0);
        let new_velocity = direction * movement_speed.0;
        if new_velocity != linear_velocity.0 {
            linear_velocity.0 = new_velocity
        }
    }
}

pub fn handle_input_skill_slot(
    mut skill_trigger_ev: EventWriter<TriggerSkillEvent>,
    player_query: Query<
        (
            Entity,
            &ActionState<PlayerActions>,
            &SkillSlotMap,
            &SkillsAvailable,
        ),
        (Or<(With<Predicted>, With<ReplicationTarget>)>,),
    >,
) {
    for (entity, action, skill_slot_map, skills_available) in player_query.iter() {
        let Some(cursor_position) = action.dual_axis_data(&PlayerActions::Cursor) else {
            continue;
        };

        for player_action in PlayerActions::variants() {
            if !action.pressed(&player_action) {
                continue;
            }
            let Some(skill_name) = skill_slot_map.get(&player_action) else {
                println!("[handle_input_skill_slot] Action is not bound to any skill");
                continue;
            };
            let Some(skill_entity) = skills_available.get(skill_name) else {
                error!(
                    "[handle_input_skill_slot] Skill {:?} is not attach to this player",
                    skill_name
                );
                continue;
            };

            skill_trigger_ev.send(TriggerSkillEvent {
                initiator: entity,
                skill: *skill_entity,
                target: cursor_position.pair,
            });
            break;
        }
    }
}

pub fn shared_move_to_target_behaviour(
    time: &Res<Time<Physics>>,
    position: &Position,
    movement_speed: &MovementSpeed,
    mut velocity: Mut<LinearVelocity>,
    mut targets: Mut<MovementTargets>,
) {
    if let Some(target) = targets.0.first() {
        let to_target: Vec2 = *target - position.0;
        let distance_to_target = to_target.length();

        // If close enough to the target, stop movement
        if distance_to_target <= 1e-4 {
            velocity.0 = Vec2::ZERO;
            targets.0.clear();
        } else {
            // Calculate direction to the target
            let direction = to_target.normalize_or_zero();
            // Compute movement distance based on speed and delta time
            let max_distance = movement_speed.0 * time.delta_secs();

            // If the next step overshoots the target, use reduced velocity
            if max_distance > distance_to_target {
                *velocity = LinearVelocity(direction * (distance_to_target / time.delta_secs()));
            // Else go at max speed
            } else {
                *velocity = LinearVelocity(
                    (direction * movement_speed.0).clamp_length_max(movement_speed.0),
                )
            }
        }
    }
}
