use crate::states::play::*;
use bevy::prelude::*;
use rust_common_game::{health::Health, shared::PIXEL_METER};

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthBarBackground;

const HEALTH_BAR_WIDTH: f32 = 50.;
const HEALTH_BAR_HEIGHT: f32 = 5.;

fn get_health_bar_width(health: &Health) -> f32 {
    let health_percentage = health.current / health.max;
    HEALTH_BAR_WIDTH * health_percentage
}

fn get_health_bar_translation_x(width: f32) -> f32 {
    -(HEALTH_BAR_WIDTH / 2.0) + (width / 2.0)
}

pub fn handle_new_health_bar(
    mut commands: Commands,
    health_query: Query<(Entity, &Health), (Added<Health>, With<Predicted>)>,
) {
    for (entity, health) in health_query.iter() {
        commands.entity(entity).with_children(|parent| {
            let health_bar_width = get_health_bar_width(health);
            let health_bar_translation_x = get_health_bar_translation_x(health_bar_width);
            let health_bar_translation_y = 2.3 * PIXEL_METER;
            parent.spawn((
                HealthBar,
                Sprite {
                    color: Color::srgb_u8(110, 20, 20),
                    custom_size: Some(Vec2::new(health_bar_width, HEALTH_BAR_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(health_bar_translation_x, health_bar_translation_y, 1.),
            ));
            parent.spawn((
                HealthBarBackground,
                Sprite {
                    color: Color::srgb_u8(16, 0, 0),
                    custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(0., health_bar_translation_y, 0.),
            ));
        });
    }
}

pub fn update_health_bar(
    health_q: Query<&Health, (Changed<Health>, With<Predicted>, Without<HealthBar>)>,
    mut health_bar_q: Query<(&mut Sprite, &mut Transform, &Parent), With<HealthBar>>,
) {
    for (mut sprite, mut transform, parent) in health_bar_q.iter_mut() {
        if let Ok(health) = health_q.get(parent.get()) {
            let health_bar_width = get_health_bar_width(health);
            let health_bar_translation_x = get_health_bar_translation_x(health_bar_width);
            sprite.custom_size = Some(Vec2::new(health_bar_width, HEALTH_BAR_HEIGHT));
            transform.translation.x = health_bar_translation_x;
        }
    }
}
