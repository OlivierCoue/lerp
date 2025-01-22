use std::time::Duration;

use bevy::{prelude::*, time::Timer};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use rust_common_game::{enemy::EnemyBundle, protocol::*};

const ENEMY_MAX_COUNT: u32 = 50;

#[derive(Resource)]
pub struct EnemyState {
    pub timer: Timer,
    pub count: u32,
}

fn on_spawn_enemies_message(
    mut events: EventReader<MessageEvent<SpawnEnemies>>,
    mut enemy_state: ResMut<EnemyState>,
) {
    for _ in events.read() {
        enemy_state.count = 0;
    }
}

fn spaw_enemy(mut commands: Commands, time: Res<Time>, mut enemy_state: ResMut<EnemyState>) {
    enemy_state.timer.tick(time.delta());

    if enemy_state.timer.finished() && enemy_state.count < ENEMY_MAX_COUNT {
        enemy_state.count += 1;
        let enemy = (
            EnemyBundle::new(&Vec2::new(0., 0.)),
            Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::All,
                    interpolation: NetworkTarget::None,
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
        commands.spawn(enemy);
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyState {
            timer: Timer::new(Duration::from_millis(10), TimerMode::Repeating),
            count: ENEMY_MAX_COUNT,
        });
        app.add_systems(Update, on_spawn_enemies_message);
        app.add_systems(FixedUpdate, spaw_enemy);
    }
}
