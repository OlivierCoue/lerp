use bevy_ecs::prelude::*;

use rust_common::{
    collisions::{collide_poly_to_circle, collide_poly_to_point, collide_rect_to_rect},
    math::Vec2,
};

use crate::game::{
    ecs::{components::prelude::*, events::prelude::*, resources::prelude::*},
    Enemie,
};

fn wolrd_bounded_x(area_config: &AreaConfig, x: f32) -> f32 {
    f32::min(f32::max(0.0, x), area_config.area_width)
}

fn world_bounded_y(area_config: &AreaConfig, y: f32) -> f32 {
    f32::min(f32::max(0.0, y), area_config.area_height)
}

pub fn world_bounded_vector2(area_config: &AreaConfig, v: Vec2) -> Vec2 {
    Vec2 {
        x: wolrd_bounded_x(area_config, v.x),
        y: world_bounded_y(area_config, v.y),
    }
}

fn is_oob(area_config: &AreaConfig, position: &Vec2) -> bool {
    position.x > area_config.area_width
        || position.x < 0.0
        || position.y > area_config.area_height
        || position.y < 0.0
}

pub fn update_pathfinder_state(
    query: Query<(Entity, &Position, &ColliderMvt), Without<Enemie>>,
    mut pathfinder_state: ResMut<PathfinderState>,
    area_config: Res<AreaConfig>,
    time: Res<Time>,
) {
    let current_game_time = time.current_millis;
    if pathfinder_state.is_init {
        return;
    }

    if pathfinder_state.last_update_at_millis + pathfinder_state.update_every_millis
        < current_game_time
    {
        pathfinder_state.reset(&area_config, &time);
        for (entity, position, collider_mvt) in &query {
            if let Some(rect) = &collider_mvt.shape.rect {
                pathfinder_state.block_nodes_in_rect(entity, &position.current, rect)
            } else if let Some(poly) = &collider_mvt.shape.poly {
                pathfinder_state.block_nodes_in_poly(entity, poly, collider_mvt.shape.inverse)
            }
        }
        pathfinder_state.is_init = true;
    }
}

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
    area_config: Res<AreaConfig>,
) {
    for (entity, game_entity, position, opt_velocity, opt_collider_mvt, opt_cast) in
        &query_entities_to_move
    {
        if let Some(mut velocity) = opt_velocity {
            if let Some(target) = velocity.get_target() {
                if is_oob(&area_config, &position.current) && !game_entity.pending_despwan {
                    let mut game_entity_mut =
                        unsafe { query_entities_to_move.get_unchecked(entity).unwrap() };
                    game_entity_mut.1.pending_despwan = true;
                } else if position.current == *target {
                    continue;
                }

                let mut new_position_current = position
                    .current
                    .move_toward(*target, velocity.get_speed() * time.delta);

                let mut collide_with_blocking_entity = false;
                // Only apply collision with others entities if the entity we attempt to move also have a collider
                if let Some(collider_mvt) = opt_collider_mvt {
                    'outer: for (
                        entity_blocking,
                        _,
                        position_blocking,
                        _,
                        opt_collider_mvt_blocking,
                        _,
                    ) in &query_entities_to_move
                    {
                        if let Some(collider_mvt_blocking) = opt_collider_mvt_blocking {
                            if entity_blocking != entity {
                                let collide = collider_mvt.shape.collide(
                                    &new_position_current,
                                    &collider_mvt_blocking.shape,
                                    &position_blocking.current,
                                );
                                if collide {
                                    let collision_normal =
                                        collider_mvt.shape.collision_normal(&position.current);

                                    // Calculate the adjusted target position, resolving the collision by sliding
                                    let adjusted_target = position.current
                                        + (position.current - position_blocking.current) // Direction away from collision
                                        .reject_from_normalized(collision_normal) // Slide along the surface
                                        .normalize() // Normalize the sliding direction
                                        * 30.0; // Account for the moving shape's size

                                    // Move toward the adjusted target directly
                                    let adjusted_position = position
                                        .current
                                        .move_toward(adjusted_target, velocity.get_speed());

                                    if !collider_mvt.shape.collide(
                                        &adjusted_position,
                                        &collider_mvt_blocking.shape,
                                        &position_blocking.current,
                                    ) {
                                        new_position_current = adjusted_position
                                    } else {
                                        collide_with_blocking_entity = true;
                                    }

                                    break 'outer;
                                }
                            }
                        }
                    }
                }

                if !collide_with_blocking_entity {
                    let mut position_mut =
                        unsafe { query_entities_to_move.get_unchecked(entity).unwrap() };
                    position_mut.2.current = new_position_current;
                    if let Some(target) = velocity.get_target() {
                        if position.current == *target {
                            writer.send(VelocityReachedTarget {
                                entity,
                                target: *target,
                            });
                            let mut velocity_mut =
                                unsafe { query_entities_to_move.get_unchecked(entity).unwrap() };
                            velocity_mut.3.unwrap().remove_current_target();
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

pub fn inc_revision_updated_component(
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

pub fn inc_revision_removed_component(
    mut cast_removed: RemovedComponents<Cast>,
    mut query: Query<&mut GameEntity>,
) {
    for entity in cast_removed.read() {
        if let Ok(mut game_entity) = query.get_mut(entity) {
            game_entity.revision += 1;
        }
    }
}

pub fn on_update_velocity_target(
    mut reader: EventReader<UpdateVelocityTarget>,
    mut query: Query<&mut Velocity>,
    area_config: Res<AreaConfig>,
) {
    for event in reader.read() {
        if let Ok(mut velocity) = query.get_mut(event.entity) {
            velocity.set_target(&area_config, event.target);
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
    area_config: Res<AreaConfig>,
) {
    for event in reader.read() {
        if let Ok(mut velocity) = query.get_mut(event.entity) {
            velocity.add_target(&area_config, event.target);
        }
    }
}

pub fn on_update_position_current(
    mut reader: EventReader<UpdatePositionCurrent>,
    mut query: Query<(Entity, &mut Position, Option<&mut Velocity>)>,
    mut writer: EventWriter<VelocityReachedTarget>,
    area_config: Res<AreaConfig>,
) {
    for event in reader.read() {
        if let Ok((entity, mut position, opt_velocity)) = query.get_mut(event.entity) {
            position.current = event.current;

            if let Some(mut velocity) = opt_velocity {
                if event.force_update_velocity_target {
                    velocity.set_target(&area_config, None);
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
