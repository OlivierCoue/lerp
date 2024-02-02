mod resolver;
mod service_area;
mod service_user;
mod types;

use self::resolver::ApiResolver;
use self::types::*;

use rust_common::proto::{udp_down::UdpMsgDownWrapper, udp_up::MsgUpWrapper};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub struct Api {
    mongo_client: mongodb::Client,
    rx_udp_user_receiver: mpsc::Receiver<(u16, MsgUpWrapper)>,
    tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
    connections_state: Arc<Mutex<ConnectionsState>>,
}
impl Api {
    pub fn new(
        mongo_client: mongodb::Client,
        rx_udp_user_receiver: mpsc::Receiver<(u16, MsgUpWrapper)>,
        tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
    ) -> Self {
        Self {
            mongo_client,
            rx_udp_user_receiver,
            tx_udp_sender,
            connections_state: Arc::new(Mutex::new(ConnectionsState::default())),
        }
    }

    pub async fn run(&mut self) {
        while let Some((udp_peer_id, udp_msg_up_wrapper)) = self.rx_udp_user_receiver.recv().await {
            let mongo_client = self.mongo_client.clone();
            let tx_udp_sender = self.tx_udp_sender.clone();
            let connections_state = self.connections_state.clone();
            tokio::task::spawn(async move {
                ApiResolver::handle_msg_up_wrapper(
                    mongo_client,
                    tx_udp_sender,
                    udp_peer_id,
                    connections_state,
                    udp_msg_up_wrapper,
                )
                .await;
            });
        }
    }
}
