use std::sync::mpsc::{Receiver, Sender};

pub fn http_client_start(rx_http_sender: Receiver<String>, tx_http_receiver: Sender<String>) {
    for msg in &rx_http_sender {
        tx_http_receiver.send(msg).unwrap();
    }
}
