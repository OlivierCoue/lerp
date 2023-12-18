use bevy_ecs::prelude::*;
use rust_common::collisions::collide_rect_to_rect;

use crate::game::components::prelude::*;

pub fn damage_on_hit(
    mut query_damage_source: Query<(&mut DamageOnHit, &Position, &Shape)>,
    mut query_damageable: Query<(Entity, &mut GameEntity, &mut Health, &Position, &Shape)>,
) {
    for (mut dmg_on_hit, dmg_on_hit_position, dmg_on_hit_shape) in &mut query_damage_source {
        for (
            damageable_entity,
            mut damageable_game_entity,
            mut damageable_health,
            damageable_position,
            damageable_shape,
        ) in &mut query_damageable
        {
            if collide_rect_to_rect(
                &dmg_on_hit_shape.rect,
                &dmg_on_hit_position.current,
                &damageable_shape.rect,
                &damageable_position.current,
            ) && dmg_on_hit.ignored_entity != damageable_entity
                && dmg_on_hit.hitted_entities.get(&damageable_entity).is_none()
            {
                if dmg_on_hit.damage_value < damageable_health.current {
                    damageable_health.current -= dmg_on_hit.damage_value;
                } else {
                    damageable_health.current = 0;
                }
                if damageable_health.current == 0 {
                    damageable_game_entity.pending_despwan = true;
                }
                dmg_on_hit.hitted_entities.insert(damageable_entity, true);
            }
        }
    }
}
