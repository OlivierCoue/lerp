use crate::common::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use lightyear::prelude::client::*;
use rust_common_game::{
    projectile::Projectile,
    protocol::*,
    shared::{ENEMY_SIZE, PLAYER_SIZE, PROJECTILE_SIZE},
};

use super::PlaySceneTag;

#[derive(Component)]
pub struct DebugCollider;

#[derive(Component)]
pub struct DebugColliderEntityRef(pub Entity);

pub(crate) fn debug_draw_colliders(
    debug_config: Res<DebugConfig>,
    render_config: Res<RenderConfig>,
    mut commands: Commands,
    collider_q: Query<
        (
            Entity,
            &Position,
            &Collider,
            Option<&DebugColliderEntityRef>,
        ),
        Without<DebugCollider>,
    >,
    mut collider_debug_q: Query<&mut Transform, With<DebugCollider>>,
) {
    if !debug_config.show_colliders {
        return;
    }

    for (entity, position, collider, debug_entity_ref) in collider_q.iter() {
        if let Some(debug_entity_ref) = debug_entity_ref {
            if let Ok(mut transform) = collider_debug_q.get_mut(debug_entity_ref.0) {
                transform.translation = apply_render_mode(&render_config, position).extend(3.);
            }
        } else {
            if let Some(ball) = collider.shape().as_ball() {
                let shape = shapes::Ellipse {
                    radii: apply_render_mode_radius(&render_config, ball.radius),
                    center: Vec2::ZERO,
                };

                let debug_entity = commands
                    .spawn((
                        PlaySceneTag,
                        DebugCollider,
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_translation(
                                apply_render_mode(&render_config, position).extend(3.),
                            ),
                            ..default()
                        },
                        Stroke::new(Color::linear_rgb(0., 1., 0.), 2.0),
                        DebugColliderEntityRef(entity),
                    ))
                    .id();
                commands
                    .entity(entity)
                    .insert(DebugColliderEntityRef(debug_entity));
            }

            if let Some(cuboid) = collider.shape().as_cuboid() {
                let top_left = Vec2::new(-cuboid.half_extents.x, cuboid.half_extents.y);
                let top_right = Vec2::new(cuboid.half_extents.x, cuboid.half_extents.y);
                let bottom_right = Vec2::new(cuboid.half_extents.x, -cuboid.half_extents.y);
                let bottom_left = Vec2::new(-cuboid.half_extents.x, -cuboid.half_extents.y);

                let shape = shapes::Polygon {
                    points: vec![
                        apply_render_mode(&render_config, &top_left),
                        apply_render_mode(&render_config, &top_right),
                        apply_render_mode(&render_config, &bottom_right),
                        apply_render_mode(&render_config, &bottom_left),
                    ],
                    closed: true,
                };

                let debug_entity = commands
                    .spawn((
                        PlaySceneTag,
                        DebugCollider,
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_translation(
                                apply_render_mode(&render_config, position).extend(3.),
                            ),
                            ..default()
                        },
                        Stroke::new(Color::linear_rgb(0., 1., 0.), 2.0),
                        DebugColliderEntityRef(entity),
                    ))
                    .id();
                commands
                    .entity(entity)
                    .insert(DebugColliderEntityRef(debug_entity));
            }
        }
    }
}

pub(crate) fn debug_undraw_colliders(
    debug_config: Res<DebugConfig>,
    mut commands: Commands,
    collider_q: Query<Entity, (With<Collider>, Without<DebugCollider>)>,
    collider_debug_q: Query<(Entity, &DebugColliderEntityRef), With<DebugCollider>>,
) {
    if !debug_config.show_colliders {
        return;
    }

    for (debug_confirmed_entity, confirmed_entity_ref) in collider_debug_q.iter() {
        if collider_q.get(confirmed_entity_ref.0).is_err() {
            commands.entity(debug_confirmed_entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct DebugConfirmedEntity;

#[derive(Component)]
pub struct DebugConfirmedEntityRef(pub Entity);

pub(crate) fn debug_draw_confirmed_entities(
    debug_config: Res<DebugConfig>,
    render_config: Res<RenderConfig>,
    mut commands: Commands,
    confirmed_q: Query<
        (
            Entity,
            &Position,
            Has<Enemy>,
            Has<Projectile>,
            Option<&DebugConfirmedEntityRef>,
        ),
        (With<Confirmed>, Without<DebugConfirmedEntity>),
    >,
    mut confirmed_debug_q: Query<&mut Transform, With<DebugConfirmedEntity>>,
) {
    if !debug_config.show_confirmed_entities {
        return;
    }

    for (entity, position, is_enemy, is_projectile, debug_entity_ref) in confirmed_q.iter() {
        if let Some(debug_entity_ref) = debug_entity_ref {
            if let Ok(mut transform) = confirmed_debug_q.get_mut(debug_entity_ref.0) {
                transform.translation = apply_render_mode(&render_config, position).extend(4.);
            }
        } else {
            let radius = if is_enemy {
                ENEMY_SIZE / 2.
            } else if is_projectile {
                PROJECTILE_SIZE / 2.
            } else {
                PLAYER_SIZE / 2.
            };

            let shape = shapes::Ellipse {
                radii: apply_render_mode_radius(&render_config, radius),
                center: Vec2::ZERO,
            };

            let debug_entity = commands
                .spawn((
                    PlaySceneTag,
                    DebugConfirmedEntity,
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shape),
                        transform: Transform::from_translation(
                            apply_render_mode(&render_config, position).extend(3.),
                        ),
                        ..default()
                    },
                    Stroke::new(Color::linear_rgb(1., 0., 0.), 2.0),
                    DebugConfirmedEntityRef(entity),
                ))
                .id();
            commands
                .entity(entity)
                .insert(DebugConfirmedEntityRef(debug_entity));
        }
    }
}

pub(crate) fn debug_undraw_confirmed_entities(
    debug_config: Res<DebugConfig>,
    mut commands: Commands,
    confirmed_q: Query<Entity, (With<Confirmed>, Without<DebugConfirmedEntity>)>,
    confirmed_debug_q: Query<(Entity, &DebugConfirmedEntityRef), With<DebugConfirmedEntity>>,
) {
    if !debug_config.show_confirmed_entities {
        return;
    }

    for (debug_confirmed_entity, confirmed_entity_ref) in confirmed_debug_q.iter() {
        if confirmed_q.get(confirmed_entity_ref.0).is_err() {
            commands.entity(debug_confirmed_entity).despawn();
        }
    }
}

pub(crate) fn _debug_draw_targets(
    mut gizmos: Gizmos,
    confirmed_q: Query<&MovementTargets, (With<Player>, With<Confirmed>)>,
    predicted_q: Query<&MovementTargets, (With<Player>, With<Predicted>)>,
    interpolated_q: Query<&MovementTargets, (With<Player>, With<Interpolated>)>,
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
