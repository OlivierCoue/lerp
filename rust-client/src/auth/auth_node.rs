use std::rc::Rc;

use godot::{
    engine::{Button, Label, LineEdit},
    prelude::*,
};
use rust_common::{
    api_auth::ApiAuthRequest,
    proto::{HttpLoginInput, MsgUp, MsgUpType, MsgUpWrapper, UdpMsgDownType},
};

use crate::{
    network::prelude::*,
    root::{Root, Scenes, PATH_AUTH, PATH_NETWORK, PATH_ROOT},
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct AuthNode {
    base: Base<Node2D>,
    root: OnReady<Gd<Root>>,
    network: OnReady<Gd<NetworkManager>>,
    label_auth_error: OnReady<Gd<Label>>,
    line_edit_username: OnReady<Gd<LineEdit>>,
}

#[godot_api]
impl INode2D for AuthNode {
    fn init(base: Base<Node2D>) -> Self {
        println!("AuthNode: init");
        Self {
            base,
            root: OnReady::manual(),
            network: OnReady::manual(),
            label_auth_error: OnReady::manual(),
            line_edit_username: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        println!("AuthNode: ready");
        self.root.init(self.base().get_node_as::<Root>(PATH_ROOT));
        self.network
            .init(self.base().get_node_as::<NetworkManager>(PATH_NETWORK));

        let auth_ui_scene = load::<PackedScene>("res://auth_ui.tscn");
        let auth_ui = auth_ui_scene.instantiate_as::<Node>();
        self.base_mut().add_child(auth_ui.upcast());

        self.label_auth_error.init(self.base().get_node_as::<Label>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/label_auth_error",
        ));
        self.line_edit_username
            .init(self.base().get_node_as::<LineEdit>(
                String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/line_edit_username",
            ));

        let mut connect_button = self.base().get_node_as::<Button>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/connect_button",
        );
        connect_button.connect(
            "pressed".into(),
            self.base().callable("on_connect_button_pressed"),
        );

        let on_http_success = self.base().callable("on_http_response");
        self.network
            .connect("http_response".into(), on_http_success);
    }

    fn process(&mut self, _: f64) {
        let rx_enet_receiver = Rc::clone(&self.network.bind().rx_udp_receiver);
        while let Ok(udp_msg_down_wrapper) = rx_enet_receiver.try_recv() {
            for udp_msg_down in udp_msg_down_wrapper.messages {
                match UdpMsgDownType::try_from(udp_msg_down.r#type) {
                    Ok(UdpMsgDownType::UserConnectSuccess) => {
                        self.root.bind_mut().change_scene(Scenes::Lobby);
                    }
                    Ok(UdpMsgDownType::UserConnectFailed) => {
                        if let Some(user_connect_failed) = udp_msg_down.user_connect_failed {
                            self.label_auth_error
                                .set_text(user_connect_failed.error_message.into());
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
        println!("on_connect_button_pressed");
        let input_username = self.line_edit_username.get_text();
        if input_username.is_empty() {
            return;
        }

        self.network
            .bind()
            .send_http(ApiAuthRequest::Login(HttpLoginInput {
                username: input_username.to_string(),
                password: "abc".into(),
            }));

        self.network.bind().send_udp(MsgUpWrapper {
            messages: vec![MsgUp {
                r#type: MsgUpType::UserConnect.into(),
                user_connect_username: input_username.to_string(),
                ..Default::default()
            }],
        });
    }

    #[func]
    fn on_http_response(&mut self, v: Variant) {
        let _ = v.to::<Gd<GdApiAuthResponse>>().bind().response;
        godot_print!("Received: on_http_success");
    }
}
