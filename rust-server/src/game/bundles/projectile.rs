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
    team: Team,
}
impl ProjectileBundle {
    pub fn new(position_current: Vector2, velocity_target: Vector2, team: Team) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::PROJECTILE),
            projectile: Projectile::default(),
            position: Position::new(position_current),
            velocity: Velocity::new(Some(velocity_target), 1000.0, true),
            collider_dmg_in: ColliderDmgIn::new(Vector2 { x: 50.0, y: 50.0 }),
            damage_on_hit: DamageOnHit {
                despawn_after_first_apply: false,
                damage_value: 5,
                hitted_entities: HashMap::new(),
            },
            team,
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
    pub team: Team,
}
impl FrozenOrbMainProjectileBundle {
    pub fn new(position_current: Vector2, velocity_target: Vector2, team: Team) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::PROJECTILE),
            frozen_orb_main_projectile: FrozenOrbMainProjectile::default(),
            position: Position::new(position_current),
            velocity: Velocity::new(Some(velocity_target), 1000.0, false),
            collider_dmg_in: ColliderDmgIn::new(Vector2 { x: 50.0, y: 50.0 }),
            damage_on_hit: DamageOnHit {
                despawn_after_first_apply: false,
                damage_value: 5,
                hitted_entities: HashMap::new(),
            },
            team,
        }
    }
}

#[derive(Bundle)]
pub struct MeleeAttackBundle {
    pub game_entity: GameEntity,
    pub position: Position,
    pub damage_on_hit: DamageOnHit,
    pub collider_dmg_in: ColliderDmgIn,
    pub team: Team,
}
impl MeleeAttackBundle {
    pub fn new(position_current: Vector2, team: Team) -> Self {
        Self {
            game_entity: GameEntity::new(GameEntityBaseType::MELEE_ATTACK),
            position: Position::new(position_current),
            collider_dmg_in: ColliderDmgIn::new(Vector2 { x: 30.0, y: 30.0 }),
            damage_on_hit: DamageOnHit {
                despawn_after_first_apply: true,
                damage_value: 5,
                hitted_entities: HashMap::new(),
            },
            team,
        }
    }
}
