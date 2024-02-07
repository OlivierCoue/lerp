mod api;
mod env;
mod game;
mod network;
mod postgres;
mod utils;
use api::Api;

use tokio::sync::mpsc;
use tokio::task::JoinSet;

use crate::{api::types::App, env::init_env, network::enet::enet_start, postgres::pg_pool_init};

use std::thread;

#[tokio::main]
async fn main() -> Result<(), ()> {
    init_env();

    let (tx_enet_sender, rx_enet_sender) = mpsc::channel(1000);
    let (tx_enet_receiver, rx_enet_receiver) = mpsc::channel(100);

    let app = App::new(pg_pool_init().await, tx_enet_sender.clone());

    let mut join_handlers = Vec::new();
    join_handlers.push(thread::spawn(move || {
        enet_start(tx_enet_receiver, rx_enet_sender)
    }));

    let mut set = JoinSet::new();

    set.spawn(async move {
        let mut users_manager = Api::new(app, rx_enet_receiver);
        users_manager.run().await;
    });

    set.join_next().await;

    for join_handler in join_handlers {
        join_handler.join().unwrap();
    }
    Ok(())
}
