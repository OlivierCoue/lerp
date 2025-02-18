use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::common_conditions::on_timer;
use bevy::utils::HashMap;
use drop::generate_loop_on_death;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use local_ip_address::local_ip;
use rust_common_game::input::PlayerActions;

use rust_common_game::prelude::*;

use std::net::SocketAddr;
use std::time::Duration;

mod drop;

#[derive(Resource, Default)]
pub struct ClientPlayerMap(HashMap<ClientId, Entity>);

fn start_server(mut commands: Commands) {
    println!("Starting server...");
    commands.start_server();
}

fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
    mut client_player_map: ResMut<ClientPlayerMap>,
) {
    for connection in connections.read() {
        let client_id = connection.client_id;
        info!("New client {:?}", client_id);

        let player_id = commands.spawn_empty().id();
        commands.entity(player_id).insert((
            PlayerBundle::new(&Vec2::new(0., 0.)),
            Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::All,
                    interpolation: NetworkTarget::None,
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
        ));

        let player_client = (
            PlayerClient {
                client_id,
                rtt: Duration::ZERO,
                jitter: Duration::ZERO,
                player_ref: player_id,
            },
            Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::Single(client_id),
                    interpolation: NetworkTarget::None,
                },
                target: ReplicationTarget {
                    target: NetworkTarget::Single(client_id),
                },
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                    ..default()
                },
                group: REPLICATION_GROUP,
                ..default()
            },
        );
        commands.spawn(player_client);

        client_player_map.0.insert(client_id, player_id);
    }
}

fn replicate_inputs(
    mut connection: ResMut<ConnectionManager>,
    mut input_events: ResMut<Events<MessageEvent<InputMessage<PlayerActions>>>>,
) {
    for event in input_events.drain() {
        let client_id = event.from();

        // Optional: do some validation on the inputs to check that there's no cheating
        // Inputs for a specific tick should be write *once*. Don't let players change old inputs.

        // rebroadcast the input to other clients
        connection
            .send_message_to_target::<InputChannel, _>(
                &event.message,
                NetworkTarget::AllExceptSingle(client_id),
            )
            .unwrap()
    }
}

fn update_player_client_metrics(
    connection_manager: Res<ConnectionManager>,
    mut q: Query<(Entity, &mut PlayerClient)>,
) {
    for (_e, mut player_client) in q.iter_mut() {
        if let Ok(connection) = connection_manager.connection(player_client.client_id) {
            player_client.rtt = connection.rtt();
            player_client.jitter = connection.jitter();
        }
    }
}

fn main() {
    let server_addr = SocketAddr::new(local_ip().unwrap().to_canonical(), 34255);

    let netcode_config = NetcodeConfig::default().with_protocol_id(0).with_key([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

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
        .add_systems(Startup, (start_server, generate_map))
        .add_systems(
            PreUpdate,
            // this system will replicate the inputs of a client to other clients
            // so that a client can predict other clients
            replicate_inputs.after(MainSet::EmitEvents),
        )
        .add_systems(
            Update,
            (
                handle_connections,
                update_player_client_metrics.run_if(on_timer(Duration::from_secs(1))),
            ),
        )
        .add_systems(FixedUpdate, generate_loop_on_death)
        .run();
}
