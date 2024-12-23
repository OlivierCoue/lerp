use std::time::Duration;

use avian2d::prelude::*;
use bevy::{prelude::*, time::Timer};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use rust_common_game::{
    character_controller::CharacterController, protocol::*, settings::ENTITY_SIZE, shared::FixedSet,
};

use crate::AutoMove;

const ENEMY_MAX_COUNT: u32 = 5;

#[derive(Resource)]
pub struct EnemyState {
    pub timer: Timer,
    pub count: u32,
}

fn spaw_enemy(mut commands: Commands, time: Res<Time>, mut enemy_state: ResMut<EnemyState>) {
    enemy_state.timer.tick(time.delta());

    if enemy_state.timer.finished() && enemy_state.count < ENEMY_MAX_COUNT {
        enemy_state.count += 1;
        let player = (
            Player {
                client_id: ClientId::Netcode(999999999),
            },
            Enemy,
            Targets(Vec::new()),
            RigidBody::Kinematic,
            CharacterController,
            Collider::circle(ENTITY_SIZE / 2.0),
            LockedAxes::ROTATION_LOCKED,
            Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::None,
                    interpolation: NetworkTarget::All,
                },
                target: ReplicationTarget {
                    target: NetworkTarget::All,
                },
                controlled_by: ControlledBy {
                    target: NetworkTarget::None,
                    ..default()
                },
                group: REPLICATION_GROUP,
                ..default()
            },
        );
        commands.spawn(player);
    }
}

#[allow(clippy::type_complexity)]
fn set_enemy_target(
    mut query_enemies: Query<&mut Targets, With<Enemy>>,
    query_players: Query<&mut Position, (With<Player>, Without<Enemy>, Without<AutoMove>)>,
) {
    let mut nearest_player_position_opt = None;
    for player_position in &query_players {
        nearest_player_position_opt = Some(&player_position.0);
    }

    let Some(nearest_player_position) = nearest_player_position_opt else {
        return;
    };

    for mut enemy_target in &mut query_enemies {
        enemy_target.0 = vec![*nearest_player_position];
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyState {
            timer: Timer::new(Duration::from_secs(4), TimerMode::Repeating),
            count: 0,
        });
        app.add_systems(
            FixedUpdate,
            (spaw_enemy, set_enemy_target).in_set(FixedSet::Main),
        );
    }
}
