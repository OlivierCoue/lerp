use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

#[derive(Event)]
pub struct UpdateVelocityTarget {
    pub entity: Entity,
    pub target: Vector2,
}

#[derive(Event)]
pub struct UpdatePositionCurrent {
    pub entity: Entity,
    pub current: Vector2,
}

#[derive(Event)]
pub struct SpawnProjectile {
    pub from_entity: Entity,
    pub from_position: Vector2,
    pub to_target: Vector2,
    pub ignored_entity: Entity,
}

#[derive(Event)]
pub struct SpawnFrozenOrb {
    pub from_entity: Entity,
    pub from_position: Vector2,
    pub to_target: Vector2,
    pub ignored_entity: Entity,
}

#[derive(Event)]
pub struct VelocityReachedTarget {
    pub entity: Entity,
    pub target: Vector2,
}
