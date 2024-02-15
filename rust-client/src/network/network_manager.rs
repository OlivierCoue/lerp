use godot::prelude::*;
use rust_common::proto::{MsgUpHandshake, MsgUpWrapper, UdpMsgDownWrapper};
use std::{rc::Rc, thread};

use crate::network::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkManager {
    base: Base<Node>,
    pub tx_udp_sender: crossbeam_channel::Sender<MsgUpWrapper>,
    pub tx_udp_handshake_sender: crossbeam_channel::Sender<MsgUpHandshake>,
    pub rx_udp_receiver: crossbeam_channel::Receiver<UdpMsgDownWrapper>,
}

#[godot_api]
impl INode for NetworkManager {
    fn init(base: Base<Node>) -> Self {
        let (tx_udp_sender, rx_udp_sender) = crossbeam_channel::unbounded();
        let (tx_udp_handshake_sender, rx_udp_handshake_sender) = crossbeam_channel::unbounded();
        let (tx_udp_receiver, rx_udp_receiver) = crossbeam_channel::unbounded();

        thread::spawn(move || {
            udp_client_start(rx_udp_sender, rx_udp_handshake_sender, tx_udp_receiver)
        });

        Self {
            base,
            tx_udp_sender,
            tx_udp_handshake_sender,
            rx_udp_receiver,
        }
    }
}

impl NetworkManager {
    pub fn send_udp(&self, udp_msg_up_wrapper: MsgUpWrapper) {
        self.tx_udp_sender
            .send(udp_msg_up_wrapper)
            .expect("[NetworkManager][send_udp] Failed to send udp msg");
    }

    pub fn send_udp_handshake(&self, udp_msg_up_hanshake: MsgUpHandshake) {
        self.tx_udp_handshake_sender
            .send(udp_msg_up_hanshake)
            .expect("[NetworkManager][send_udp_handshake] Failed to send udp msg");
    }
}
