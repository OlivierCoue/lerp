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
use rust_common::proto::data::UdpMsgDownWrapper;

use crate::enet::enet_start;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Network {
    #[base]
    base: Base<Node>,

    pub udp_msg_down_wrappers: Arc<Mutex<VecDeque<UdpMsgDownWrapper>>>,
    pub tx_enet_sender: Arc<Mutex<Option<Sender<String>>>>,
}

#[godot_api]
impl INode for Network {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Network init");

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
                    .send(String::from(
                        "{\"msg_type\":\"PlayerPing\",\"msg_payload\":\"{}\"}\n",
                    ))
                    .expect("Failed to send msg to server");
            }
        });

        if let Some(some_tx_enet_sender) = &*self.tx_enet_sender.lock().unwrap() {
            some_tx_enet_sender
                .send(String::from(
                    "{\"msg_type\":\"PlayerInit\",\"msg_payload\":\"{}\"}",
                ))
                .expect("Failed to send msg to server");
        }
    }

    // fn process(&mut self, _: f64) {
    //     if let Ok(mut udp_msg_down_wrappers) = self.udp_msg_down_wrappers.lock() {
    //         while let Some(udp_msg_down_wrapper) = udp_msg_down_wrappers.pop_front() {
    //             godot_print!("Received msg: {:?}", udp_msg_down_wrapper.server_time);
    //             self.base.emit_signal("udp_msg_down_received".into(), &[]);
    //         }
    //     }
    // }
}

#[godot_api]
impl Network {
    #[signal]
    fn udp_msg_down_received();
}

impl Network {
    pub fn send(&self, msg: String) {
        if let Some(some_tx_enet_sender) = &*self.tx_enet_sender.lock().unwrap() {
            some_tx_enet_sender
                .send(msg)
                .expect("Failed to send msg to server");
        }
    }
}
