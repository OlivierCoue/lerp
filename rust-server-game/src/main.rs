mod api;
mod env;
mod game;
mod network;
mod postgres;
mod utils;
use api::Api;

use game::area_gen::generate_area;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

use crate::{api::types::App, env::init_env, network::enet::enet_start, postgres::pg_pool_init};

use std::thread;
const DEBUG_GC: bool = false;

#[tokio::main]
async fn main() -> Result<(), ()> {
    init_env();

    if DEBUG_GC {
        let mut handlers = Vec::new();
        for i in 0..5 {
            handlers.push(thread::spawn(move || {
                generate_area(i);
            }));
        }
        for handler in handlers {
            handler.join().unwrap();
        }
    }

    let (tx_enet_sender, rx_enet_sender) = mpsc::channel(1000);
    let (tx_enet_receiver, rx_enet_receiver) = mpsc::channel(100);
    let (tx_enet_handshake_receiver, rx_enet_handshake_receiver) = mpsc::channel(100);
    let (tx_from_instance_internal_messages, rx_from_instance_internal_messages) =
        mpsc::channel(100);

    let app = App::new(
        pg_pool_init().await,
        tx_enet_sender,
        tx_from_instance_internal_messages,
    );

    let mut join_handlers = Vec::new();
    join_handlers.push(thread::spawn(move || {
        enet_start(tx_enet_receiver, rx_enet_sender, tx_enet_handshake_receiver)
    }));

    let mut set = JoinSet::new();

    set.spawn(async move {
        Api::run(
            app,
            rx_enet_receiver,
            rx_enet_handshake_receiver,
            rx_from_instance_internal_messages,
        )
        .await;
    });

    set.join_next().await;

    for join_handler in join_handlers {
        join_handler.join().unwrap();
    }
    Ok(())
}
