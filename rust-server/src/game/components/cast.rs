use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

#[derive(Debug, Copy, Clone)]
pub enum Spell {
    FrozenOrb(Entity, Vector2, Vector2, Entity),
    Projectile(Entity, Vector2, Vector2, Entity),
    MeleeAttack(Entity, Vector2, Entity),
}

#[derive(Component)]
pub struct Cast {
    pub spell: Spell,
    pub end_at_millis: u32,
}
impl Cast {
    pub fn new(spell: Spell, end_at_millis: u32) -> Self {
        Self {
            spell,
            end_at_millis,
        }
    }
}
