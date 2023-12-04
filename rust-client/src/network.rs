use std::{
    collections::VecDeque,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use godot::{
    bind::{godot_api, GodotClass},
    engine::{INode, Node},
    log::godot_print,
    obj::Base,
};
use rust_common::proto::{
    udp_down::UdpMsgDownWrapper,
    udp_up::{UdpMsgUp, UdpMsgUpType, UdpMsgUpWrapper},
};

use crate::enet::enet_start;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Network {
    #[base]
    base: Base<Node>,

    pub udp_msg_down_wrappers: Arc<Mutex<VecDeque<UdpMsgDownWrapper>>>,
    pub tx_enet_sender: Arc<Mutex<Option<Sender<UdpMsgUpWrapper>>>>,
}

#[godot_api]
impl INode for Network {
    fn init(base: Base<Node>) -> Self {
        Network {
            base,
            udp_msg_down_wrappers: Arc::new(Mutex::new(VecDeque::<UdpMsgDownWrapper>::new())),
            tx_enet_sender: Arc::new(Mutex::new(None)),
        }
    }

    fn ready(&mut self) {
        godot_print!("Network ready with path {}", self.base.get_path());

        let udp_msg_down_wrappers_clone = Arc::clone(&self.udp_msg_down_wrappers);
        let (tx_enet_sender, rx_enet_sender) = mpsc::channel();
        *self.tx_enet_sender.lock().unwrap() = Some(tx_enet_sender);

        thread::spawn(move || enet_start(udp_msg_down_wrappers_clone, rx_enet_sender));

        let tx_enet_sender_clone = Arc::clone(&self.tx_enet_sender);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(1000));
            if let Some(some_tx_enet_sender_clone) = &*tx_enet_sender_clone.lock().unwrap() {
                some_tx_enet_sender_clone
                    .send(UdpMsgUpWrapper {
                        messages: vec![UdpMsgUp {
                            _type: UdpMsgUpType::PLAYER_PING.into(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    })
                    .expect("Failed to send msg: PLAYER_PING to server");
            }
        });

        if let Some(some_tx_enet_sender) = &*self.tx_enet_sender.lock().unwrap() {
            some_tx_enet_sender
                .send(UdpMsgUpWrapper {
                    messages: vec![UdpMsgUp {
                        _type: UdpMsgUpType::PLAYER_INIT.into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
                .expect("Failed to send msg: PLAYER_INIT to server");
        }
    }
}

#[godot_api]
impl Network {
    #[signal]
    fn udp_msg_down_received();
}

impl Network {
    pub fn send(&self, udp_msg_up_wrapper: UdpMsgUpWrapper) {
        if let Some(some_tx_enet_sender) = &*self.tx_enet_sender.lock().unwrap() {
            some_tx_enet_sender
                .send(udp_msg_up_wrapper)
                .expect("Failed to send msg to server");
        }
    }
}
