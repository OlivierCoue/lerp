use avian2d::{parry::utils::hashmap::HashMap, prelude::LinearVelocity};
use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StatKind {
    SkillSpeed,
    MovementSpeed,
}
impl StatKind {
    fn variants() -> [StatKind; 2] {
        [StatKind::SkillSpeed, StatKind::MovementSpeed]
    }
}

pub enum StatModifierKind {
    Flat,
    Increase,
    More,
}

#[derive(Component, Default)]
pub struct StatsModifiers {
    flat_map: HashMap<StatKind, f32>,
    increase_map: HashMap<StatKind, f32>,
    more_map: HashMap<StatKind, f32>,
}
impl StatsModifiers {
    pub fn reset(&mut self) {
        self.flat_map.clear();
        self.increase_map.clear();
        self.more_map.clear();
    }

    pub fn get_flat(&self, kind: &StatKind) -> f32 {
        (1. + *self.flat_map.get(kind).unwrap_or(&0.)).max(0.)
    }
    pub fn add_flat(&mut self, kind: StatKind, value: f32) {
        *self.flat_map.entry(kind).or_insert(0.) += value;
    }

    pub fn get_increase(&self, kind: &StatKind) -> f32 {
        (1. + *self.increase_map.get(kind).unwrap_or(&0.)).max(0.)
    }
    pub fn add_increase(&mut self, kind: StatKind, value: f32) {
        *self.increase_map.entry(kind).or_insert(0.) += value;
    }

    pub fn get_more(&self, kind: &StatKind) -> f32 {
        (*self.more_map.get(kind).unwrap_or(&1.)).max(0.)
    }

    pub fn add_more(&mut self, kind: StatKind, value: f32) {
        let entry = self.more_map.entry(kind).or_insert(1.);

        if value > 0. {
            *entry *= 1. + value;
        } else if value < 0. {
            *entry /= 1. + value.abs();
        }
    }

    pub fn apply(&self, base_value: f32, kind: &StatKind) -> f32 {
        (base_value + self.get_flat(kind)) * self.get_increase(kind) * self.get_more(kind)
    }
}

pub enum StatCondition {
    SkillInProgress,
    Stationary,
}

impl StatCondition {
    fn is_met(&self, entity: Entity, world: &World) -> bool {
        match &self {
            Self::SkillInProgress => world.get::<SkillInProgress>(entity).is_some(),
            Self::Stationary => world
                .get::<LinearVelocity>(entity)
                .is_some_and(|linear_velocity| linear_velocity.0 == Vec2::ZERO),
        }
    }
}

pub struct ConditionalStatModifierSource {
    pub stat: StatKind,
    pub modifier: f32,
    pub modifier_type: StatModifierKind,
    pub conditions: Vec<StatCondition>,
}

#[derive(Component)]
pub struct StatModifierSources {
    pub conditionals: Vec<ConditionalStatModifierSource>,
}

pub fn compute_stats_modifier(
    mut commands: Commands,
    world: &World,
    mut target_q: Query<(Entity, &StatModifierSources)>,
) {
    for (entity, stats_modifier_sources) in target_q.iter_mut() {
        let mut stats_modifier = StatsModifiers::default();

        for source in &stats_modifier_sources.conditionals {
            let all_conditions_met = source
                .conditions
                .iter()
                .all(|condition| condition.is_met(entity, world));

            if all_conditions_met {
                match source.modifier_type {
                    StatModifierKind::Flat => stats_modifier.add_flat(source.stat, source.modifier),
                    StatModifierKind::Increase => {
                        stats_modifier.add_increase(source.stat, source.modifier)
                    }
                    StatModifierKind::More => stats_modifier.add_more(source.stat, source.modifier),
                }
            }
        }

        commands.entity(entity).insert(stats_modifier);
    }
}

pub fn apply_stats_modifier(mut target_q: Query<(&StatsModifiers, Option<&mut MovementSpeed>)>) {
    for (stats_modifier, mut movement_speed) in target_q.iter_mut() {
        for stat_kind in StatKind::variants() {
            match stat_kind {
                StatKind::MovementSpeed => {
                    if let Some(movement_speed) = movement_speed.as_deref_mut() {
                        movement_speed.current =
                            stats_modifier.apply(movement_speed.base, &stat_kind)
                    }
                }
                StatKind::SkillSpeed => {}
            }
        }
    }
}

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (compute_stats_modifier, apply_stats_modifier)
                .chain()
                .in_set(GameSimulationSet::ComputeStats),
        );
    }
}
