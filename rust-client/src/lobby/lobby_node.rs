use std::rc::Rc;

use godot::{
    engine::{Button, LineEdit},
    prelude::*,
};
use rust_common::proto::{
    udp_down::UdpMsgDownType,
    udp_up::{MsgUp, MsgUpType, MsgUpWrapper},
};

use crate::{
    network::prelude::*,
    root::{Root, Scenes, PATH_LOBBY, PATH_NETWORK, PATH_ROOT},
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct LobbyNode {
    base: Base<Node2D>,
    root: Option<Gd<Root>>,
    network: Option<Gd<NetworkManager>>,
    line_edit_join_game_id: Option<Gd<LineEdit>>,
}

#[godot_api]
impl INode2D for LobbyNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            root: None,
            network: None,
            line_edit_join_game_id: None,
        }
    }

    fn ready(&mut self) {
        self.root = Some(self.base().get_node_as::<Root>(PATH_ROOT));
        self.network = Some(self.base().get_node_as::<NetworkManager>(PATH_NETWORK));

        let lobby_ui_scene = load::<PackedScene>("res://lobby_ui.tscn");
        let lobby_ui = lobby_ui_scene.instantiate_as::<Node>();
        self.base_mut().add_child(lobby_ui.upcast());

        let mut button_create_game = self.base().get_node_as::<Button>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/button_create_game",
        );
        button_create_game.connect(
            "pressed".into(),
            self.base().callable("on_button_create_game_pressed"),
        );

        let mut button_join_game = self.base().get_node_as::<Button>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/button_join_game",
        );
        button_join_game.connect(
            "pressed".into(),
            self.base().callable("on_button_join_game_pressed"),
        );

        let mut button_logout = self.base().get_node_as::<Button>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/button_logout",
        );
        button_logout.connect(
            "pressed".into(),
            self.base().callable("on_button_logout_pressed"),
        );

        self.line_edit_join_game_id = Some(self.base().get_node_as::<LineEdit>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/line_edit_join_game_id",
        ));
    }

    fn process(&mut self, _: f64) {
        let rx_enet_receiver = Rc::clone(&self.network.as_ref().unwrap().bind().rx_enet_receiver);
        while let Ok(udp_msg_down_wrapper) = rx_enet_receiver.try_recv() {
            for udp_msg_down in udp_msg_down_wrapper.messages {
                #[allow(clippy::single_match)]
                match udp_msg_down._type.unwrap() {
                    UdpMsgDownType::USER_DISCONNECT_SUCCESS => {
                        self.root
                            .as_mut()
                            .unwrap()
                            .bind_mut()
                            .change_scene(Scenes::Auth);
                    }
                    UdpMsgDownType::USER_CREATE_WORDL_INSTANCE_SUCCESS => {
                        let payload = udp_msg_down
                            .user_create_world_instance_success
                            .into_option()
                            .unwrap();
                        godot_print!("USER_CREATE_WORDL_INSTANCE_SUCCESS: (id: {})", payload.id);
                        self.root
                            .as_mut()
                            .unwrap()
                            .bind_mut()
                            .change_scene(Scenes::Play(payload.id));
                    }
                    _ => {}
                }
            }
        }
    }
}
#[godot_api]
impl LobbyNode {
    #[func]
    fn on_button_create_game_pressed(&mut self) {
        self.network.as_ref().unwrap().bind().send(MsgUpWrapper {
            messages: vec![MsgUp {
                _type: MsgUpType::USER_CREATE_WORLD_INSTANCE.into(),
                ..Default::default()
            }],
            ..Default::default()
        })
    }
    #[func]
    fn on_button_join_game_pressed(&mut self) {
        let input_wolrd_instance_id = self.line_edit_join_game_id.as_ref().unwrap().get_text();
        if input_wolrd_instance_id.is_empty() {
            return;
        }

        self.root
            .as_mut()
            .unwrap()
            .bind_mut()
            .change_scene(Scenes::Play(input_wolrd_instance_id.into()));
    }
    #[func]
    fn on_button_logout_pressed(&mut self) {
        self.network.as_ref().unwrap().bind().send(MsgUpWrapper {
            messages: vec![MsgUp {
                _type: MsgUpType::USER_DISCONNECT.into(),
                ..Default::default()
            }],
            ..Default::default()
        })
    }
}
