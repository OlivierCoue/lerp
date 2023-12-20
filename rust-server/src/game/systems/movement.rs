use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::collisions::collide_rect_to_rect;

use crate::game::{components::prelude::*, events::prelude::*, resources::prelude::*};

pub const GRID_SIZE_X_MIN: f32 = -1024.0;
pub const GRID_SIZE_X_MAX: f32 = 1024.0;
pub const GRID_SIZE_Y_MIN: f32 = -1024.0;
pub const GRID_SIZE_Y_MAX: f32 = 1024.0;

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

#[allow(clippy::type_complexity)]
pub fn movement(
    mut query_entities_to_move: Query<(
        Entity,
        &mut GameEntity,
        &Position,
        &Velocity,
        Option<&ColliderMvt>,
    )>,
    query_entities_blocking: Query<(Entity, &Position, &ColliderMvt)>,
    mut writer_update_position_current: EventWriter<UpdatePositionCurrent>,
    mut writer_update_velocity_target: EventWriter<UpdateVelocityTarget>,
    // mut writer_add_velocity_target: EventWriter<AddVelocityTarget>,
    time: Res<Time>,
) {
    for (entity, mut game_entity, position, velocity, opt_collider_mvt) in
        &mut query_entities_to_move
    {
        if let Some(target) = velocity.get_target() {
            if is_oob(&position.current) && !game_entity.pending_despwan {
                game_entity.pending_despwan = true;
            } else if position.current == *target {
                continue;
            }

            let new_position_current = position
                .current
                .move_toward(*target, velocity.get_speed() * time.delta);

            let mut collide_with_blocking_entity = false;
            // Only apply collision with others entities if the entity we attempt to move also have a collider
            if let Some(collider_mvt) = opt_collider_mvt {
                for (entity_blocking, position_blocking, collider_mvt_blocking) in
                    &query_entities_blocking
                {
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

            if !collide_with_blocking_entity {
                writer_update_position_current.send(UpdatePositionCurrent {
                    entity,
                    current: new_position_current,
                    force_update_velocity_target: false,
                })
            } else {
                writer_update_velocity_target.send(UpdateVelocityTarget {
                    entity,
                    target: None,
                })
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
        &mut GameEntity,
        Or<(
            Changed<GameEntity>,
            Changed<Position>,
            Changed<Velocity>,
            Changed<Health>,
        )>,
    >,
) {
    for mut game_entity in &mut query {
        game_entity.revision += 1;
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
                    velocity.set_target(Some(event.current));
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
