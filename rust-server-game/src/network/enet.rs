use enet_sys::*;
use prost::Message;
use rust_common::proto::*;

use local_ip_address::local_ip;
use std::{
    collections::HashMap,
    ffi::CString,
    mem::MaybeUninit,
    sync::{Arc, Mutex},
    thread,
};
use tokio::sync::mpsc;

use crate::env::ENV_UDP_PORT;
pub struct ENetPeerPtrWrapper(*mut _ENetPeer);

unsafe impl Sync for ENetPeerPtrWrapper {}
unsafe impl Send for ENetPeerPtrWrapper {}

pub struct ENetHostPtrWrapper(*mut ENetHost);
unsafe impl Sync for ENetHostPtrWrapper {}
unsafe impl Send for ENetHostPtrWrapper {}

pub struct ENetEventWrapper(_ENetEvent);
unsafe impl Sync for ENetEventWrapper {}
unsafe impl Send for ENetEventWrapper {}

const MAX_PEERS_COUNT: usize = 10;
const MAX_CHANNEL_COUNT: usize = 2;

pub fn enet_start(
    tx_enet_sender: mpsc::Sender<(u16, MsgUpWrapper)>,
    rx_enet_sender: mpsc::Receiver<(u16, UdpMsgDownWrapper)>,
    tx_enet_handshake_sender: mpsc::Sender<(u16, MsgUpHandshake)>,
) {
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let peers_for_manage: Arc<Mutex<HashMap<u16, ENetPeerPtrWrapper>>> = Arc::clone(&peers);
    let peers_for_send: Arc<Mutex<HashMap<u16, ENetPeerPtrWrapper>>> = Arc::clone(&peers);

    let mut handlers = Vec::new();
    handlers.push(thread::spawn(move || {
        enet_receive(tx_enet_sender, tx_enet_handshake_sender, peers_for_manage)
    }));
    handlers.push(thread::spawn(move || {
        enet_send(rx_enet_sender, peers_for_send)
    }));

    for handler in handlers {
        handler.join().unwrap();
    }
}

fn enet_receive(
    tx_enet_sender: mpsc::Sender<(u16, MsgUpWrapper)>,
    tx_enet_handshake_sender: mpsc::Sender<(u16, MsgUpHandshake)>,
    peers_for_manage: Arc<Mutex<HashMap<u16, ENetPeerPtrWrapper>>>,
) {
    if unsafe { enet_initialize() } != 0 {
        panic!("[ENet] Could not initialize.");
    }

    println!("[ENet] initialized.");

    let address_str = local_ip().unwrap().to_string();
    println!("[ENet] Address: {}", address_str);
    let port = std::env::var(ENV_UDP_PORT).unwrap().parse::<u16>().unwrap();

    let address: MaybeUninit<ENetAddress> = MaybeUninit::uninit();
    let mut address = unsafe { address.assume_init() };
    address.port = port;

    let address_hostname = CString::new(address_str.clone().as_str()).unwrap();

    if unsafe { enet_address_set_host(&mut address, address_hostname.as_ptr()) } != 0 {
        panic!("[ENet] Invalid hostname \"{}\".", address_str);
    }

    let host = ENetHostPtrWrapper(unsafe {
        enet_host_create(
            &address,          // address to bind the server host to
            MAX_PEERS_COUNT,   // allow up to 32 clients and/or outgoing connections
            MAX_CHANNEL_COUNT, // allow up to 2 channels to be used, 0 and 1
            0,                 // assume any amount of incoming bandwidth
            0,                 // assume any amount of outgoing bandwidth
        )
    });

    if host.0.is_null() {
        panic!("[ENet] Failed to create host.");
    }

    let mut event: ENetEventWrapper =
        ENetEventWrapper(unsafe { MaybeUninit::zeroed().assume_init() });

    loop {
        if unsafe { enet_host_service(host.0, &mut event.0, 5) } > 0 {
            #[allow(non_upper_case_globals)]
            match event.0.type_ {
                _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                    println!(
                        "[ENet] A peer connected. (id: {})",
                        unsafe { *event.0.peer }.incomingPeerID
                    );
                    peers_for_manage.lock().unwrap().insert(
                        unsafe { *event.0.peer }.incomingPeerID,
                        ENetPeerPtrWrapper(event.0.peer),
                    );
                }
                _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                    println!(
                        "[ENet] A peer disconnected. (id: {})",
                        unsafe { *event.0.peer }.incomingPeerID,
                    );
                    peers_for_manage
                        .lock()
                        .unwrap()
                        .remove(&unsafe { *event.0.peer }.incomingPeerID);

                    tx_enet_sender
                        .blocking_send((
                            unsafe { *event.0.peer }.incomingPeerID,
                            MsgUpWrapper {
                                messages: vec![MsgUp {
                                    r#type: MsgUpType::UserDisconnect.into(),
                                    ..Default::default()
                                }],
                            },
                        ))
                        .unwrap();
                }
                _ENetEventType_ENET_EVENT_TYPE_RECEIVE => {
                    let recv_packet_raw: &[u8] = unsafe {
                        std::slice::from_raw_parts(
                            (*event.0.packet).data,
                            (*event.0.packet).dataLength,
                        )
                    };
                    let channel_id = event.0.channelID;

                    match channel_id {
                        0 => {
                            match MsgUpWrapper::decode(recv_packet_raw) {
                                Ok(udp_msg_up) => {
                                    tx_enet_sender
                                        .blocking_send((
                                            unsafe { *event.0.peer }.incomingPeerID,
                                            udp_msg_up,
                                        ))
                                        .unwrap();
                                }
                                Err(err) => {
                                    println!(
                                        "Channel 0: Failed to parse recv_packet_raw err: {:#?}",
                                        err
                                    );
                                }
                            };
                        }
                        1 => {
                            match MsgUpHandshake::decode(recv_packet_raw) {
                                Ok(udp_msg_up) => {
                                    tx_enet_handshake_sender
                                        .blocking_send((
                                            unsafe { *event.0.peer }.incomingPeerID,
                                            udp_msg_up,
                                        ))
                                        .unwrap();
                                }
                                Err(err) => {
                                    println!(
                                        "Channel 1: Failed to parse recv_packet_raw err: {:#?}",
                                        err
                                    );
                                }
                            };
                        }
                        _ => println!("Unsuported channel"),
                    }
                }
                _ENetEventType_ENET_EVENT_TYPE_NONE => {
                    println!("[ENet] _ENetEventType_ENET_EVENT_TYPE_NONE");
                }
                _ => unreachable!(),
            }
        }
    }
}

fn enet_send(
    mut rx_enet_sender: mpsc::Receiver<(u16, UdpMsgDownWrapper)>,
    peers_for_send: Arc<Mutex<HashMap<u16, ENetPeerPtrWrapper>>>,
) {
    while let Some((peer_id, msg_to_send)) = rx_enet_sender.blocking_recv() {
        if let Some(peer) = peers_for_send.lock().unwrap().get(&peer_id) {
            let mut out_bytes = Vec::with_capacity(msg_to_send.encoded_len());
            msg_to_send.encode(&mut out_bytes).unwrap();
            let packet: *mut _ENetPacket = unsafe {
                enet_packet_create(
                    out_bytes.as_ptr().cast(),
                    out_bytes.len(),
                    _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE,
                )
            };
            if !peer.0.is_null() {
                unsafe {
                    enet_peer_send(peer.0, 0, packet);
                }
            }
        }
    }
}
