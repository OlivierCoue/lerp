use godot::{obj::WithBaseField, prelude::*};

use crate::{
    auth::auth_node::AuthNode, lobby::lobby_node::LobbyNode, network::prelude::*,
    play::prelude::PlayNode,
};

pub const DEBUG: bool = false;

pub const PATH_ROOT: &str = "/root/Root";

pub const NODE_NETWORK: &str = "NetworkManager";
pub const PATH_NETWORK: &str = "/root/Root/NetworkManager";

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
}

#[godot_api]
impl INode2D for Root {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Root init");

        Self {
            base,
            current_scene: Scenes::Auth,
        }
    }

    fn ready(&mut self) {
        let mut network: Gd<NetworkManager> =
            Gd::<NetworkManager>::from_init_fn(NetworkManager::init);
        network.set_name(NODE_NETWORK.into());
        self.base_mut().add_child(network.upcast());

        let mut auth_node: Gd<AuthNode> = Gd::<AuthNode>::from_init_fn(AuthNode::init);
        auth_node.set_name(NODE_AUTH.into());
        self.base_mut().add_child(auth_node.upcast());
    }
}

impl Root {
    pub fn change_scene(&mut self, scene: Scenes) {
        match &self.current_scene {
            Scenes::Auth => {
                let auth_node = self.base().get_node_as::<AuthNode>(PATH_AUTH);
                self.base_mut().remove_child(auth_node.upcast());
            }
            Scenes::Lobby => {
                let lobby_node = self.base().get_node_as::<LobbyNode>(PATH_LOBBY);
                self.base_mut().remove_child(lobby_node.upcast());
            }
            Scenes::Play(_) => {
                let play_node = self.base().get_node_as::<PlayNode>(PATH_PLAY);
                self.base_mut().remove_child(play_node.upcast());
            }
        }

        match &scene {
            Scenes::Auth => {
                let mut auth_node: Gd<AuthNode> = Gd::<AuthNode>::from_init_fn(AuthNode::init);
                auth_node.set_name(NODE_AUTH.into());
                self.base_mut().add_child(auth_node.upcast());
            }
            Scenes::Lobby => {
                let mut lobby_node: Gd<LobbyNode> = Gd::<LobbyNode>::from_init_fn(LobbyNode::init);
                lobby_node.set_name(NODE_LOBBY.into());
                self.base_mut().add_child(lobby_node.upcast());
            }
            Scenes::Play(world_instance_id) => {
                let mut play_node: Gd<PlayNode> = Gd::<PlayNode>::from_init_fn(PlayNode::init);
                play_node.set_name(NODE_PLAY.into());
                play_node
                    .bind_mut()
                    .init_state(String::from(world_instance_id));
                self.base_mut().add_child(play_node.upcast());
            }
        }
        self.current_scene = scene;
    }
}
