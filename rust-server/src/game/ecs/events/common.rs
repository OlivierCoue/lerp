use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

use crate::game::ecs::components::prelude::*;

#[derive(Event)]
pub struct UpdateVelocityTarget {
    pub entity: Entity,
    pub target: Option<Vector2>,
}

#[derive(Event)]
pub struct UpdateVelocityTargetWithPathFinder {
    pub entity: Entity,
    pub target: Vector2,
}

#[derive(Event)]
pub struct AddVelocityTarget {
    pub entity: Entity,
    pub target: Vector2,
}

#[derive(Event)]
pub struct UpdatePositionCurrent {
    pub entity: Entity,
    pub current: Vector2,
    pub force_update_velocity_target: bool,
}

#[derive(Event)]
pub struct VelocityReachedTarget {
    pub entity: Entity,
    pub target: Vector2,
}

#[derive(Event)]
pub struct CastSpell {
    pub from_entity: Entity,
    pub spell: Spell,
}
