use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rand::Rng;
use rust_common::collisions::collide_rect_to_rect;

use crate::{
    game::{
        bundles::prelude::*, components::prelude::*, events::prelude::*, resources::prelude::*,
    },
    utils::get_game_time,
};

const GRID_SIZE_X_MIN: f32 = -1024.0;
const GRID_SIZE_X_MAX: f32 = 1024.0;
const GRID_SIZE_Y_MIN: f32 = -1024.0;
const GRID_SIZE_Y_MAX: f32 = 1024.0;

fn wolrd_bounded_x(x: f32) -> f32 {
    f32::min(f32::max(GRID_SIZE_X_MIN, x), GRID_SIZE_X_MAX)
}

fn world_bounded_y(y: f32) -> f32 {
    f32::min(f32::max(GRID_SIZE_Y_MIN, y), GRID_SIZE_Y_MAX)
}

fn world_bounded_vector2(v: Vector2) -> Vector2 {
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

pub fn movement(
    mut query: Query<(Entity, &mut GameEntity, &mut Position, &Velocity)>,
    mut writer: EventWriter<VelocityReachedTarget>,
    time: Res<Time>,
) {
    for (entity, mut game_entity, mut position, velocity) in &mut query {
        if is_oob(&position.current) && !game_entity.pending_despwan {
            game_entity.pending_despwan = true;
        } else if position.current == *velocity.get_target() {
            continue;
        }

        let new_position_current = position
            .current
            .move_toward(*velocity.get_target(), velocity.get_speed() * time.delta);
        position.current = new_position_current;
        if position.current == *velocity.get_target() {
            writer.send(VelocityReachedTarget {
                entity,
                target: *velocity.get_target(),
            })
        }
    }
}

pub fn damage_on_hit(
    mut query_damage_source: Query<(&mut DamageOnHit, &Position, &Shape)>,
    mut query_damageable: Query<(Entity, &mut GameEntity, &mut Health, &Position, &Shape)>,
) {
    for (mut dmg_on_hit, dmg_on_hit_position, dmg_on_hit_shape) in &mut query_damage_source {
        for (
            damageable_entity,
            mut damageable_game_entity,
            mut damageable_health,
            damageable_position,
            damageable_shape,
        ) in &mut query_damageable
        {
            if collide_rect_to_rect(
                &dmg_on_hit_shape.rect,
                &dmg_on_hit_position.current,
                &damageable_shape.rect,
                &damageable_position.current,
            ) && dmg_on_hit.ignored_entity != damageable_entity
                && dmg_on_hit.hitted_entities.get(&damageable_entity).is_none()
            {
                if dmg_on_hit.damage_value < damageable_health.current {
                    damageable_health.current -= dmg_on_hit.damage_value;
                } else {
                    damageable_health.current = 0;
                }
                if damageable_health.current == 0 {
                    damageable_game_entity.pending_despwan = true;
                }
                dmg_on_hit.hitted_entities.insert(damageable_entity, true);
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
        if !game_entity.pending_despwan
            && velocity.get_despawn_at_target()
            && position.current == *velocity.get_target()
        {
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

pub fn enemies_spawner(mut enemies_state: ResMut<EnemiesState>, mut command: Commands) {
    let current_game_time = get_game_time();
    if enemies_state.last_spawn_at_millis + enemies_state.spwan_every_millis < current_game_time {
        enemies_state.last_spawn_at_millis = current_game_time;
        let random = rand::thread_rng().gen_range(GRID_SIZE_X_MIN..GRID_SIZE_X_MAX - 1.0);
        let position_current = match rand::thread_rng().gen_range(0..4) {
            0 => Vector2::new(GRID_SIZE_X_MIN, random),
            1 => Vector2::new(GRID_SIZE_X_MAX, random),
            2 => Vector2::new(random, GRID_SIZE_Y_MIN),
            3 => Vector2::new(random, GRID_SIZE_Y_MAX),
            _ => panic!("Unexpected value"),
        };
        command.spawn(EnemyBundle::new(position_current));
    }
}

pub fn on_update_velocity_target(
    mut reader: EventReader<UpdateVelocityTarget>,
    mut query: Query<(&mut Velocity, &Position)>,
) {
    for event in reader.read() {
        if let Ok((mut velocity, position)) = query.get_mut(event.entity) {
            velocity.set_target(position.current, world_bounded_vector2(event.target));
        }
    }
}

pub fn on_update_position_current(
    mut reader: EventReader<UpdatePositionCurrent>,
    mut query: Query<(&mut Position, Option<&mut Velocity>)>,
) {
    for event in reader.read() {
        if let Ok((mut position, opt_velocity)) = query.get_mut(event.entity) {
            position.current = world_bounded_vector2(event.current);

            if let Some(mut velocity) = opt_velocity {
                velocity.set_target(position.current, world_bounded_vector2(event.current));
            }
        }
    }
}

pub fn on_spawn_projectile(
    mut command: Commands,
    mut reader: EventReader<SpawnProjectile>,
    mut query: Query<(&Position, Option<&mut Velocity>)>,
) {
    for event in reader.read() {
        command.spawn(ProjectileBundle::new(
            event.from_position,
            event.to_target,
            event.ignored_entity,
        ));
        if let Ok((position, Some(mut velocity))) = query.get_mut(event.from_entity) {
            velocity.set_target(position.current, position.current);
        }
    }
}

pub fn on_spawn_frozen_orb(
    mut command: Commands,
    mut reader: EventReader<SpawnFrozenOrb>,
    mut query: Query<(&Position, Option<&mut Velocity>)>,
) {
    for event in reader.read() {
        command.spawn(FrozenOrbMainProjectileBundle::new(
            event.from_position,
            event.to_target,
            event.ignored_entity,
        ));
        if let Ok((position, Some(mut velocity))) = query.get_mut(event.from_entity) {
            velocity.set_target(position.current, position.current);
        }
    }
}

pub fn on_frozen_orb_velocity_reached_target(
    mut query: Query<(
        Entity,
        &FrozenOrbMainProjectile,
        &mut GameEntity,
        &Position,
        &DamageOnHit,
    )>,
    mut reader: EventReader<VelocityReachedTarget>,
    mut writer: EventWriter<SpawnProjectile>,
) {
    for event in reader.read() {
        if let Ok((entity, _, mut game_entity, position, damage_on_hit)) =
            query.get_mut(event.entity)
        {
            writer.send_batch(vec![
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x + 200.0,
                        y: position.current.y,
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x - 200.0,
                        y: position.current.y,
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x,
                        y: position.current.y + 200.0,
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x,
                        y: position.current.y - 200.0,
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x + (200.0 * 0.75),
                        y: position.current.y - (200.0 * 0.75),
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x + (200.0 * 0.75),
                        y: position.current.y + (200.0 * 0.75),
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x - (200.0 * 0.75),
                        y: position.current.y + (200.0 * 0.75),
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x - (200.0 * 0.75),
                        y: position.current.y - (200.0 * 0.75),
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
            ]);
            game_entity.pending_despwan = true;
        }
    }
}
