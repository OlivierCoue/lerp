use std::time::Duration;

use avian2d::prelude::*;
use bevy::{prelude::*, time::Timer};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use rust_common_game::{
    character_controller::CharacterController,
    protocol::*,
    shared::{ENEMY_BASE_MOVEMENT_SPEED, ENEMY_SIZE},
};

const ENEMY_MAX_COUNT: u32 = 50;

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
            Enemy,
            MovementTargets(Vec::new()),
            RigidBody::Kinematic,
            CharacterController,
            Collider::circle(ENEMY_SIZE / 2.),
            LockedAxes::ROTATION_LOCKED,
            MovementSpeed(ENEMY_BASE_MOVEMENT_SPEED),
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
        commands.spawn(player);
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyState {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            count: 0,
        });
        app.add_systems(FixedUpdate, spaw_enemy);
    }
}
