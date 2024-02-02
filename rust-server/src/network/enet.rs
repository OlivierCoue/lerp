use enet_cs_sys::*;
use rust_common::proto::{
    udp_down::UdpMsgDownWrapper,
    udp_up::{MsgUp, MsgUpType, MsgUpWrapper},
};
use tokio::sync::mpsc;

use std::{
    collections::HashMap,
    ffi::CString,
    mem::MaybeUninit,
    sync::{Arc, Mutex},
    thread,
};

use protobuf::Message;
pub struct ENetPeerPtrWrapper(*mut _ENetPeer);

unsafe impl Sync for ENetPeerPtrWrapper {}
unsafe impl Send for ENetPeerPtrWrapper {}

pub struct ENetHostPtrWrapper(*mut enet_cs_sys::_ENetHost);
unsafe impl Sync for ENetHostPtrWrapper {}
unsafe impl Send for ENetHostPtrWrapper {}

pub struct ENetEventWrapper(_ENetEvent);
unsafe impl Sync for ENetEventWrapper {}
unsafe impl Send for ENetEventWrapper {}

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 34254;
const MAX_PEERS_COUNT: usize = 10;

pub fn enet_start(
    tx_enet_sender: mpsc::Sender<(u16, MsgUpWrapper)>,
    rx_enet_sender: mpsc::Receiver<(u16, UdpMsgDownWrapper)>,
) {
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let peers_for_manage: Arc<Mutex<HashMap<u16, ENetPeerPtrWrapper>>> = Arc::clone(&peers);
    let peers_for_send: Arc<Mutex<HashMap<u16, ENetPeerPtrWrapper>>> = Arc::clone(&peers);

    let mut handlers = Vec::new();
    handlers.push(thread::spawn(move || {
        enet_receive(tx_enet_sender, peers_for_manage)
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
    peers_for_manage: Arc<Mutex<HashMap<u16, ENetPeerPtrWrapper>>>,
) {
    if unsafe { enet_initialize() } != 0 {
        panic!("[ENet] Could not initialize.");
    }

    println!("[ENet] initialized.");

    let address: MaybeUninit<ENetAddress> = MaybeUninit::uninit();
    let mut address = unsafe { address.assume_init() };
    address.port = PORT;

    let address_hostname = CString::new(ADDRESS).unwrap();

    if unsafe { enet_address_set_hostname(&mut address, address_hostname.as_ptr()) } != 0 {
        panic!("[ENet] Invalid hostname \"{}\".", ADDRESS);
    }

    let host =
        ENetHostPtrWrapper(unsafe { enet_host_create(&address, MAX_PEERS_COUNT, 2, 0, 0, 0) });

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
                }
                _ENetEventType_ENET_EVENT_TYPE_DISCONNECT_TIMEOUT => {
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
                                    _type: MsgUpType::USER_DISCONNECT.into(),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        ))
                        .unwrap();
                }
                _ENetEventType_ENET_EVENT_TYPE_RECEIVE => {
                    let recv_packet_raw: &[u8] = unsafe {
                        std::slice::from_raw_parts(
                            (*event.0.packet).data,
                            (*event.0.packet)
                                .dataLength
                                .try_into()
                                .expect("packet data too long for an `usize`"),
                        )
                    };
                    // let channel_id = event.channelID;
                    // println!("received msg on channel: {}", channel_id);

                    match MsgUpWrapper::parse_from_bytes(recv_packet_raw) {
                        Ok(udp_msg_up) => {
                            tx_enet_sender
                                .blocking_send((
                                    unsafe { *event.0.peer }.incomingPeerID,
                                    udp_msg_up,
                                ))
                                .unwrap();
                        }
                        Err(err) => {
                            println!("Failed to parse recv_packet_raw err: {:#?}", err);
                        }
                    };
                }
                _ENetEventType_ENET_EVENT_TYPE_NONE => {}
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
            let out_bytes = msg_to_send.write_to_bytes().unwrap();
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
