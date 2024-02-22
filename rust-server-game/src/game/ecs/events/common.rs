use bevy_ecs::prelude::*;
use rust_common::math::Vec2;

use crate::game::ecs::components::prelude::*;

#[derive(Event)]
pub struct UpdateVelocityTarget {
    pub entity: Entity,
    pub target: Option<Vec2>,
}

#[derive(Event)]
pub struct UpdateVelocityTargetWithPathFinder {
    pub entity: Entity,
    pub target: Vec2,
}

#[derive(Event)]
pub struct AddVelocityTarget {
    pub entity: Entity,
    pub target: Vec2,
}

#[derive(Event)]
pub struct UpdatePositionCurrent {
    pub entity: Entity,
    pub current: Vec2,
    pub force_update_velocity_target: bool,
}

#[derive(Event)]
pub struct VelocityReachedTarget {
    pub entity: Entity,
    pub target: Vec2,
}

#[derive(Event)]
pub struct CastSpell {
    pub from_entity: Entity,
    pub spell: Spell,
}
