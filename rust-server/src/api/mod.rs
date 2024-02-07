mod resolver;
mod service_area;
mod service_user;
pub mod types;

use self::resolver::ApiResolver;
use self::types::*;

use rust_common::proto::udp_up::MsgUpWrapper;
use tokio::sync::mpsc;

pub struct Api {
    app: App,
    rx_udp_user_receiver: mpsc::Receiver<(u16, MsgUpWrapper)>,
}
impl Api {
    pub fn new(app: App, rx_udp_user_receiver: mpsc::Receiver<(u16, MsgUpWrapper)>) -> Self {
        Self {
            app,
            rx_udp_user_receiver,
        }
    }

    pub async fn run(&mut self) {
        while let Some((udp_peer_id, udp_msg_up_wrapper)) = self.rx_udp_user_receiver.recv().await {
            let app = self.app.clone();
            tokio::task::spawn(async move {
                ApiResolver::handle_msg_up_wrapper(app, udp_peer_id, udp_msg_up_wrapper).await;
            });
        }
    }
}
