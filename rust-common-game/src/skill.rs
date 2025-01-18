use std::time::Duration;

use crate::mana::Mana;
use crate::utils::xor_u64s;
use bevy::{prelude::*, utils::HashMap};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug, Copy, Reflect)]
#[repr(u64)]
pub enum SkillName {
    BowAttack = 1,
    SplitArrow = 2,
}
impl SkillName {
    /// You could use the `strum` crate to derive this automatically!
    pub fn variants() -> impl Iterator<Item = SkillName> {
        use SkillName::*;
        [BowAttack, SplitArrow].iter().copied()
    }
}

#[derive(Event)]
pub struct TriggerSkillEvent {
    pub initiator: Entity,
    pub skill: Entity,
    pub target: Vec2,
}

#[derive(Event)]
pub struct ExcecuteSkillEvent {
    pub initiator: Entity,
    pub skill: Entity,
    pub target: Vec2,
    pub skill_instance_hash: u64,
}

#[derive(Event)]
pub struct AttachSkillEvent {
    pub to: Entity,
    pub skill_name: SkillName,
}

pub struct SkillData {
    pub cooldown: Duration,
    pub cost: Option<SkillCost>,
    pub projectile: Option<SkillProjectile>,
    pub damage_on_hit: Option<SkillDamageOnHit>,
}

#[derive(Component, Default, Deref)]
pub struct SkillInstanceHash(pub u64);

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
                cooldown: Duration::from_millis(200),
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
                cooldown: Duration::from_millis(200),
                cost: Some(SkillCost { mana: 8. }),
                projectile: Some(SkillProjectile {
                    count: 3.,
                    pierce_count: 2,
                }),
                damage_on_hit: Some(SkillDamageOnHit { value: 7. }),
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

        skill.insert(SkillCooldown {
            timer: Timer::new(skill_data.cooldown, TimerMode::Once),
        });

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
    mut trigger_skill_ev: EventReader<TriggerSkillEvent>,
    mut excecute_skill_ev: EventWriter<ExcecuteSkillEvent>,
    mut skill_q: Query<(&Skill, &mut SkillCooldown, Option<&SkillCost>), With<Skill>>,
    mut initiator_q: Query<&mut Mana, Without<Skill>>,
) {
    for event in trigger_skill_ev.read() {
        let Ok((skill, mut skill_cooldown, skill_cost)) = skill_q.get_mut(event.skill) else {
            println!("[on_trigger_skill_event] Cannot find skill entity");
            continue;
        };

        if skill_cooldown.timer.finished() {
            skill_cooldown.timer.reset();
        } else {
            continue;
        }

        let Ok(mut initiator_mana) = initiator_q.get_mut(event.initiator) else {
            println!("[on_trigger_skill_event] Cannot find initiator entity");
            continue;
        };

        if let Some(skill_cost) = skill_cost {
            let mana_after_use = initiator_mana.current - skill_cost.mana;
            if mana_after_use < 0. {
                continue;
            }
            initiator_mana.current = mana_after_use;
        }

        let skill_instance_hash = xor_u64s(&[skill.0 as u64, tick_manager.tick().0 as u64]);

        excecute_skill_ev.send(ExcecuteSkillEvent {
            initiator: event.initiator,
            skill: event.skill,
            target: event.target,
            skill_instance_hash,
        });
    }
}
