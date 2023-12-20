use std::collections::HashMap;

use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::proto::common::GameEntityBaseType;

use crate::game::components::prelude::*;

#[derive(Component, Default)]
pub struct Projectile {}

#[derive(Bundle)]
pub struct ProjectileBundle {
    game_entity: GameEntity,
    projectile: Projectile,
    position: Position,
    velocity: Velocity,
    collider_dmg_in: ColliderDmgIn,
    damage_on_hit: DamageOnHit,
}
impl ProjectileBundle {
    pub fn new(
        position_current: Vector2,
        velocity_target: Vector2,
        ignored_entity: Entity,
    ) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::PROJECTILE),
            projectile: Projectile::default(),
            position: Position {
                current: position_current,
            },
            velocity: Velocity::new(Some(velocity_target), 1000.0, true),
            collider_dmg_in: ColliderDmgIn::new(Vector2 { x: 50.0, y: 50.0 }),
            damage_on_hit: DamageOnHit {
                damage_value: 5,
                hitted_entities: HashMap::new(),
                ignored_entity,
            },
        }
    }
}

#[derive(Component, Default)]
pub struct FrozenOrbMainProjectile {}

#[derive(Bundle)]
pub struct FrozenOrbMainProjectileBundle {
    pub game_entity: GameEntity,
    pub frozen_orb_main_projectile: FrozenOrbMainProjectile,
    pub position: Position,
    pub velocity: Velocity,
    pub collider_dmg_in: ColliderDmgIn,
    pub damage_on_hit: DamageOnHit,
}
impl FrozenOrbMainProjectileBundle {
    pub fn new(
        position_current: Vector2,
        velocity_target: Vector2,
        ignored_entity: Entity,
    ) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::PROJECTILE),
            frozen_orb_main_projectile: FrozenOrbMainProjectile::default(),
            position: Position {
                current: position_current,
            },
            velocity: Velocity::new(Some(velocity_target), 1000.0, false),
            collider_dmg_in: ColliderDmgIn::new(Vector2 { x: 50.0, y: 50.0 }),
            damage_on_hit: DamageOnHit {
                damage_value: 5,
                hitted_entities: HashMap::new(),
                ignored_entity,
            },
        }
    }
}
