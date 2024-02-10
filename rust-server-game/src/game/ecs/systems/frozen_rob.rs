use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

use crate::game::ecs::{bundles::prelude::*, components::prelude::*, events::prelude::*};

pub fn on_frozen_orb_velocity_reached_target(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &FrozenOrbMainProjectile,
        &mut GameEntity,
        &Position,
        &Team,
    )>,
    mut reader: EventReader<VelocityReachedTarget>,
) {
    for event in reader.read() {
        if let Ok((_, _, mut game_entity, position, team)) = query.get_mut(event.entity) {
            commands.spawn_batch(vec![
                ProjectileBundle::new(
                    event.target,
                    Vector2 {
                        x: position.current.x + 200.0,
                        y: position.current.y,
                    },
                    *team,
                ),
                ProjectileBundle::new(
                    event.target,
                    Vector2 {
                        x: position.current.x - 200.0,
                        y: position.current.y,
                    },
                    *team,
                ),
                ProjectileBundle::new(
                    event.target,
                    Vector2 {
                        x: position.current.x,
                        y: position.current.y + 200.0,
                    },
                    *team,
                ),
                ProjectileBundle::new(
                    event.target,
                    Vector2 {
                        x: position.current.x,
                        y: position.current.y - 200.0,
                    },
                    *team,
                ),
                ProjectileBundle::new(
                    event.target,
                    Vector2 {
                        x: position.current.x + (200.0 * 0.75),
                        y: position.current.y - (200.0 * 0.75),
                    },
                    *team,
                ),
                ProjectileBundle::new(
                    event.target,
                    Vector2 {
                        x: position.current.x + (200.0 * 0.75),
                        y: position.current.y + (200.0 * 0.75),
                    },
                    *team,
                ),
                ProjectileBundle::new(
                    event.target,
                    Vector2 {
                        x: position.current.x - (200.0 * 0.75),
                        y: position.current.y + (200.0 * 0.75),
                    },
                    *team,
                ),
                ProjectileBundle::new(
                    event.target,
                    Vector2 {
                        x: position.current.x - (200.0 * 0.75),
                        y: position.current.y - (200.0 * 0.75),
                    },
                    *team,
                ),
            ]);
            game_entity.pending_despwan = true;
        }
    }
}
