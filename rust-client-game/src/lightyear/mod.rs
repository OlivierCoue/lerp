use bevy::prelude::*;
use lightyear::prelude::{client::*, *};
use rand::Rng;
use rust_common_game::prelude::*;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

fn display_network_status(state: Res<State<NetworkingState>>) {
    if state.is_changed() {
        match state.get() {
            NetworkingState::Disconnected => {
                println!("NET: Disconnected");
            }
            NetworkingState::Connecting => {
                println!("NET: Connecting");
            }
            NetworkingState::Connected => {
                println!("NET: Connected");
            }
        };
    }
}

pub fn get_client_net_config(server_address: IpAddr) -> client::NetConfig {
    let mut rng = rand::rng();
    let client_id = rng.random_range(1..10001);

    // let link_conditioner = LinkConditionerConfig::good_condition();
    let link_conditioner = LinkConditionerConfig {
        incoming_latency: Duration::from_millis(0),
        incoming_jitter: Duration::from_millis(0),
        incoming_loss: 0.00,
    };

    // let server_addr = SocketAddr::new(IpAddr::from_str("15.237.150.220").unwrap(), 34255);
    let server_addr = SocketAddr::new(server_address, 34255);
    let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0);

    let io = client::IoConfig::from_transport(client::ClientTransport::UdpSocket(client_addr))
        .with_conditioner(link_conditioner);
    // let io = client::IoConfig::from_transport(client::ClientTransport::UdpSocket(client_addr));

    let auth = client::Authentication::Manual {
        server_addr,
        client_id,
        private_key: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
        protocol_id: 0,
    };
    let netcode_config = client::NetcodeConfig::default();

    client::NetConfig::Netcode {
        auth,
        io,
        config: netcode_config,
    }
}

pub struct LightyearPlugin;

impl Plugin for LightyearPlugin {
    fn build(&self, app: &mut App) {
        let client_config = client::ClientConfig {
            shared: shared_config(Mode::Separate),
            net: get_client_net_config(IpAddr::from_str("127.0.0.1").unwrap()),
            replication: ReplicationConfig {
                send_interval: REPLICATION_INTERVAL,
                ..default()
            },
            prediction: client::PredictionConfig {
                minimum_input_delay_ticks: 6,
                maximum_input_delay_before_prediction: 6,
                maximum_predicted_ticks: 100,
                ..default()
            },
            ..default()
        };

        let client_plugin = client::ClientPlugins::new(client_config);
        app.add_plugins(client_plugin);
        app.add_plugins(SharedPlugin);
        app.add_systems(Update, display_network_status);
    }
}
