use rust_common::proto::{MsgUpHandshake, MsgUpWrapper, UdpMsgDownWrapper};
use std::thread::{self};
use tokio::sync::oneshot;

use crate::udp::prelude::*;

#[derive(Clone)]
pub struct UdpState {
    tx_udp_state_in_events: crossbeam_channel::Sender<UdpStateInEvent>,
    pub tx_udp_sender: crossbeam_channel::Sender<MsgUpWrapper>,
    pub tx_udp_handshake_sender: crossbeam_channel::Sender<MsgUpHandshake>,
    pub rx_udp_receiver: crossbeam_channel::Receiver<UdpMsgDownWrapper>,
}

pub enum UdpStateInEvent {
    StartClient(oneshot::Sender<()>, u16),
    StopClient(oneshot::Sender<()>),
}

pub struct UdpStateManager {
    #[allow(dead_code)]
    state: UdpState,

    rx_udp_sender: crossbeam_channel::Receiver<MsgUpWrapper>,
    rx_udp_handshake_sender: crossbeam_channel::Receiver<MsgUpHandshake>,
    tx_udp_receiver: crossbeam_channel::Sender<UdpMsgDownWrapper>,

    tx_sender_stop: Option<oneshot::Sender<()>>,
    tx_sender_handshake_stop: Option<oneshot::Sender<()>>,

    running_thread_handle: Option<thread::JoinHandle<()>>,
}

impl UdpState {
    pub fn start_client(&mut self, server_port: u16) {
        let (tx, rx) = oneshot::channel();
        self.tx_udp_state_in_events
            .send(UdpStateInEvent::StartClient(tx, server_port))
            .unwrap();
        rx.blocking_recv().unwrap();
    }

    pub async fn start_client_async(&mut self, server_port: u16) {
        let (tx, rx) = oneshot::channel();
        self.tx_udp_state_in_events
            .send(UdpStateInEvent::StartClient(tx, server_port))
            .unwrap();
        rx.await.unwrap();
    }

    pub fn stop_client(&mut self) {
        let (tx, rx) = oneshot::channel();
        self.tx_udp_state_in_events
            .send(UdpStateInEvent::StopClient(tx))
            .unwrap();
        rx.blocking_recv().unwrap();
    }

    pub async fn stop_client_async(&mut self) {
        let (tx, rx) = oneshot::channel();
        self.tx_udp_state_in_events
            .send(UdpStateInEvent::StopClient(tx))
            .unwrap();
        rx.await.unwrap();
    }

    pub fn send_udp(&self, udp_msg_up_wrapper: MsgUpWrapper) {
        self.tx_udp_sender
            .send(udp_msg_up_wrapper)
            .expect("[NetworkManager][send_udp] Failed to send udp msg");
    }

    pub fn send_udp_handshake(&self, udp_msg_up_hanshake: MsgUpHandshake) {
        self.tx_udp_handshake_sender
            .send(udp_msg_up_hanshake)
            .expect("[NetworkManager][send_udp_handshake] Failed to send udp msg");
    }
}

impl UdpStateManager {
    #[allow(clippy::new_without_default)]
    pub fn new_with_state(
        tx_udp_state_in_events: crossbeam_channel::Sender<UdpStateInEvent>,
    ) -> (Self, UdpState) {
        let (tx_udp_sender, rx_udp_sender) = crossbeam_channel::unbounded();
        let (tx_udp_handshake_sender, rx_udp_handshake_sender) = crossbeam_channel::unbounded();
        let (tx_udp_receiver, rx_udp_receiver) = crossbeam_channel::unbounded();

        let state = UdpState {
            tx_udp_state_in_events,
            rx_udp_receiver,
            tx_udp_handshake_sender,
            tx_udp_sender,
        };

        (
            Self {
                state: state.clone(),

                rx_udp_sender,
                rx_udp_handshake_sender,
                tx_udp_receiver,

                tx_sender_stop: None,
                tx_sender_handshake_stop: None,

                running_thread_handle: None,
            },
            state,
        )
    }

    pub fn start(&mut self, rx_udp_state_in_events: crossbeam_channel::Receiver<UdpStateInEvent>) {
        for event in &rx_udp_state_in_events {
            match event {
                UdpStateInEvent::StartClient(tx, server_port) => {
                    self.start_udp_client(server_port);
                    tx.send(()).unwrap();
                }
                UdpStateInEvent::StopClient(tx) => {
                    self.stop_udp_client();
                    tx.send(()).unwrap();
                }
            }
        }
    }

    fn start_udp_client(&mut self, server_port: u16) {
        if self.running_thread_handle.is_some()
            || self.tx_sender_stop.is_some()
            || self.tx_sender_handshake_stop.is_some()
        {
            panic!("[NetworkManager][start_udp_client] Udp Client is already running and must be stopped to start it.");
        };

        let rx_udp_sender = self.rx_udp_sender.clone();
        let rx_udp_handshake_sender = self.rx_udp_handshake_sender.clone();
        let tx_udp_receiver = self.tx_udp_receiver.clone();

        let (tx_receiver_ready, rx_receiver_ready) = oneshot::channel();
        let (tx_sender_ready, rx_sender_ready) = oneshot::channel();
        let (tx_sender_handshake_ready, rx_sender_handshake_ready) = oneshot::channel();

        let (tx_sender_stop, rx_sender_stop) = oneshot::channel();
        let (tx_sender_handshake_stop, rx_sender_handshake_stop) = oneshot::channel();

        self.tx_sender_stop = Some(tx_sender_stop);
        self.tx_sender_handshake_stop = Some(tx_sender_handshake_stop);

        self.running_thread_handle = Some(thread::spawn(move || {
            udp_client_start(
                tx_receiver_ready,
                tx_sender_ready,
                tx_sender_handshake_ready,
                //
                rx_sender_stop,
                rx_sender_handshake_stop,
                //
                rx_udp_sender,
                rx_udp_handshake_sender,
                tx_udp_receiver,
                //
                server_port,
            )
        }));

        rx_receiver_ready.blocking_recv().unwrap();
        rx_sender_ready.blocking_recv().unwrap();
        rx_sender_handshake_ready.blocking_recv().unwrap();

        println!("[NetworkManager][start_udp_client] Udp Client successfully started.")
    }

    fn stop_udp_client(&mut self) {
        if self.running_thread_handle.is_none()
            || self.tx_sender_stop.is_none()
            || self.tx_sender_handshake_stop.is_none()
        {
            panic!("[NetworkManager][stop_udp_client] Udp Client is not running and must be started to stop it.");
        };

        self.tx_sender_stop.take().unwrap().send(()).unwrap();
        self.tx_sender_handshake_stop
            .take()
            .unwrap()
            .send(())
            .unwrap();

        self.running_thread_handle.take().unwrap().join().unwrap();

        println!("[NetworkManager][stop_udp_client] Udp Client successfully stopped.")
    }
}
