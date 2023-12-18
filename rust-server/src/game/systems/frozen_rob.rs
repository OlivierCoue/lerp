use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

use crate::game::{bundles::prelude::*, components::prelude::*, events::prelude::*};

pub fn on_spawn_frozen_orb(
    mut command: Commands,
    mut reader: EventReader<SpawnFrozenOrb>,
    mut query: Query<Option<&mut Velocity>>,
) {
    for event in reader.read() {
        command.spawn(FrozenOrbMainProjectileBundle::new(
            event.from_position,
            event.to_target,
            event.ignored_entity,
        ));
        if let Ok(Some(mut velocity)) = query.get_mut(event.from_entity) {
            velocity.set_target(None);
        }
    }
}

pub fn on_frozen_orb_velocity_reached_target(
    mut query: Query<(
        Entity,
        &FrozenOrbMainProjectile,
        &mut GameEntity,
        &Position,
        &DamageOnHit,
    )>,
    mut reader: EventReader<VelocityReachedTarget>,
    mut writer: EventWriter<SpawnProjectile>,
) {
    for event in reader.read() {
        if let Ok((entity, _, mut game_entity, position, damage_on_hit)) =
            query.get_mut(event.entity)
        {
            writer.send_batch(vec![
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x + 200.0,
                        y: position.current.y,
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x - 200.0,
                        y: position.current.y,
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x,
                        y: position.current.y + 200.0,
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x,
                        y: position.current.y - 200.0,
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x + (200.0 * 0.75),
                        y: position.current.y - (200.0 * 0.75),
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x + (200.0 * 0.75),
                        y: position.current.y + (200.0 * 0.75),
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x - (200.0 * 0.75),
                        y: position.current.y + (200.0 * 0.75),
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
                SpawnProjectile {
                    from_entity: entity,
                    from_position: event.target,
                    to_target: Vector2 {
                        x: position.current.x - (200.0 * 0.75),
                        y: position.current.y - (200.0 * 0.75),
                    },
                    ignored_entity: damage_on_hit.ignored_entity,
                },
            ]);
            game_entity.pending_despwan = true;
        }
    }
}
