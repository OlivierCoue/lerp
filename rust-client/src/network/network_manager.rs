use godot::prelude::*;
use rust_common::proto::{MsgUpWrapper, UdpMsgDownWrapper};
use std::{
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::network::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkManager {
    base: Base<Node>,
    tx_udp_sender: Sender<MsgUpWrapper>,
    pub rx_udp_receiver: Rc<Receiver<UdpMsgDownWrapper>>,
}

#[godot_api]
impl INode for NetworkManager {
    fn init(base: Base<Node>) -> Self {
        let (tx_udp_sender, rx_udp_sender) = mpsc::channel();
        let (tx_udp_receiver, rx_udp_receiver) = mpsc::channel();

        thread::spawn(move || udp_client_start(rx_udp_sender, tx_udp_receiver));

        Self {
            base,
            tx_udp_sender,
            rx_udp_receiver: Rc::new(rx_udp_receiver),
        }
    }
}

impl NetworkManager {
    pub fn send_udp(&self, udp_msg_up_wrapper: MsgUpWrapper) {
        self.tx_udp_sender
            .send(udp_msg_up_wrapper)
            .expect("[NetworkManager][send_udp] Failed to send udp msg");
    }
}
