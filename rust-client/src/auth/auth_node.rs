use std::rc::Rc;

use godot::{
    engine::{Button, HttpClient, HttpRequest, Label, LineEdit},
    prelude::*,
};
use rust_common::proto::{
    udp_down::UdpMsgDownType,
    udp_up::{MsgUp, MsgUpType, MsgUpWrapper},
};

use crate::{
    network::prelude::*,
    root::{Root, Scenes, PATH_AUTH, PATH_NETWORK, PATH_ROOT},
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct AuthNode {
    base: Base<Node2D>,
    root: Option<Gd<Root>>,
    network: Option<Gd<NetworkManager>>,
    label_auth_error: Option<Gd<Label>>,
    line_edit_username: Option<Gd<LineEdit>>,
}

#[godot_api]
impl INode2D for AuthNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            root: None,
            network: None,
            label_auth_error: None,
            line_edit_username: None,
        }
    }

    fn ready(&mut self) {
        self.root = Some(self.base().get_node_as::<Root>(PATH_ROOT));
        self.network = Some(self.base().get_node_as::<NetworkManager>(PATH_NETWORK));

        let auth_ui_scene = load::<PackedScene>("res://auth_ui.tscn");
        let auth_ui = auth_ui_scene.instantiate_as::<Node>();
        self.base_mut().add_child(auth_ui.upcast());

        let mut connect_button = self.base().get_node_as::<Button>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/connect_button",
        );
        connect_button.connect(
            "pressed".into(),
            self.base().callable("on_connect_button_pressed"),
        );

        self.label_auth_error = Some(self.base().get_node_as::<Label>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/label_auth_error",
        ));
        self.line_edit_username = Some(self.base().get_node_as::<LineEdit>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/line_edit_username",
        ));
    }

    fn process(&mut self, _: f64) {
        let rx_enet_receiver = Rc::clone(&self.network.as_ref().unwrap().bind().rx_enet_receiver);
        while let Ok(udp_msg_down_wrapper) = rx_enet_receiver.try_recv() {
            for udp_msg_down in udp_msg_down_wrapper.messages {
                match udp_msg_down._type.unwrap() {
                    UdpMsgDownType::USER_CONNECT_SUCCESS => {
                        if let Some(root) = self.root.as_mut() {
                            root.bind_mut().change_scene(Scenes::Lobby);
                        }
                    }
                    UdpMsgDownType::USER_CONNECT_FAILED => {
                        if let (Some(label_auth_error), Some(user_connect_failed)) = (
                            self.label_auth_error.as_mut(),
                            udp_msg_down.user_connect_failed.into_option(),
                        ) {
                            label_auth_error.set_text(user_connect_failed.error_message.into());
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
#[godot_api]
impl AuthNode {
    #[func]
    fn on_connect_button_pressed(&mut self) {
        let input_username = self.line_edit_username.as_ref().unwrap().get_text();
        if input_username.is_empty() {
            return;
        }

        self.network.as_ref().unwrap().bind().send(MsgUpWrapper {
            messages: vec![MsgUp {
                _type: MsgUpType::USER_CONNECT.into(),
                user_connect_username: Some(input_username.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        })
    }
}
