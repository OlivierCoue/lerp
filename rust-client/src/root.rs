use godot::{
    engine::{InputEvent, InputEventMouse},
    prelude::*,
};
use rust_common::proto::data::UdpMsgDownType;

use crate::{network::Network, play_node::PlayNode};

#[derive(GodotClass)]
#[class(base=Node)]
struct Root {
    #[base]
    base: Base<Node>,

    network: Option<Gd<Network>>,
    play_node: Option<Gd<PlayNode>>,
}

#[godot_api]
impl INode for Root {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Root init");

        Self {
            base,
            network: None,
            play_node: None,
        }
    }

    fn ready(&mut self) {
        let mut network = Gd::<Network>::from_init_fn(Network::init);
        network.set_name("Network".into());
        self.network = Some(network.clone());
        self.base.add_child(network.upcast());

        let play_node: Gd<PlayNode> = Gd::<PlayNode>::from_init_fn(PlayNode::init);
        self.play_node = Some(play_node.clone());
        self.base.add_child(play_node.upcast());
    }

    fn process(&mut self, _: f64) {
        if let Some(network) = &mut self.network {
            if let Ok(mut udp_msg_down_wrappers) = network.bind_mut().udp_msg_down_wrappers.lock() {
                while let Some(udp_msg_down_wrapper) = udp_msg_down_wrappers.pop_front() {
                    self.base.emit_signal("udp_msg_down_received".into(), &[]);

                    if let Some(play_node) = &mut self.play_node {
                        for udp_msg_down in udp_msg_down_wrapper.messages {
                            match udp_msg_down._type.unwrap() {
                                UdpMsgDownType::GAME_ENTITY_UPDATE => {
                                    if let Some(entity_update) = udp_msg_down.game_entity_update.0 {
                                        play_node.bind_mut().update_entity(&entity_update);
                                    }
                                }
                                UdpMsgDownType::GAME_ENTITY_REMOVED => {
                                    if let Some(entity_removed) = udp_msg_down.game_entity_removed.0
                                    {
                                        play_node.bind_mut().remove_entity(&entity_removed);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("left_mouse_button".into()) {
            godot_print!("Left button pressed");
            let event_mouse: Gd<InputEventMouse> = event.cast();
            let mouse_position = event_mouse.get_position();
            if let Some(network) = &self.network {
                network.bind().send(
                    "{\"msg_type\":\"PlayerMove\",\"msg_payload\":\"{\\\"x\\\":".to_owned()
                        + &mouse_position.x.to_string()
                        + ",\\\"y\\\":"
                        + &mouse_position.y.to_string()
                        + "}\"}",
                )
            }
        } else if event.is_action_pressed("key_e".into()) {
            godot_print!("Key E pressed");
            let mouse_position = self.base.get_viewport().unwrap().get_mouse_position();
            if let Some(network) = &self.network {
                network.bind().send(
                    "{\"msg_type\":\"PlayerThrowFrozenOrb\",\"msg_payload\":\"{\\\"x\\\":"
                        .to_owned()
                        + &mouse_position.x.to_string()
                        + ",\\\"y\\\":"
                        + &mouse_position.y.to_string()
                        + "}\"}",
                )
            }
        }
    }
}
