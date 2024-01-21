use crate::game::bundles::prelude::*;
use crate::game::components::prelude::*;
use crate::game::events::prelude::*;
use crate::utils::get_game_time;
use bevy_ecs::prelude::*;

pub fn on_cast_spell(
    mut command: Commands,
    mut reader: EventReader<CastSpell>,
    mut query: Query<(Entity, Option<&mut Velocity>), Without<Cast>>,
) {
    for event in reader.read() {
        if let Ok((_, opt_velocity)) = query.get_mut(event.from_entity) {
            command
                .entity(event.from_entity)
                .insert(Cast::new(event.spell, get_game_time() + 200));

            if let Some(mut velocity) = opt_velocity {
                velocity.set_target(None);
            }
        }
    }
}

pub fn create_casted_spells(mut command: Commands, query: Query<(Entity, &Cast), With<Cast>>) {
    let now = get_game_time();
    for (entity, cast) in &query {
        if now < cast.end_at_millis {
            continue;
        }

        match cast.spell {
            Spell::FrozenOrb(_, from_position, to_target, ignored_entity) => {
                command.spawn(FrozenOrbMainProjectileBundle::new(
                    from_position,
                    to_target,
                    ignored_entity,
                ));
            }
            Spell::Projectile(_, _, _, _) => {
                println!("unsupported")
            }
        }
        command.entity(entity).remove::<Cast>();
    }
}
