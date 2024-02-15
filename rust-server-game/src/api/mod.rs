mod resolver;
mod service_area;
mod service_user;
pub mod types;

use crate::game::internal_message::OutboundAreaMessage;

use self::resolver::ApiResolver;
use self::types::*;

use rust_common::proto::{MsgUpHandshake, MsgUpWrapper};
use tokio::sync::mpsc;

pub struct Api {}
impl Api {
    pub async fn run(
        app: App,
        mut rx_udp_user_receiver: mpsc::Receiver<(u16, MsgUpWrapper)>,
        mut rx_udp_user_handshake_receiver: mpsc::Receiver<(u16, MsgUpHandshake)>,
        mut rx_from_instance_internal_messages: mpsc::Receiver<OutboundAreaMessage>,
    ) {
        let app_1 = app.clone();
        let app_2 = app.clone();
        let app_3 = app.clone();

        let join_handle_1 = tokio::task::spawn(async move {
            while let Some((udp_peer_id, udp_msg_up_wrapper)) = rx_udp_user_receiver.recv().await {
                let app = app_1.clone();
                tokio::task::spawn(async move {
                    ApiResolver::handle_msg_up_wrapper(app, udp_peer_id, udp_msg_up_wrapper).await;
                });
            }
        });

        let join_handle_2 = tokio::task::spawn(async move {
            while let Some(outbound_area_message) = rx_from_instance_internal_messages.recv().await
            {
                let app = app_2.clone();
                tokio::task::spawn(async move {
                    ApiResolver::handle_outbound_area_message(app, outbound_area_message).await;
                });
            }
        });

        let join_handle_3 = tokio::task::spawn(async move {
            while let Some((udp_peer_id, udp_msg_up)) = rx_udp_user_handshake_receiver.recv().await
            {
                let app = app_3.clone();
                tokio::task::spawn(async move {
                    ApiResolver::handle_handshake_msg(app, udp_peer_id, udp_msg_up).await;
                });
            }
        });

        join_handle_1.await.unwrap();
        join_handle_2.await.unwrap();
        join_handle_3.await.unwrap();
    }
}
