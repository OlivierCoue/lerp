use godot::prelude::*;
use rust_common::{
    api_auth::{ApiAuthRequest, ApiAuthResponse},
    proto::{MsgUpWrapper, UdpMsgDownWrapper},
};
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
    tx_http_sender: Sender<ApiAuthRequest>,
    pub rx_http_receiver: Rc<Receiver<ApiAuthResponse>>,
}

#[godot_api]
impl INode for NetworkManager {
    fn init(base: Base<Node>) -> Self {
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
        while let Ok(response) = rx_http_receiver.try_recv() {
            let data = Gd::from_init_fn(|base| GdApiAuthResponse::new(base, response));
            self.base_mut()
                .emit_signal("http_response".into(), &[data.to_variant()]);
        }
    }
}

#[godot_api]
impl NetworkManager {
    #[signal]
    fn http_response() {}
}

impl NetworkManager {
    pub fn send_udp(&self, udp_msg_up_wrapper: MsgUpWrapper) {
        self.tx_udp_sender
            .send(udp_msg_up_wrapper)
            .expect("[NetworkManager][send_udp] Failed to send udp msg");
    }

    pub fn send_http(&self, request: ApiAuthRequest) {
        self.tx_http_sender
            .send(request)
            .expect("[NetworkManager][send_http] Failed to send http msg");
    }
}

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct GdApiAuthResponse {
    base: Base<RefCounted>,
    pub response: ApiAuthResponse,
}

impl GdApiAuthResponse {
    pub fn new(base: Base<RefCounted>, response: ApiAuthResponse) -> Self {
        Self { base, response }
    }
}
