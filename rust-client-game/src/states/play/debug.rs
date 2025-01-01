use crate::common::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use lightyear::prelude::client::*;
use rust_common_game::{protocol::*, settings::*};

use super::PlaySceneTag;

pub fn debug_draw_colliders(
    mut commands: Commands,
    query: Query<(Entity, &Collider), Added<Collider>>,
    render_config: Res<RenderConfig>,
) {
    for (entity, collider) in query.iter() {
        if let Some(ball) = collider.shape().as_ball() {
            let radii_y = match render_config.mode {
                RenderMode::Iso => ball.radius * (2.0f32.sqrt() / 2.0),
                RenderMode::Cart => ball.radius,
            };

            let shape = shapes::Ellipse {
                radii: Vec2::new(ball.radius, radii_y),
                center: Vec2::ZERO,
            };

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shape),
                        transform: Transform::from_translation(Vec2::ZERO.extend(3.)),
                        ..default()
                    },
                    Stroke::new(Color::linear_rgb(0., 1., 0.), 2.0),
                ));
            });
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

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shape),
                        transform: Transform::from_translation(Vec2::ZERO.extend(3.)),
                        ..default()
                    },
                    Stroke::new(Color::linear_rgb(0., 1., 0.), 2.0),
                ));
            });
        }
    }
}

#[derive(Component)]
pub struct DebugComfirmedEntity;

#[derive(Component)]
pub struct DebugComfirmedEntityRef(pub Entity);

#[allow(clippy::type_complexity)]
pub(crate) fn debug_draw_confirmed_entities(
    mut commands: Commands,
    confirmed_q: Query<
        (
            Entity,
            &Position,
            Has<Enemy>,
            Option<&DebugComfirmedEntityRef>,
        ),
        (With<Player>, With<Confirmed>, Without<DebugComfirmedEntity>),
    >,
    mut confirmed_debug_q: Query<&mut Transform, With<DebugComfirmedEntity>>,
    render_config: Res<RenderConfig>,
) {
    for (entity, position, is_enemy, debug_entity_ref) in confirmed_q.iter() {
        if let Some(debug_entity_ref) = debug_entity_ref {
            if let Ok(mut transform) = confirmed_debug_q.get_mut(debug_entity_ref.0) {
                transform.translation = apply_render_mode(&render_config, position).extend(3.);
            }
        } else {
            let radius = if is_enemy {
                ENTITY_SIZE / 2.0 / 2.
            } else {
                ENTITY_SIZE / 2.
            };

            let radii_y = match render_config.mode {
                RenderMode::Iso => radius * (2.0f32.sqrt() / 2.0),
                RenderMode::Cart => radius,
            };

            let shape = shapes::Ellipse {
                radii: Vec2::new(radius, radii_y),
                center: Vec2::ZERO,
            };

            let debug_entity = commands
                .spawn((
                    PlaySceneTag,
                    DebugComfirmedEntity,
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shape),
                        transform: Transform::from_translation(
                            apply_render_mode(&render_config, position).extend(3.),
                        ),
                        ..default()
                    },
                    Stroke::new(Color::linear_rgb(1., 0., 0.), 2.0),
                ))
                .id();
            commands
                .entity(entity)
                .insert(DebugComfirmedEntityRef(debug_entity));
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn debug_draw_targets(
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
