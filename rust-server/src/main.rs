mod game;
mod maths;
mod network;
mod utils;
use game::serialize::udp_msg_up::UdpMsgUp;

use crate::game::Game;
use crate::network::enet::enet_start;

use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let (tx_enet_sender, rx_enet_sender) = mpsc::channel();

    let clients_msg: Arc<Mutex<VecDeque<(u16, UdpMsgUp)>>> =
        Arc::new(Mutex::new(VecDeque::<(u16, UdpMsgUp)>::new()));
    let clients_msg_udp_receiver = Arc::clone(&clients_msg);
    let clients_msg_game = Arc::clone(&clients_msg);

    let enet_process = thread::spawn(move || enet_start(clients_msg_udp_receiver, rx_enet_sender));

    let mut game = Game::new(&tx_enet_sender, &clients_msg_game);

    game.start();

    enet_process.join().unwrap();
}
