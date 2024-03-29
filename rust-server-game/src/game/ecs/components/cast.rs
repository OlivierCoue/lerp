use bevy_ecs::prelude::*;

use rust_common::{
    helper::vec2_to_point,
    math::Vec2,
    proto::{UdpCast, UdpSpell},
};

use crate::game::ecs::{components::prelude::Team, resources::prelude::Time};

#[derive(Debug, Copy, Clone)]
pub enum Spell {
    FrozenOrb(Entity, Vec2, Vec2, Team),
    Projectile(Entity, Vec2, Vec2, Team),
    MeleeAttack(Entity, Vec2, Team),
}

#[derive(Component)]
pub struct Cast {
    pub spell: Spell,
    pub end_at_millis: u32,
    pub duration_millis: u32,
}
impl Cast {
    pub fn new(spell: Spell, time: &Time, duration_millis: u32) -> Self {
        Self {
            spell,
            end_at_millis: time.current_millis + duration_millis,
            duration_millis,
        }
    }

    pub fn to_proto(&self) -> UdpCast {
        let target = match self.spell {
            Spell::FrozenOrb(_, _, target, _) => target,
            Spell::Projectile(_, _, target, _) => target,
            Spell::MeleeAttack(_, target, _) => target,
        };

        UdpCast {
            spell: UdpSpell::SpellMeleeAttack.into(),
            target: Some(vec2_to_point(&target)),
            duration: self.duration_millis,
        }
    }
}
