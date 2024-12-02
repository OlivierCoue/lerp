mod map;
mod player;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use crate::common::*;
use crate::states::play::map::*;
use crate::states::play::player::*;
use avian2d::prelude::*;
use bevy::prelude::*;

use bevy_transform_interpolation::TransformEasingSet;
use bevy_transform_interpolation::TransformInterpolationPlugin;
use lightyear::client::input::native::InputSystemSet;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use rust_common_game::settings::*;
use rust_common_game::shared::SharedPlugin;

#[derive(Component)]
pub struct PlaySceneTag;

pub fn play_scene_setup(mut commands: Commands) {
    println!("[play_scene_setup]");

    commands.connect_client();
    commands.spawn((PlaySceneTag, Camera2dBundle::default()));
    commands.spawn((
        PlaySceneTag,
        TextBundle::from_section("Play Scene", TextStyle::default()),
    ));
}

pub fn play_scene_logic(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Lobby);
    }
}

pub fn play_scene_cleanup(mut commands: Commands, query: Query<Entity, With<PlaySceneTag>>) {
    println!("[play_scene_cleanup]");
    commands.disconnect_client();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

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

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        let client_id = 0;
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(100),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.00,
        };

        let server_addr = SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), 34255);
        let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0);
        let io_config =
            client::IoConfig::from_transport(client::ClientTransport::UdpSocket(client_addr))
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
            io: io_config,
            config: netcode_config,
        };

        let client_config = client::ClientConfig {
            shared: shared_config(Mode::Separate),
            net: net_config,
            replication: ReplicationConfig {
                send_interval: REPLICATION_INTERVAL,
                ..default()
            },
            prediction: PredictionConfig {
                always_rollback: false,
                ..Default::default()
            },
            ..default()
        };
        let client_plugin = client::ClientPlugins::new(client_config);
        app.add_plugins(client_plugin);
        app.add_plugins(SharedPlugin);

        app.add_plugins(TransformInterpolationPlugin::default());

        app.add_systems(OnEnter(AppState::Play), play_scene_setup);
        app.add_systems(Update, play_scene_logic.run_if(in_state(AppState::Play)));
        app.add_systems(OnExit(AppState::Play), play_scene_cleanup);

        app.add_systems(OnEnter(AppState::Play), setup_map);

        app.add_systems(Update, handle_new_player.run_if(in_state(AppState::Play)));
        app.add_systems(
            FixedPostUpdate,
            sync_position_to_transform
                .before(TransformSystem::TransformPropagate)
                .run_if(in_state(AppState::Play)),
        );
        app.add_systems(
            PostUpdate,
            camera_follow
                .before(TransformSystem::TransformPropagate)
                .after(TransformEasingSet)
                .run_if(in_state(AppState::Play)),
        );

        app.add_systems(FixedUpdate, movement.run_if(in_state(AppState::Play)));
        // app.add_systems(Update, capture_world_click.run_if(in_state(AppState::Play)));
        app.add_systems(Update, set_player_target.run_if(in_state(AppState::Play)));
        app.add_systems(
            Update,
            display_network_status.run_if(in_state(AppState::Play)),
        );
        app.add_systems(
            Update,
            (
                draw_confirmed_player.run_if(in_state(AppState::Play)),
                draw_predicted_target.run_if(in_state(AppState::Play)),
            ),
        );
        app.add_systems(
            FixedPreUpdate,
            buffer_input.in_set(InputSystemSet::BufferInputs),
        );

        // app.add_event::<LeftClickEvent>();
    }
}
