use bevy_ecs::prelude::*;
use rust_common::collisions::collide_rect_to_rect;

use crate::game::components::prelude::*;

#[allow(clippy::type_complexity)]
pub fn damage_on_hit(
    mut query_damage_source: Query<
        (
            &mut GameEntity,
            &mut DamageOnHit,
            &Position,
            &ColliderDmgIn,
            &Team,
        ),
        With<DamageOnHit>,
    >,
    mut query_damageable: Query<
        (
            Entity,
            &mut GameEntity,
            &mut Health,
            &Position,
            &ColliderDmgIn,
            &Team,
        ),
        Without<DamageOnHit>,
    >,
) {
    for (mut game_entity, mut dmg_on_hit, dmg_on_hit_position, dmg_on_hit_collider_dmg_in, team) in
        &mut query_damage_source
    {
        for (
            damageable_entity,
            mut damageable_game_entity,
            mut damageable_health,
            damageable_position,
            damageable_collider_dmg_in,
            damageable_team,
        ) in &mut query_damageable
        {
            if collide_rect_to_rect(
                &dmg_on_hit_collider_dmg_in.rect,
                &dmg_on_hit_position.current,
                &damageable_collider_dmg_in.rect,
                &damageable_position.current,
            ) && team != damageable_team
                && dmg_on_hit.hitted_entities.get(&damageable_entity).is_none()
            {
                if dmg_on_hit.damage_value < damageable_health.get_current() {
                    let current = damageable_health.get_current();
                    damageable_health.set_current(current - dmg_on_hit.damage_value);
                } else {
                    damageable_health.set_current(0);
                }
                if damageable_health.get_current() == 0 {
                    damageable_game_entity.pending_despwan = true;
                }
                dmg_on_hit.hitted_entities.insert(damageable_entity, true);
            }
        }
        if dmg_on_hit.despawn_after_first_apply {
            game_entity.pending_despwan = true;
        }
    }
}
