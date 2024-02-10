use godot::prelude::*;
use std::{
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::network::prelude::*;

use rust_common::proto::{udp_down::UdpMsgDownWrapper, udp_up::MsgUpWrapper};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkManager {
    base: Base<Node>,
    tx_udp_sender: Sender<MsgUpWrapper>,
    pub rx_udp_receiver: Rc<Receiver<UdpMsgDownWrapper>>,
    tx_http_sender: Sender<String>,
    pub rx_http_receiver: Rc<Receiver<String>>,
}

#[godot_api]
impl INode for NetworkManager {
    fn init(base: Base<Node>) -> Self {
        println!("NetworkManager init");
        let (tx_udp_sender, rx_udp_sender) = mpsc::channel();
        let (tx_udp_receiver, rx_udp_receiver) = mpsc::channel();

        thread::spawn(move || udp_client_start(rx_udp_sender, tx_udp_receiver));

        let (tx_http_sender, rx_http_sender) = mpsc::channel();
        let (tx_http_receiver, rx_http_receiver) = mpsc::channel();

        thread::spawn(move || http_client_start(rx_http_sender, tx_http_receiver));

        Self {
            base,
            tx_udp_sender,
            rx_udp_receiver: Rc::new(rx_udp_receiver),
            tx_http_sender,
            rx_http_receiver: Rc::new(rx_http_receiver),
        }
    }

    fn process(&mut self, _: f64) {
        let rx_http_receiver = Rc::clone(&self.rx_http_receiver);
        while let Ok(msg) = rx_http_receiver.try_recv() {
            println!("emit");
            let data = Gd::from_init_fn(|base| HttpResponse::new(base, msg));
            self.base_mut()
                .emit_signal("http_success".into(), &[data.to_variant()]);
        }
    }
}

#[godot_api]
impl NetworkManager {
    #[signal]
    fn http_success() {}
}

impl NetworkManager {
    // pub fn test(&mut self) {
    //     let obj = Gd::from_init_fn(|base| Data::new(base, 1));

    //     self.base_mut()
    //         .emit_signal("signal".into(), &[obj.to_variant()]);
    // }

    pub fn send_udp(&self, udp_msg_up_wrapper: MsgUpWrapper) {
        self.tx_udp_sender
            .send(udp_msg_up_wrapper)
            .expect("[NetworkManager][send_udp] Failed to send udp msg");
    }

    pub fn send_http(&self, msg: String) {
        self.tx_http_sender
            .send(msg)
            .expect("[NetworkManager][send_http] Failed to send http msg");
    }
}

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct HttpResponse {
    base: Base<RefCounted>,
    pub value: String,
}

impl HttpResponse {
    pub fn new(base: Base<RefCounted>, value: String) -> Self {
        Self { base, value }
    }
}
