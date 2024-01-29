mod api;
mod game;
mod network;
mod utils;
use api::Api;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

use tokio::sync::mpsc;
use tokio::task::JoinSet;

use crate::network::enet::enet_start;

use std::thread;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mongo_client_uri = String::from("mongodb://admin:password@localhost:27017/?authSource=admin&replicaSet=rs0&readPreference=nearest");
    let mongo_options =
        ClientOptions::parse_with_resolver_config(&mongo_client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    let mongo_client = Client::with_options(mongo_options).unwrap();
    for name in mongo_client.list_database_names(None, None).await.unwrap() {
        println!("- {}", name);
    }

    let (tx_enet_sender, rx_enet_sender) = mpsc::channel(1000);
    let (tx_enet_receiver, rx_enet_receiver) = mpsc::channel(100);

    let tx_enet_sender_2 = tx_enet_sender.clone();

    let mut join_handlers = Vec::new();
    join_handlers.push(thread::spawn(move || {
        enet_start(tx_enet_receiver, rx_enet_sender)
    }));

    let mut set = JoinSet::new();
    set.spawn(async move {
        let mut users_manager = Api::new(mongo_client.clone(), rx_enet_receiver, tx_enet_sender_2);
        users_manager.run().await;
    });

    set.join_next().await;

    for join_handler in join_handlers {
        join_handler.join().unwrap();
    }
    Ok(())
}
