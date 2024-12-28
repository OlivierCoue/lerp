mod enemy;
mod map;
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
        Targets(Vec::new()),
        RigidBody::Kinematic,
        CharacterController,
        Collider::circle(ENTITY_SIZE / 2.0),
        LockedAxes::ROTATION_LOCKED,
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
            Targets(Vec::new()),
            RigidBody::Kinematic,
            CharacterController,
            Collider::circle(ENTITY_SIZE / 2.0),
            LockedAxes::ROTATION_LOCKED,
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

fn movement(
    time: Res<Time<Physics>>,
    mut query: Query<(&Position, &mut Targets, &mut LinearVelocity)>,
) {
    for (position, targets, velocity) in &mut query {
        shared_movement_behaviour(&time, position, velocity, targets);
    }
}

fn set_player_target(mut query: Query<(&ActionState<PlayerActions>, &mut Targets), With<Player>>) {
    for (action, mut targets) in query.iter_mut() {
        if action.pressed(&PlayerActions::Move) {
            let Some(cursor_position) = action.dual_axis_data(&PlayerActions::Cursor) else {
                println!("[set_player_target] cursor_position not set skipping");
                return;
            };

            *targets = Targets(vec![Vec2::new(
                cursor_position.pair.x,
                cursor_position.pair.y,
            )])
        }
    }
}

fn aplly_auto_move(
    mut query: Query<&mut Targets, With<AutoMove>>,
    time: Res<Time>,
    mut config: ResMut<AutoMoveConfig>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        config.direction = -config.direction;
        for mut targets in &mut query {
            *targets = Targets(vec![Vec2::new(1000. * config.direction, 0.)])
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
        .add_systems(FixedUpdate, (movement, set_player_target).chain())
        .add_plugins(EnemyPlugin)
        .run();
}
