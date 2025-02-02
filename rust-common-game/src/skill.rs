use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug, Copy, Reflect)]
#[repr(u64)]
pub enum SkillName {
    BowAttack = 1,
    SplitArrow = 2,
    FlowerArrow = 3,
}
impl SkillName {
    /// You could use the `strum` crate to derive this automatically!
    pub fn variants() -> impl Iterator<Item = SkillName> {
        use SkillName::*;
        [BowAttack, SplitArrow, FlowerArrow].iter().copied()
    }
}

#[derive(Event)]
pub struct TriggerSkillEvent {
    pub initiator: Entity,
    pub skill: Entity,
    pub target: Vec2,
}

#[derive(Event, Clone, Copy)]
pub struct ExcecuteSkillEvent {
    pub initiator: Entity,
    pub skill: Entity,
    pub target: Vec2,
    pub skill_instance_hash: u64,
}

pub struct SkillData {
    pub cooldown: Option<Duration>,
    pub cost: Option<SkillCost>,
    pub projectile: Option<SkillProjectile>,
    pub damage_on_hit: Option<SkillDamageOnHit>,
}

#[derive(Component, Default, Deref)]
pub struct SkillInstanceHash(pub u64);

#[derive(Component)]
pub struct SkillInProgress {
    pub timer: Timer,
    pub excecute_sent: bool,
    pub excecute_skill_event: ExcecuteSkillEvent,
}

#[derive(Component)]
pub struct SkillSpeed {
    pub value: Duration,
}

#[derive(Component, Deref, DerefMut)]
pub struct Skill(pub SkillName);

#[derive(Component, Clone, Copy)]
pub struct SkillCost {
    mana: f32,
}

#[derive(Component, Clone)]
pub struct SkillCooldown {
    timer: Timer,
}

#[derive(Component, Clone, Copy)]
pub struct SkillProjectile {
    pub count: f32,
    pub pierce_count: u32,
}

#[derive(Component, Clone, Copy)]
pub struct SkillDamageOnHit {
    pub value: f32,
}

#[derive(Resource, Deref)]
pub struct SkillDb {
    map: HashMap<SkillName, SkillData>,
}
impl Default for SkillDb {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            SkillName::BowAttack,
            SkillData {
                cooldown: None,
                cost: Some(SkillCost { mana: 6. }),
                projectile: Some(SkillProjectile {
                    count: 1.,
                    pierce_count: 0,
                }),
                damage_on_hit: Some(SkillDamageOnHit { value: 10. }),
            },
        );
        map.insert(
            SkillName::SplitArrow,
            SkillData {
                cooldown: None,
                cost: Some(SkillCost { mana: 8. }),
                projectile: Some(SkillProjectile {
                    count: 3.,
                    pierce_count: 2,
                }),
                damage_on_hit: Some(SkillDamageOnHit { value: 7. }),
            },
        );
        map.insert(
            SkillName::FlowerArrow,
            SkillData {
                cooldown: Some(Duration::from_millis(3000)),
                cost: Some(SkillCost { mana: 30. }),
                projectile: Some(SkillProjectile {
                    count: 20.,
                    pierce_count: 99,
                }),
                damage_on_hit: Some(SkillDamageOnHit { value: 100. }),
            },
        );
        Self { map }
    }
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct SkillsAvailable {
    pub map: HashMap<SkillName, Entity>,
}

#[derive(Component)]
pub struct DamageOnHit {
    pub value: f32,
}

#[derive(Component)]
pub struct Pierce {
    pub count: u32,
}

pub fn attach_skill(
    commands: &mut Commands,
    to: Entity,
    skills_available: &mut SkillsAvailable,
    skill_name: &SkillName,
    skill_db: &SkillDb,
) {
    let skill_data = skill_db.get(skill_name).unwrap();
    commands.entity(to).with_children(|parent| {
        let mut skill = parent.spawn((Skill(*skill_name),));

        if let Some(cooldown) = skill_data.cooldown {
            skill.insert(SkillCooldown {
                timer: Timer::new(cooldown, TimerMode::Once),
            });
        }

        if let Some(cost) = skill_data.cost {
            skill.insert(cost);
        }

        if let Some(projectile) = skill_data.projectile {
            skill.insert(projectile);
        }

        if let Some(damage_on_hit) = skill_data.damage_on_hit {
            skill.insert(damage_on_hit);
        }

        skills_available.insert(*skill_name, skill.id());
    });
}

pub fn attach_all_skills(
    commands: &mut Commands,
    to: Entity,
    skills_available: &mut SkillsAvailable,
    skill_db: &SkillDb,
) {
    for skill_name in SkillName::variants() {
        attach_skill(commands, to, skills_available, &skill_name, skill_db);
    }
}

pub fn progress_skill_cooldown_timers(
    time: Res<Time<Fixed>>,
    mut skill_cooldown_q: Query<&mut SkillCooldown>,
) {
    for mut skill_cooldown in skill_cooldown_q.iter_mut() {
        skill_cooldown.timer.tick(time.delta());
    }
}

pub fn on_trigger_skill_event(
    tick_manager: Res<TickManager>,
    mut commands: Commands,
    mut trigger_skill_ev: EventReader<TriggerSkillEvent>,
    mut skill_q: Query<(&Skill, Option<&mut SkillCooldown>, Option<&SkillCost>), With<Skill>>,
    mut initiator_q: Query<(Entity, &SkillSpeed, &mut Mana, Has<SkillInProgress>), Without<Skill>>,
) {
    for event in trigger_skill_ev.read() {
        let Ok((skill, skill_cooldown, skill_cost)) = skill_q.get_mut(event.skill) else {
            println!("[on_trigger_skill_event] Cannot find skill entity");
            continue;
        };

        let Ok((
            initiator_entity,
            initiator_skill_speed,
            mut initiator_mana,
            initiator_has_skill_in_progress,
        )) = initiator_q.get_mut(event.initiator)
        else {
            println!("[on_trigger_skill_event] Cannot find initiator entity");
            continue;
        };

        // 1. Check if the initiator can trigger the skill

        // Check that no other skill is already in progress
        if initiator_has_skill_in_progress {
            continue;
        }

        // Check that the skill is not in cooldown
        if let Some(skill_cooldown) = &skill_cooldown {
            if !skill_cooldown.timer.finished() {
                continue;
            }
        }

        // Check that the initiator as enouth resources (and save values)
        let mut mana_after_use = initiator_mana.current;
        if let Some(skill_cost) = skill_cost {
            mana_after_use = initiator_mana.current - skill_cost.mana;
            if mana_after_use < 0. {
                continue;
            }
        }

        // 2. Actually trigger the skill

        // Start cooldown
        if let Some(mut skill_cooldown) = skill_cooldown {
            skill_cooldown.timer.reset();
        }

        // Consume resources (from saved values)
        if skill_cost.is_some() {
            initiator_mana.current = mana_after_use;
        }

        let skill_instance_hash = xor_u64s(&[skill.0 as u64, tick_manager.tick().0 as u64]);

        commands.entity(initiator_entity).insert(SkillInProgress {
            timer: Timer::new(initiator_skill_speed.value, TimerMode::Once),
            excecute_sent: false,
            excecute_skill_event: ExcecuteSkillEvent {
                initiator: event.initiator,
                skill: event.skill,
                target: event.target,
                skill_instance_hash,
            },
        });
    }
}

pub fn progress_skill_in_progress_timers(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut excecute_skill_ev: EventWriter<ExcecuteSkillEvent>,
    mut skill_in_progress_q: Query<(Entity, &mut SkillInProgress)>,
) {
    for (entity, mut skill_in_progress) in skill_in_progress_q.iter_mut() {
        skill_in_progress.timer.tick(time.delta());

        if !skill_in_progress.excecute_sent
            && skill_in_progress.timer.remaining_secs()
                <= skill_in_progress.timer.duration().as_secs_f32() * 0.66
        {
            excecute_skill_ev.send(skill_in_progress.excecute_skill_event);
            skill_in_progress.excecute_sent = true
        }

        if skill_in_progress.timer.finished() {
            commands.entity(entity).remove::<SkillInProgress>();
        }
    }
}
