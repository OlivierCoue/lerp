use bevy_ecs::prelude::*;

use crate::game::{bundles::prelude::*, components::prelude::*, events::prelude::*};

pub fn on_spawn_projectile(
    mut command: Commands,
    mut reader: EventReader<SpawnProjectile>,
    mut query: Query<Option<&mut Velocity>>,
) {
    for event in reader.read() {
        command.spawn(ProjectileBundle::new(
            event.from_position,
            event.to_target,
            event.ignored_entity,
        ));
        if let Ok(Some(mut velocity)) = query.get_mut(event.from_entity) {
            velocity.set_target(None);
        }
    }
}
