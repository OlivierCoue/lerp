use crate::game::ecs::bundles::prelude::*;
use crate::game::ecs::components::prelude::*;
use crate::game::ecs::events::prelude::*;
use crate::game::ecs::resources::prelude::*;
use bevy_ecs::prelude::*;

pub fn on_cast_spell(
    mut command: Commands,
    mut reader: EventReader<CastSpell>,
    mut query: Query<(Entity, Option<&mut Velocity>), Without<Cast>>,
    area_config: Res<AreaConfig>,
    time: Res<Time>,
) {
    for event in reader.read() {
        if let Ok((_, opt_velocity)) = query.get_mut(event.from_entity) {
            command
                .entity(event.from_entity)
                .insert(Cast::new(event.spell, &time, 200));

            if let Some(mut velocity) = opt_velocity {
                velocity.set_target(&area_config, None);
            }
        }
    }
}

pub fn create_casted_spells(
    mut command: Commands,
    query: Query<(Entity, &Cast), With<Cast>>,
    time: Res<Time>,
) {
    let now = time.current_millis;
    for (entity, cast) in &query {
        if now < cast.end_at_millis {
            continue;
        }

        match cast.spell {
            Spell::FrozenOrb(_, from_position, to_target, team) => {
                command.spawn(FrozenOrbMainProjectileBundle::new(
                    from_position,
                    to_target,
                    team,
                ));
            }
            Spell::Projectile(_, from_position, to_target, team) => {
                command.spawn(ProjectileBundle::new(from_position, to_target, team));
            }
            Spell::MeleeAttack(_, from_position, team) => {
                command.spawn(MeleeAttackBundle::new(from_position, team));
            }
        }
        command.entity(entity).remove::<Cast>();
    }
}
