use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use bevy::prelude::*;
use lightyear::prelude::{client::*, *};
use rust_common_game::settings::*;
use rust_common_game::shared::*;

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

pub struct LightyearPlugin;

impl Plugin for LightyearPlugin {
    fn build(&self, app: &mut App) {
        let client_id = 0;

        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(100),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.00,
        };

        let server_addr = SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), 34255);
        let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0);

        let io = client::IoConfig::from_transport(client::ClientTransport::UdpSocket(client_addr))
            .with_conditioner(link_conditioner);

        let auth = client::Authentication::Manual {
            server_addr,
            client_id,
            private_key: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            protocol_id: 0,
        };
        let netcode_config = client::NetcodeConfig::default();

        let net_config = client::NetConfig::Netcode {
            auth,
            io,
            config: netcode_config,
        };

        let client_config = client::ClientConfig {
            shared: shared_config(Mode::Separate),
            net: net_config,
            replication: ReplicationConfig {
                send_interval: REPLICATION_INTERVAL,
                ..default()
            },
            prediction: client::PredictionConfig {
                always_rollback: false,
                ..Default::default()
            },
            ..default()
        };

        let client_plugin = client::ClientPlugins::new(client_config);
        app.add_plugins(client_plugin);
        app.add_plugins(SharedPlugin);
        app.add_systems(Update, display_network_status);
    }
}
