use godot::prelude::*;
use std::{
    collections::VecDeque,
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use crate::network::prelude::*;

use rust_common::proto::{udp_down::UdpMsgDownWrapper, udp_up::MsgUpWrapper};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkManager {
    base: Base<Node>,
    pub udp_msg_down_wrappers: Arc<Mutex<VecDeque<UdpMsgDownWrapper>>>,
    pub tx_enet_sender: Sender<MsgUpWrapper>,
    pub rx_enet_receiver: Rc<Receiver<UdpMsgDownWrapper>>,
}

#[godot_api]
impl INode for NetworkManager {
    fn init(base: Base<Node>) -> Self {
        let udp_msg_down_wrappers = Arc::new(Mutex::new(VecDeque::<UdpMsgDownWrapper>::new()));
        let (tx_enet_sender, rx_enet_sender) = mpsc::channel();
        let (tx_enet_receiver, rx_enet_receiver) = mpsc::channel();
        let rx_enet_receiver = Rc::new(rx_enet_receiver);
        let udp_msg_down_wrappers_clone = Arc::clone(&udp_msg_down_wrappers);
        thread::spawn(move || {
            enet_start(
                Arc::clone(&udp_msg_down_wrappers_clone),
                rx_enet_sender,
                tx_enet_receiver,
            )
        });

        Self {
            base,
            udp_msg_down_wrappers,
            tx_enet_sender,
            rx_enet_receiver,
        }
    }
}

impl NetworkManager {
    pub fn send(&self, udp_msg_up_wrapper: MsgUpWrapper) {
        self.tx_enet_sender
            .send(udp_msg_up_wrapper)
            .expect("Failed to send msg to server");
    }
}
