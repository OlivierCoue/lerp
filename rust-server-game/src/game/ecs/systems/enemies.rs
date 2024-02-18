use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rand::Rng;
use rust_common::math::get_point_from_points_and_distance;

use crate::game::ecs::{
    bundles::prelude::*, components::prelude::*, events::prelude::*, resources::prelude::*,
};

pub fn enemies_spawner(
    mut enemies_state: ResMut<EnemiesState>,
    area_config: Res<AreaConfig>,
    time: Res<Time>,
    mut command: Commands,
) {
    if !enemies_state.is_enable() {
        return;
    }

    let current_game_time = time.current_millis;
    if enemies_state.last_spawn_at_millis == 0
        || enemies_state.last_spawn_at_millis + enemies_state.spwan_every_millis < current_game_time
    {
        enemies_state.last_spawn_at_millis = current_game_time;
        let random = rand::thread_rng().gen_range(0.0..area_config.area_width - 1.0);
        let position_current = match rand::thread_rng().gen_range(0..4) {
            0 => Vector2::new(0.0, random),
            1 => Vector2::new(area_config.area_width, random),
            2 => Vector2::new(random, 0.0),
            3 => Vector2::new(random, area_config.area_height),
            _ => panic!("Unexpected value"),
        };
        let is_wizard: u32 = rand::thread_rng().gen_range(0..2);
        command.spawn(EnemyBundle::new(position_current, is_wizard == 1));
    }
}

#[allow(clippy::type_complexity)]
pub fn enemies_ai(
    mut query_enemies: Query<
        (Entity, &mut Enemie, &Position, &Team),
        (With<Enemie>, Without<Player>),
    >,
    query_players: Query<&Position, (With<Player>, Without<Enemie>)>,
    mut writer_update_velocity_target_with_pathfinder: EventWriter<
        UpdateVelocityTargetWithPathFinder,
    >,
    mut writer_cast_spell: EventWriter<CastSpell>,
    time: Res<Time>,
) {
    let aggro_range = 700.0;
    for (enemy_entity, mut enemy, enemy_position, team) in &mut query_enemies {
        let current_game_time = time.current_millis;
        if enemy.last_action_at_millis != 0
            && enemy.last_action_at_millis + 1000 > current_game_time
        {
            continue;
        }

        enemy.last_action_at_millis = current_game_time;
        let mut opt_closest_player_location = None;
        let mut closest_player_distance = 0.0;

        for player_position in &query_players {
            let player_distance = enemy_position.current.distance_to(player_position.current);
            if player_distance <= aggro_range
                && (opt_closest_player_location.is_none()
                    || player_distance < closest_player_distance)
            {
                opt_closest_player_location = Some(player_position.current);
                closest_player_distance = player_distance;
            }
        }

        if let Some(closest_player_location) = opt_closest_player_location {
            if !enemy.is_wizard {
                if closest_player_distance <= 40.0 {
                    writer_cast_spell.send(CastSpell {
                        from_entity: enemy_entity,
                        spell: Spell::MeleeAttack(
                            enemy_entity,
                            get_point_from_points_and_distance(
                                enemy_position.current,
                                closest_player_location,
                                40.0,
                            ),
                            *team,
                        ),
                    })
                } else {
                    writer_update_velocity_target_with_pathfinder.send(
                        UpdateVelocityTargetWithPathFinder {
                            entity: enemy_entity,
                            target: closest_player_location,
                        },
                    )
                }
            } else if closest_player_distance <= 400.0 {
                writer_cast_spell.send(CastSpell {
                    from_entity: enemy_entity,
                    spell: Spell::Projectile(
                        enemy_entity,
                        enemy_position.current,
                        get_point_from_points_and_distance(
                            enemy_position.current,
                            closest_player_location,
                            400.0,
                        ),
                        *team,
                    ),
                })
            } else {
                writer_update_velocity_target_with_pathfinder.send(
                    UpdateVelocityTargetWithPathFinder {
                        entity: enemy_entity,
                        target: get_point_from_points_and_distance(
                            closest_player_location,
                            enemy_position.current,
                            350.0,
                        ),
                    },
                )
            }
        }
    }
}
