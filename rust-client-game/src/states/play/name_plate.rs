use crate::states::play::*;
use bevy::prelude::*;
use lightyear::prelude::PreSpawnedPlayerObject;
use rust_common_game::{
    death::Dead, health::Health, mana::Mana, shared::PIXEL_METER, skill::SkillInProgress,
};

#[derive(Component)]
pub struct HasNamePlate(pub Entity);

#[derive(Component)]
pub struct NamePlate;

#[derive(Component)]
pub struct HealthBar(pub Entity);

#[derive(Component)]
pub struct HealthBarBackground;

#[derive(Component)]
pub struct ManaBar(pub Entity);

#[derive(Component)]
pub struct ManaBarBackground;

#[derive(Component)]
pub struct SkillInProgressBar(pub Entity);

const NAME_PLATE_Y_OFFSET: f32 = 2.3 * PIXEL_METER;
const RESOURCE_BAR_MAX_WIDTH: f32 = 50.;

const HEALTH_BAR_HEIGHT: f32 = 5.;
const MANA_BAR_HEIGHT: f32 = 4.;
const SKILL_IN_PROGRESS_BAR_HEIGHT: f32 = 2.;

fn get_resource_bar_width(current: f32, max: f32) -> f32 {
    let health_percentage = current / max;
    RESOURCE_BAR_MAX_WIDTH * health_percentage
}

fn get_resource_bar_translation_x(width: f32) -> f32 {
    -(RESOURCE_BAR_MAX_WIDTH / 2.0) + (width / 2.0)
}

pub fn remove_name_plate(
    mut commands: Commands,
    health_query: Query<
        (Entity, &HasNamePlate),
        (
            With<Dead>,
            Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>,
        ),
    >,
) {
    for (entity, name_plate) in health_query.iter() {
        commands.entity(name_plate.0).despawn_recursive();
        commands.entity(entity).remove::<HasNamePlate>();
    }
}

pub fn add_name_plate(
    mut commands: Commands,
    health_query: Query<
        (Entity, &Health, Option<&Mana>),
        (
            Without<HasNamePlate>,
            Without<Dead>,
            With<Health>,
            Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>,
        ),
    >,
) {
    for (entity, health, mana) in health_query.iter() {
        let name_plate = commands
            .spawn((NamePlate, Visibility::default(), Transform::default()))
            .with_children(|parent| {
                let health_bar_width = get_resource_bar_width(health.current, health.max);
                let health_bar_translation_x = get_resource_bar_translation_x(health_bar_width);

                let health_bar_translation_y_offset =
                    if mana.is_some() { MANA_BAR_HEIGHT } else { 0. };
                let health_bar_translation_y =
                    NAME_PLATE_Y_OFFSET + health_bar_translation_y_offset;
                parent.spawn((
                    HealthBar(entity),
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
                        custom_size: Some(Vec2::new(RESOURCE_BAR_MAX_WIDTH, HEALTH_BAR_HEIGHT)),
                        ..default()
                    },
                    Transform::from_xyz(0., health_bar_translation_y, 0.),
                ));

                if let Some(mana) = mana {
                    let mana_bar_width = get_resource_bar_width(mana.current, mana.max);
                    let mana_bar_translation_x = get_resource_bar_translation_x(mana_bar_width);

                    let mana_bar_translation_y = NAME_PLATE_Y_OFFSET;

                    parent.spawn((
                        ManaBar(entity),
                        Sprite {
                            color: Color::srgb_u8(27, 62, 141),
                            custom_size: Some(Vec2::new(mana_bar_width, MANA_BAR_HEIGHT)),
                            ..default()
                        },
                        Transform::from_xyz(mana_bar_translation_x, mana_bar_translation_y, 1.),
                    ));
                    parent.spawn((
                        ManaBarBackground,
                        Sprite {
                            color: Color::srgb_u8(0, 0, 16),
                            custom_size: Some(Vec2::new(RESOURCE_BAR_MAX_WIDTH, MANA_BAR_HEIGHT)),
                            ..default()
                        },
                        Transform::from_xyz(0., mana_bar_translation_y, 0.),
                    ));

                    let skill_in_progress_bar_translation_y = NAME_PLATE_Y_OFFSET - MANA_BAR_HEIGHT;

                    parent.spawn((
                        SkillInProgressBar(entity),
                        Sprite {
                            color: Color::srgb_u8(141, 141, 141),
                            custom_size: Some(Vec2::new(0., SKILL_IN_PROGRESS_BAR_HEIGHT)),
                            ..default()
                        },
                        Transform::from_xyz(0., skill_in_progress_bar_translation_y, 1.),
                    ));
                }
            })
            .id();

        commands
            .entity(entity)
            .insert(HasNamePlate(name_plate))
            .add_child(name_plate);
    }
}

pub fn update_health_bar(
    health_q: Query<&Health, (Changed<Health>, With<Predicted>, Without<HealthBar>)>,
    mut health_bar_q: Query<(&mut Sprite, &mut Transform, &HealthBar), With<HealthBar>>,
) {
    for (mut sprite, mut transform, parent) in health_bar_q.iter_mut() {
        if let Ok(health) = health_q.get(parent.0) {
            let health_bar_width = get_resource_bar_width(health.current, health.max);
            let health_bar_translation_x = get_resource_bar_translation_x(health_bar_width);
            sprite.custom_size = Some(Vec2::new(health_bar_width, HEALTH_BAR_HEIGHT));
            transform.translation.x = health_bar_translation_x;
        }
    }
}

pub fn update_mana_bar(
    mana_q: Query<&Mana, (Changed<Mana>, With<Predicted>, Without<ManaBar>)>,
    mut mana_bar_q: Query<(&mut Sprite, &mut Transform, &ManaBar), With<ManaBar>>,
) {
    for (mut sprite, mut transform, parent) in mana_bar_q.iter_mut() {
        if let Ok(mana) = mana_q.get(parent.0) {
            let mana_bar_width = get_resource_bar_width(mana.current, mana.max);
            let mana_bar_translation_x = get_resource_bar_translation_x(mana_bar_width);
            sprite.custom_size = Some(Vec2::new(mana_bar_width, MANA_BAR_HEIGHT));
            transform.translation.x = mana_bar_translation_x;
        }
    }
}

pub fn update_skill_in_progress_bar(
    skill_in_progress_q: Query<&SkillInProgress, (With<Predicted>, Without<SkillInProgressBar>)>,
    mut skill_in_progress_bar_q: Query<
        (&mut Sprite, &mut Transform, &SkillInProgressBar),
        With<SkillInProgressBar>,
    >,
) {
    for (mut sprite, mut transform, parent) in skill_in_progress_bar_q.iter_mut() {
        if let Ok(mana) = skill_in_progress_q.get(parent.0) {
            let skill_in_progress_bar_width = get_resource_bar_width(
                mana.timer.elapsed_secs(),
                mana.timer.duration().as_secs_f32(),
            );
            let mana_bar_translation_x =
                get_resource_bar_translation_x(skill_in_progress_bar_width);
            sprite.custom_size = Some(Vec2::new(
                skill_in_progress_bar_width,
                SKILL_IN_PROGRESS_BAR_HEIGHT,
            ));
            transform.translation.x = mana_bar_translation_x;
        } else {
            sprite.custom_size = Some(Vec2::new(0., SKILL_IN_PROGRESS_BAR_HEIGHT));
        }
    }
}
