use avian2d::prelude::*;
use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::{prelude::ActionState, Actionlike};
use lightyear::{
    inputs::leafwing::input_buffer::InputBuffer,
    prelude::{
        client::{Predicted, Rollback},
        server::{Replicate, ReplicationTarget, SyncTarget},
        NetworkIdentity, NetworkTarget, PreSpawnedPlayerObject, TickManager,
    },
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InputVec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect, Actionlike)]
pub enum PlayerActions {
    Move,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    SkillSlot1,
    SkillSlot2,
    SkillSlot3,
    #[actionlike(DualAxis)]
    Cursor,
    SpawnEnemies,
    // TODO: Dirty to dup them for local/remote but don't know how to it in better way yet
    // Also dirty to use TripleAxis to store entity bits but need to PR Leafwing or use another lib...
    #[actionlike(TripleAxis)]
    PickupDroppedItemLocal,
    #[actionlike(TripleAxis)]
    PickupDroppedItemRemote,
}

impl PlayerActions {
    /// You could use the `strum` crate to derive this automatically!
    pub fn variants() -> impl Iterator<Item = PlayerActions> {
        use PlayerActions::*;
        [SkillSlot1, SkillSlot2, SkillSlot3].iter().copied()
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

#[derive(Event)]
/// Trigered when player start to move/dash or any action that should cancel current action
pub struct PlayerCancelAction(pub Entity);

pub fn handle_input_move_wasd(
    mut commands: Commands,
    tick_manager: Res<TickManager>,
    rollback: Option<Res<Rollback>>,
    mut player_cancel_action_ev: EventWriter<PlayerCancelAction>,
    mut player_query: Query<
        (
            Entity,
            &ActionState<PlayerActions>,
            &InputBuffer<PlayerActions>,
            &mut LinearVelocity,
            &Position,
            &MovementSpeed,
            Option<&MovementTarget>,
        ),
        (With<Player>, Or<(With<Predicted>, With<ReplicationTarget>)>),
    >,
) {
    let tick = rollback
        .as_ref()
        .map(|rb| tick_manager.tick_or_rollback_tick(rb))
        .unwrap_or(tick_manager.tick());

    for (entity, action, buffer, mut linear_velocity, position, movement_speed, movement_target) in
        player_query.iter_mut()
    {
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

        if direction != Vec2::ZERO {
            player_cancel_action_ev.send(PlayerCancelAction(entity));
            // TODO: Do we need this here ?
            if movement_target.is_some() {
                commands.entity(entity).remove::<MovementTarget>();
            }
        } else if let Some(movement_target) = movement_target {
            let to_target: Vec2 = movement_target.0 - position.0;
            let distance_to_target = to_target.length();
            // Target reached stopping
            if distance_to_target <= 1e-4 {
                direction = Vec2::ZERO;
                commands.entity(entity).remove::<MovementTarget>();
            } else {
                direction = to_target.normalize();
            }
        }

        let new_velocity = direction * movement_speed.current;
        if new_velocity != linear_velocity.0 {
            linear_velocity.0 = new_velocity
        }
    }
}

fn handle_input_skill_slot(
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

fn handle_input_spawn_enemies(
    tick_manager: Res<TickManager>,
    identity: NetworkIdentity,
    mut commands: Commands,
    player_query: Query<
        &ActionState<PlayerActions>,
        (Or<(With<Predicted>, With<ReplicationTarget>)>,),
    >,
) {
    for action in player_query.iter() {
        if !action.just_pressed(&PlayerActions::SpawnEnemies) {
            continue;
        }

        let mut count = 0;
        for x in 0..5 {
            for y in 0..5 {
                count += 1;
                let enemy = (
                    EnemyBundle::new(0, &Vec2::new(x as f32 * ENEMY_SIZE, y as f32 * ENEMY_SIZE)),
                    PreSpawnedPlayerObject::new(xor_u64s(&[tick_manager.tick().0 as u64, count])),
                );
                let enemy_entity = commands.spawn(enemy).id();

                if identity.is_server() {
                    commands.entity(enemy_entity).insert((Replicate {
                        sync: SyncTarget {
                            prediction: NetworkTarget::All,
                            interpolation: NetworkTarget::None,
                        },
                        target: ReplicationTarget {
                            target: NetworkTarget::All,
                        },
                        group: REPLICATION_GROUP,
                        ..default()
                    },));
                }
            }
        }
    }
}

fn handle_input_click(
    identity: NetworkIdentity,
    mut commands: Commands,
    player_query: Query<
        (Entity, &ActionState<PlayerActions>),
        (Or<(With<Predicted>, With<ReplicationTarget>)>,),
    >,
) {
    for (player_entity, action_state) in player_query.iter() {
        let action = if identity.is_client() {
            &PlayerActions::PickupDroppedItemLocal
        } else {
            &PlayerActions::PickupDroppedItemRemote
        };
        let Some(clicked_entity) = action_state.triple_axis_data(action) else {
            continue;
        };
        if clicked_entity.triple == Vec3::ZERO {
            continue;
        }
        // Dirty way to recreate Entity from bits stored in 3 f32 in a triple axis Leafwing action..
        let clicked_entity = Entity::from_bits(vec3_to_u64(clicked_entity.triple));

        commands
            .entity(player_entity)
            .insert(PendingItemDroppedPickup(clicked_entity));
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerCancelAction>();
        app.add_systems(
            FixedUpdate,
            (
                handle_input_move_wasd,
                handle_input_skill_slot,
                handle_input_spawn_enemies,
                handle_input_click,
            )
                .chain()
                .in_set(GameSimulationSet::RegisterInputs),
        );
    }
}
