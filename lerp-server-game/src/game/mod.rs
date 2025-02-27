use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::common_conditions::on_timer;
use bevy::utils::HashMap;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::WyRand;
use item_drop::generate_item_dropped_on_death;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear::server::input::leafwing::InputSystemSet;
use local_ip_address::local_ip;
use lerp_common_game::input::PlayerActions;
use lerp_common_game::prelude::*;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

mod item_drop;

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
    map: Res<Map>,
) {
    for connection in connections.read() {
        let client_id = connection.client_id;
        info!("New client {:?}", client_id);

        let player_id = commands.spawn_empty().id();
        commands.entity(player_id).insert((
            PlayerBundle::new(&map.player_spawn_position),
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
    mut receive_inputs: ResMut<Events<ServerReceiveMessage<InputMessage<PlayerActions>>>>,
    mut send_inputs: EventWriter<ServerSendMessage<InputMessage<PlayerActions>>>,
) {
    // rebroadcast the input to other clients
    // we are calling drain() here so make sure that this system runs after the `ReceiveInputs` set,
    // so that the server had the time to process the inputs
    send_inputs.send_batch(receive_inputs.drain().map(|ev| {
        ServerSendMessage::new_with_target::<InputChannel>(
            ev.message,
            NetworkTarget::AllExceptSingle(ev.from),
        )
    }));
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

#[derive(Resource)]
struct ExitState {
    pub port: u16,
    pub lifetime: Duration,
    pub instance_exit_rx: oneshot::Receiver<bool>,
    pub instance_exit_tx: mpsc::Sender<u16>,
}

fn exit_listener_system(
    mut exit_state: ResMut<ExitState>,
    mut app_exit_event: EventWriter<AppExit>,
    player_q: Query<&Player>,
) {
    exit_state.lifetime += Duration::from_millis(100);

    if exit_state.instance_exit_rx.try_recv().is_ok()
        || (exit_state.lifetime > Duration::from_secs(10) && player_q.is_empty())
    {
        exit_state
            .instance_exit_tx
            .blocking_send(exit_state.port)
            .unwrap();
        app_exit_event.send(AppExit::Success);
    }
}

pub(crate) struct GameInstanceConfig {
    pub port: u16,
    pub exit_channel_rx: oneshot::Receiver<bool>,
    pub instance_exit_tx: mpsc::Sender<u16>,
}

pub(crate) fn start_game_world(config: GameInstanceConfig) {
    let server_addr = SocketAddr::new(local_ip().unwrap().to_canonical(), config.port);

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
        shared: shared_config(),
        net: vec![net_config],
        replication: ReplicationConfig {
            send_updates_mode: SendUpdatesMode::SinceLastAck,
        },
        ..default()
    };
    let server_plugin = server::ServerPlugins::new(server_config);

    App::new()
        .add_plugins((MinimalPlugins, StatesPlugin))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(server_plugin.build())
        .add_plugins(SharedPlugin)
        .init_resource::<ClientPlayerMap>()
        .insert_resource(ExitState {
            port: config.port,
            instance_exit_rx: config.exit_channel_rx,
            instance_exit_tx: config.instance_exit_tx,
            lifetime: Duration::ZERO,
        })
        .add_systems(Startup, start_server)
        .add_systems(OnEnter(NetworkingState::Started), generate_map)
        .add_systems(
            PreUpdate,
            replicate_inputs.after(InputSystemSet::ReceiveInputs),
        )
        .add_systems(
            Update,
            (
                handle_connections,
                update_player_client_metrics.run_if(on_timer(Duration::from_secs(1))),
                exit_listener_system.run_if(on_timer(Duration::from_millis(100))),
            ),
        )
        .add_systems(FixedUpdate, generate_item_dropped_on_death)
        .run();

    info!("start_game_world stopped");
}
