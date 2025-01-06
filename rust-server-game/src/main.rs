mod enemy;
mod map;
mod projectile;

use std::net::SocketAddr;
use std::time::Duration;

use avian2d::prelude::*;

use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::utils::HashMap;
use enemy::EnemyPlugin;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use local_ip_address::local_ip;

use map::setup_map;
use rust_common_game::character_controller::*;
use rust_common_game::projectile::move_projectiles;
use rust_common_game::projectile::PreviousPosition;
use rust_common_game::projectile::Projectile;
use rust_common_game::projectile::ProjectileData;
use rust_common_game::protocol::*;
use rust_common_game::settings::*;
use rust_common_game::shared::*;

#[derive(Resource, Default)]
pub struct ClientPlayerMap(HashMap<ClientId, Entity>);

#[derive(Component)]
pub struct AutoMove;

#[derive(Resource)]
pub struct AutoMoveConfig {
    pub timer: Timer,
    pub direction: f32,
}

fn start_server(mut commands: Commands) {
    println!("Starting server...");
    commands.start_server();
    let player = (
        Player {
            client_id: ClientId::Netcode(999999999),
        },
        MovementTargets(Vec::new()),
        RigidBody::Kinematic,
        CharacterController,
        Collider::circle(PLAYER_SIZE / 2.0),
        LockedAxes::ROTATION_LOCKED,
        MovementSpeed(PLAYER_BASE_MOVEMENT_SPEED),
        AutoMove,
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

fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
    mut client_player_map: ResMut<ClientPlayerMap>,
) {
    for connection in connections.read() {
        let client_id = connection.client_id;
        info!("New client {:?}", client_id);
        let player = (
            Player { client_id },
            MovementTargets(Vec::new()),
            RigidBody::Kinematic,
            CharacterController,
            Collider::circle(PLAYER_SIZE / 2.0),
            LockedAxes::ROTATION_LOCKED,
            MovementSpeed(PLAYER_BASE_MOVEMENT_SPEED),
            Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::Single(client_id),
                    interpolation: NetworkTarget::AllExceptSingle(client_id),
                },
                target: ReplicationTarget {
                    target: NetworkTarget::All,
                },
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                    ..default()
                },
                group: REPLICATION_GROUP,
                ..default()
            },
        );
        let player = commands.spawn(player);
        client_player_map.0.insert(client_id, player.id());
    }
}

fn move_to_target(
    time: Res<Time<Physics>>,
    mut query: Query<(
        &Position,
        &mut MovementTargets,
        &mut LinearVelocity,
        &MovementSpeed,
    )>,
) {
    for (position, targets, velocity, movement_speed) in &mut query {
        shared_move_to_target_behaviour(&time, position, movement_speed, velocity, targets);
    }
}

fn handle_move_click(
    mut query: Query<(&ActionState<PlayerActions>, &mut MovementTargets), With<Player>>,
) {
    for (action, targets) in query.iter_mut() {
        shared_handle_move_click_behavior(action, targets);
    }
}

#[allow(clippy::type_complexity)]
pub fn handle_move_wasd(
    mut query: Query<
        (
            &ActionState<PlayerActions>,
            &MovementSpeed,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    for (action, movement_speed, velocity) in query.iter_mut() {
        shared_handle_move_wasd_behavior(action, movement_speed, velocity);
    }
}

fn handle_skill_slot(
    tick_manager: Res<TickManager>,
    mut commands: Commands,
    mut query: Query<(&Player, &ActionState<PlayerActions>, &Position), With<ReplicationTarget>>,
) {
    for (player, action, player_position) in query.iter_mut() {
        if action.pressed(&PlayerActions::SkillSlot1) {
            let Some(cursor_position) = action.dual_axis_data(&PlayerActions::Cursor) else {
                println!("cursor_position not set skipping");
                return;
            };
            let direction = (cursor_position.pair - player_position.0).normalize();
            let velocity = direction * PROJECTILE_BASE_MOVEMENT_SPEED;
            // println!(
            //     "projectile tick: {} count: {} cursor: {}:{} p position: {}:{}",
            //     tick_manager.tick().0,
            //     projectile_stats.fired_count,
            //     cursor_position.pair.x,
            //     cursor_position.pair.y,
            //     player_position.x,
            //     player_position.y
            // );
            commands.spawn((
                Projectile,
                ProjectileData {
                    max_distance: 10. * PIXEL_METER,
                    distance_traveled: 0.,
                },
                RigidBody::Kinematic,
                Collider::circle(PROJECTILE_SIZE / 2.),
                LockedAxes::ROTATION_LOCKED,
                PreviousPosition(player_position.0),
                Position::from_xy(player_position.x, player_position.y),
                LinearVelocity(velocity),
                PreSpawnedPlayerObject::new(tick_manager.tick().0 as u64 + 65_535 + 1),
                Replicate {
                    sync: SyncTarget {
                        prediction: NetworkTarget::Single(player.client_id),
                        interpolation: NetworkTarget::AllExceptSingle(player.client_id),
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
            ));
        }
    }
}

fn aplly_auto_move(
    mut query: Query<&mut MovementTargets, With<AutoMove>>,
    time: Res<Time>,
    mut config: ResMut<AutoMoveConfig>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        config.direction = -config.direction;
        for mut targets in &mut query {
            *targets = MovementTargets(vec![Vec2::new(1000. * config.direction, 0.)])
        }
    }
}

fn main() {
    let server_addr = SocketAddr::new(local_ip().unwrap().to_canonical(), 34255);

    let netcode_config = NetcodeConfig::default().with_protocol_id(0).with_key([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

    // let link_conditioner = LinkConditionerConfig {
    //     incoming_latency: Duration::from_millis(100),
    //     incoming_jitter: Duration::from_millis(0),
    //     incoming_loss: 0.00,
    // };

    let net_config = NetConfig::Netcode {
        config: netcode_config,
        io: IoConfig {
            transport: ServerTransport::UdpSocket(server_addr),
            ..default()
        },
    };

    let server_config = server::ServerConfig {
        shared: shared_config(Mode::Separate),
        net: vec![net_config],
        replication: ReplicationConfig {
            send_interval: REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    };
    let server_plugin = server::ServerPlugins::new(server_config);

    App::new()
        .add_plugins((MinimalPlugins, StatesPlugin))
        .add_plugins(LogPlugin {
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
            ..default()
        })
        .add_plugins(server_plugin.build())
        .add_plugins(SharedPlugin)
        .init_resource::<ClientPlayerMap>()
        .insert_resource(AutoMoveConfig {
            timer: Timer::new(Duration::from_secs(4), TimerMode::Repeating),
            direction: 1.,
        })
        .add_systems(Startup, (start_server, setup_map))
        .add_systems(Update, handle_connections)
        .add_systems(FixedUpdate, aplly_auto_move)
        .add_systems(
            FixedUpdate,
            (
                move_to_target,
                handle_move_wasd,
                handle_move_click,
                handle_skill_slot,
                move_projectiles,
            )
                .chain(),
        )
        .add_plugins(EnemyPlugin)
        .run();
}
