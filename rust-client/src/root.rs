use std::thread::{self};

use godot::{obj::WithBaseField, prelude::*};

use crate::{
    auth::auth_node::AuthNode,
    global_state::{GlobalState, GlobalStateManager},
    lobby::lobby_node::LobbyNode,
    play::prelude::PlayNode,
    udp::prelude::*,
};

pub const DEBUG: bool = false;

pub const PATH_ROOT: &str = "/root/Root";

pub const NODE_PLAY: &str = "Play";
pub const PATH_PLAY: &str = "/root/Root/Play";

pub const NODE_AUTH: &str = "Auth";
pub const PATH_AUTH: &str = "/root/Root/Auth";

pub const NODE_LOBBY: &str = "Lobby";
pub const PATH_LOBBY: &str = "/root/Root/Lobby";

#[derive(Clone)]
pub enum Scenes {
    Auth,
    Lobby,
    // Play(world_instance_id)
    Play(String),
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Root {
    base: Base<Node2D>,
    current_scene: Scenes,
    global_state: GlobalState,
    udp_state: UdpState,
}

#[godot_api]
impl INode2D for Root {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Root init");

        let (tx_local_to_global_state_events, rx_local_to_global_state_events) =
            crossbeam_channel::unbounded();

        let global_state = GlobalState::new(tx_local_to_global_state_events);
        let mut global_state_manager = GlobalStateManager::new(global_state.clone());

        thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .max_blocking_threads(4)
                .thread_name("global-pool")
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    global_state_manager
                        .start(rx_local_to_global_state_events)
                        .await;
                });
        });

        let (tx_udp_state_in_events, rx_udp_state_in_events) = crossbeam_channel::unbounded();

        let (mut udp_state_manager, udp_state) =
            UdpStateManager::new_with_state(tx_udp_state_in_events);

        thread::spawn(move || {
            udp_state_manager.start(rx_udp_state_in_events);
        });

        Self {
            base,
            current_scene: Scenes::Auth,
            global_state,
            udp_state,
        }
    }

    fn ready(&mut self) {
        let mut auth_node: Gd<AuthNode> =
            Gd::<AuthNode>::from_init_fn(|base| AuthNode::init(base, self.global_state.clone()));
        auth_node.set_name(NODE_AUTH.into());
        self.base_mut().add_child(auth_node.upcast());
    }
}

impl Root {
    pub fn change_scene(&mut self, scene: Scenes) {
        match &self.current_scene {
            Scenes::Auth => {
                let mut auth_node = self.base().get_node_as::<AuthNode>(PATH_AUTH);
                auth_node.queue_free();
            }
            Scenes::Lobby => {
                let mut lobby_node = self.base().get_node_as::<LobbyNode>(PATH_LOBBY);
                lobby_node.queue_free();
            }
            Scenes::Play(_) => {
                let mut play_node = self.base().get_node_as::<PlayNode>(PATH_PLAY);
                play_node.queue_free();
            }
        }

        match &scene {
            Scenes::Auth => {
                let mut auth_node: Gd<AuthNode> = Gd::<AuthNode>::from_init_fn(|base| {
                    AuthNode::init(base, self.global_state.clone())
                });
                auth_node.set_name(NODE_AUTH.into());
                self.base_mut().add_child(auth_node.upcast());
            }
            Scenes::Lobby => {
                let mut lobby_node: Gd<LobbyNode> = Gd::<LobbyNode>::from_init_fn(|base| {
                    LobbyNode::init(base, self.global_state.clone(), self.udp_state.clone())
                });
                lobby_node.set_name(NODE_LOBBY.into());
                self.base_mut().add_child(lobby_node.upcast());
            }
            Scenes::Play(world_instance_id) => {
                let mut play_node: Gd<PlayNode> = Gd::<PlayNode>::from_init_fn(|base| {
                    PlayNode::init(base, world_instance_id.clone(), self.udp_state.clone())
                });
                play_node.set_name(NODE_PLAY.into());
                self.base_mut().add_child(play_node.upcast());
            }
        }
        self.current_scene = scene;
    }
}
