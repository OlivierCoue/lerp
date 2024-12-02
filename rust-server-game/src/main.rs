use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::str::FromStr;

use avian2d::prelude::*;

use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use local_ip_address::local_ip;

use rust_common_game::protocol::*;
use rust_common_game::settings::*;
use rust_common_game::shared::*;

fn start_server(mut commands: Commands) {
    println!("Starting server...");
    commands.start_server();
}

fn handle_connections(mut connections: EventReader<ConnectEvent>, mut commands: Commands) {
    for connection in connections.read() {
        let client_id = connection.client_id;
        info!("New client {:?}", client_id);
        let player = (
            Player { client_id },
            Targets(Vec::new()),
            RigidBody::Dynamic,
            Collider::circle(ENTITY_SIZE / 2.0),
            LockedAxes::ROTATION_LOCKED,
            Restitution::new(1.0),
            Friction::new(0.0),
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
        commands.spawn(player);
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

fn set_player_target(
    mut input_reader: EventReader<InputEvent<Inputs>>,
    mut query: Query<&mut Targets, With<Player>>,
) {
    for input in input_reader.read() {
        if let Some(Inputs::Target(target)) = input.input() {
            let Ok(mut targets) = query.get_single_mut() else {
                return;
            };
            *targets = Targets(vec![Vec2::new(target.x, target.y)])
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
        .add_systems(Startup, start_server)
        .add_systems(Update, handle_connections)
        .add_systems(
            FixedUpdate,
            (movement, set_player_target).chain().in_set(FixedSet::Main),
        )
        .run();
}
