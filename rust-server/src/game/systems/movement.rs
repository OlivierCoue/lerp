use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::collisions::collide_rect_to_rect;

use crate::{
    game::{components::prelude::*, events::prelude::*, resources::prelude::*},
    utils::get_game_time,
};

pub const GRID_WIDTH: u32 = 2048;
pub const GRID_HEIGHT: u32 = 2048;
pub const GRID_SIZE_X_MIN: f32 = 0.0;
pub const GRID_SIZE_X_MAX: f32 = 2048.0;
pub const GRID_SIZE_Y_MIN: f32 = 0.0;
pub const GRID_SIZE_Y_MAX: f32 = 2048.0;

fn wolrd_bounded_x(x: f32) -> f32 {
    f32::min(f32::max(GRID_SIZE_X_MIN, x), GRID_SIZE_X_MAX)
}

fn world_bounded_y(y: f32) -> f32 {
    f32::min(f32::max(GRID_SIZE_Y_MIN, y), GRID_SIZE_Y_MAX)
}

pub fn world_bounded_vector2(v: Vector2) -> Vector2 {
    Vector2 {
        x: wolrd_bounded_x(v.x),
        y: world_bounded_y(v.y),
    }
}

fn is_oob(position: &Vector2) -> bool {
    position.x > GRID_SIZE_X_MAX
        || position.x < GRID_SIZE_X_MIN
        || position.y > GRID_SIZE_Y_MAX
        || position.y < GRID_SIZE_Y_MIN
}

pub fn update_pathfinder_state(
    query: Query<(Entity, &Position, &ColliderMvt)>,
    mut pathfinder_state: ResMut<PathfinderState>,
) {
    let current_game_time = get_game_time();

    if pathfinder_state.last_update_at_millis + pathfinder_state.update_every_millis
        < current_game_time
    {
        pathfinder_state.reset();
        for (entity, position, collider_mvt) in &query {
            pathfinder_state.block_nodes_in_rect(entity, &position.current, &collider_mvt.rect)
        }
    }
}

#[allow(clippy::type_complexity)]
#[allow(unused_mut)] // mut is actually used on query_entities_to_move with get_component_unchecked_mut
pub fn movement(
    mut commands: Commands,
    mut query_entities_to_move: Query<(
        Entity,
        &mut GameEntity,
        &mut Position,
        Option<&mut Velocity>,
        Option<&ColliderMvt>,
        Option<&Cast>,
    )>,
    mut writer_update_velocity_target: EventWriter<UpdateVelocityTarget>,
    mut writer: EventWriter<VelocityReachedTarget>,
    time: Res<Time>,
) {
    for (entity, game_entity, position, opt_velocity, opt_collider_mvt, opt_cast) in
        &query_entities_to_move
    {
        if let Some(mut velocity) = opt_velocity {
            if let Some(target) = velocity.get_target() {
                if is_oob(&position.current) && !game_entity.pending_despwan {
                    let mut game_entity_mut = unsafe {
                        query_entities_to_move
                            .get_component_unchecked_mut::<GameEntity>(entity)
                            .unwrap()
                    };
                    game_entity_mut.pending_despwan = true;
                } else if position.current == *target {
                    continue;
                }

                let new_position_current = position
                    .current
                    .move_toward(*target, velocity.get_speed() * time.delta);

                let mut collide_with_blocking_entity = false;
                // Only apply collision with others entities if the entity we attempt to move also have a collider
                if let Some(collider_mvt) = opt_collider_mvt {
                    for (entity_blocking, _, position_blocking, _, opt_collider_mvt_blocking, _) in
                        &query_entities_to_move
                    {
                        if let Some(collider_mvt_blocking) = opt_collider_mvt_blocking {
                            if entity_blocking != entity
                                && collide_rect_to_rect(
                                    &collider_mvt.rect,
                                    &new_position_current,
                                    &collider_mvt_blocking.rect,
                                    &position_blocking.current,
                                )
                            {
                                collide_with_blocking_entity = true;
                                break;
                            }
                        }
                    }
                }

                if !collide_with_blocking_entity {
                    let mut position_mut = unsafe {
                        query_entities_to_move
                            .get_component_unchecked_mut::<Position>(entity)
                            .unwrap()
                    };
                    position_mut.current = new_position_current;
                    if let Some(target) = velocity.get_target() {
                        if position.current == *target {
                            writer.send(VelocityReachedTarget {
                                entity,
                                target: *target,
                            });
                            let mut velocity_mut = unsafe {
                                query_entities_to_move
                                    .get_component_unchecked_mut::<Velocity>(entity)
                                    .unwrap()
                            };
                            velocity_mut.remove_current_target();
                        }
                    }
                    if opt_cast.is_some() {
                        commands.entity(entity).remove::<Cast>();
                    }
                } else {
                    writer_update_velocity_target.send(UpdateVelocityTarget {
                        entity,
                        target: None,
                    });
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn despawn_if_velocity_at_target(
    mut query: Query<
        (&mut GameEntity, &Position, &Velocity),
        Or<(Changed<Position>, Changed<Velocity>)>,
    >,
) {
    for (mut game_entity, position, velocity) in &mut query {
        if let Some(target) = velocity.get_target() {
            if !game_entity.pending_despwan
                && velocity.get_despawn_at_target()
                && position.current == *target
            {
                game_entity.pending_despwan = true;
            }
        } else if velocity.get_despawn_at_target() && !game_entity.pending_despwan {
            game_entity.pending_despwan = true;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn increase_game_entity_revision(
    mut query: Query<
        (
            &mut GameEntity,
            Option<&mut Position>,
            Option<&mut Velocity>,
            Option<&mut Health>,
        ),
        Or<(
            Changed<GameEntity>,
            Changed<Position>,
            Changed<Velocity>,
            Changed<Health>,
        )>,
    >,
) {
    for (mut game_entity, opt_position, opt_velocity, opt_health) in &mut query {
        let mut increase_revision = game_entity.pending_despwan;
        if let Some(mut position) = opt_position {
            if position.revision > position.revision_checkpoint {
                position.revision_checkpoint = position.revision;
                increase_revision = true;
            }
        }
        if let Some(mut velocity) = opt_velocity {
            if velocity.revision > velocity.revision_checkpoint {
                velocity.revision_checkpoint = velocity.revision;
                increase_revision = true;
            }
        }
        if let Some(mut health) = opt_health {
            if health.revision > health.revision_checkpoint {
                health.revision_checkpoint = health.revision;
                increase_revision = true;
            }
        }
        if increase_revision {
            game_entity.revision += 1;
        }
    }
}

pub fn on_update_velocity_target(
    mut reader: EventReader<UpdateVelocityTarget>,
    mut query: Query<&mut Velocity>,
) {
    for event in reader.read() {
        if let Ok(mut velocity) = query.get_mut(event.entity) {
            velocity.set_target(event.target);
        }
    }
}

pub fn on_update_velocity_target_with_pathfinder(
    mut reader: EventReader<UpdateVelocityTargetWithPathFinder>,
    mut query: Query<(Entity, &Position, &mut Velocity)>,
    mut pathfinder_state: ResMut<PathfinderState>,
) {
    let mut handlers_per_entities = Vec::new();
    for event in reader.read() {
        if let Ok((entity, position, _)) = query.get(event.entity) {
            handlers_per_entities.push((
                pathfinder_state.get_path_async(entity, position.current, event.target),
                entity,
            ));
        }
    }

    for (handler, entity) in handlers_per_entities {
        let opt_targets = handler.join();
        if let Ok((_, _, mut velocity)) = query.get_mut(entity) {
            if let Ok(Some(targets)) = opt_targets {
                velocity.set_targets(targets);
            }
        }
    }
}

pub fn on_add_velocity_target(
    mut reader: EventReader<AddVelocityTarget>,
    mut query: Query<&mut Velocity>,
) {
    for event in reader.read() {
        if let Ok(mut velocity) = query.get_mut(event.entity) {
            velocity.add_target(event.target);
        }
    }
}

pub fn on_update_position_current(
    mut reader: EventReader<UpdatePositionCurrent>,
    mut query: Query<(Entity, &mut Position, Option<&mut Velocity>)>,
    mut writer: EventWriter<VelocityReachedTarget>,
) {
    for event in reader.read() {
        if let Ok((entity, mut position, opt_velocity)) = query.get_mut(event.entity) {
            position.current = event.current;

            if let Some(mut velocity) = opt_velocity {
                if event.force_update_velocity_target {
                    velocity.set_target(None);
                }

                if let Some(target) = velocity.get_target() {
                    if position.current == *target {
                        writer.send(VelocityReachedTarget {
                            entity,
                            target: *target,
                        });
                        velocity.remove_current_target();
                    }
                }
            }
        }
    }
}
