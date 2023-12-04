mod game;
mod network;
mod utils;
use rust_common::proto::udp_up::UdpMsgUpWrapper;

use crate::game::Game;
use crate::network::enet::enet_start;

use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let (tx_enet_sender, rx_enet_sender) = mpsc::channel();

    let clients_msg = Arc::new(Mutex::new(VecDeque::<(u16, UdpMsgUpWrapper)>::new()));
    let clients_msg_udp_receiver = Arc::clone(&clients_msg);
    let clients_msg_game = Arc::clone(&clients_msg);

    let enet_process = thread::spawn(move || enet_start(clients_msg_udp_receiver, rx_enet_sender));

    let mut game = Game::new(&tx_enet_sender, &clients_msg_game);

    game.start();

    enet_process.join().unwrap();
}
