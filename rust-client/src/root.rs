use godot::{engine::InputEvent, prelude::*};
use rust_common::proto::{
    common::Point,
    udp_down::UdpMsgDownType,
    udp_up::{UdpMsgUp, UdpMsgUpType, UdpMsgUpWrapper},
};

use crate::{network::Network, play_node::PlayNode, utils::iso_to_cart};

pub const DEBUG: bool = false;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Root {
    #[base]
    base: Base<Node2D>,

    network: Option<Gd<Network>>,
    play_node: Option<Gd<PlayNode>>,
}

#[godot_api]
impl INode2D for Root {
    fn init(base: Base<Node2D>) -> Self {
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

        self.base.set_y_sort_enabled(true);
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
            let mouse_position = iso_to_cart(&self.base.get_global_mouse_position());
            self.base
                .emit_signal("player_move_start".into(), &[mouse_position.to_variant()]);
            if let Some(network) = &self.network {
                network.bind().send(UdpMsgUpWrapper {
                    messages: vec![UdpMsgUp {
                        _type: UdpMsgUpType::PLAYER_MOVE.into(),
                        player_move: Some(Point {
                            x: mouse_position.x,
                            y: mouse_position.y,
                            ..Default::default()
                        })
                        .into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
            }
        } else if event.is_action_pressed("key_e".into()) {
            godot_print!("Key E pressed");
            let mouse_position = iso_to_cart(&self.base.get_global_mouse_position());
            self.base
                .emit_signal("player_throw_fireball_start".into(), &[]);
            if let Some(network) = &self.network {
                network.bind().send(UdpMsgUpWrapper {
                    messages: vec![UdpMsgUp {
                        _type: UdpMsgUpType::PLAYER_THROW_FROZEN_ORB.into(),
                        player_throw_frozen_orb: Some(Point {
                            x: mouse_position.x,
                            y: mouse_position.y,
                            ..Default::default()
                        })
                        .into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
            }
        } else if event.is_action_pressed("key_r".into()) {
            godot_print!("Key R pressed");
            let mouse_position = iso_to_cart(&self.base.get_global_mouse_position());
            if let Some(network) = &self.network {
                network.bind().send(UdpMsgUpWrapper {
                    messages: vec![UdpMsgUp {
                        _type: UdpMsgUpType::PLAYER_TELEPORT.into(),
                        player_teleport: Some(Point {
                            x: mouse_position.x,
                            y: mouse_position.y,
                            ..Default::default()
                        })
                        .into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
            }
        }
    }
}

#[godot_api]
impl Root {
    #[signal]
    fn player_move_start();
    #[signal]
    fn player_throw_fireball_start();
}
