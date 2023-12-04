use enet_cs_sys::*;
use rust_common::proto::{udp_down::UdpMsgDownWrapper, udp_up::UdpMsgUpWrapper};

use std::{
    collections::{HashMap, VecDeque},
    ffi::CString,
    mem::MaybeUninit,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use protobuf::Message;
pub struct ENetPeerPtrWrapper(*mut _ENetPeer);

unsafe impl Sync for ENetPeerPtrWrapper {}
unsafe impl Send for ENetPeerPtrWrapper {}

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 34254;
const MAX_PEERS_COUNT: usize = 10;

pub fn enet_start(
    peers_msg: Arc<Mutex<VecDeque<(u16, UdpMsgUpWrapper)>>>,
    rx_enet_sender: Receiver<(u16, UdpMsgDownWrapper)>,
) {
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let peers_for_manage = Arc::clone(&peers);
    let peers_for_send = Arc::clone(&peers);

    let enet_receiver = thread::spawn(move || {
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

        let host = unsafe { enet_host_create(&address, MAX_PEERS_COUNT, 2, 0, 0, 0) };

        if host.is_null() {
            panic!("[ENet] Failed to create host.");
        }

        let mut event: _ENetEvent = unsafe { MaybeUninit::zeroed().assume_init() };

        loop {
            if unsafe { enet_host_service(host, &mut event, 5) } > 0 {
                #[allow(non_upper_case_globals)]
                match event.type_ {
                    _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                        println!(
                            "[ENet] A peer connected. (id: {})",
                            unsafe { *event.peer }.incomingPeerID
                        );
                        peers_for_manage.lock().unwrap().insert(
                            unsafe { *event.peer }.incomingPeerID,
                            ENetPeerPtrWrapper(event.peer),
                        );
                    }
                    _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                        println!(
                            "[ENet] A peer disconnected. (id: {})",
                            unsafe { *event.peer }.incomingPeerID,
                        );
                        peers_for_manage
                            .lock()
                            .unwrap()
                            .remove(&unsafe { *event.peer }.incomingPeerID);
                    }
                    _ENetEventType_ENET_EVENT_TYPE_DISCONNECT_TIMEOUT => {
                        println!(
                            "[ENet] A peer disconnected. (id: {})",
                            unsafe { *event.peer }.incomingPeerID,
                        );
                        peers_for_manage
                            .lock()
                            .unwrap()
                            .remove(&unsafe { *event.peer }.incomingPeerID);
                    }
                    _ENetEventType_ENET_EVENT_TYPE_RECEIVE => {
                        let recv_packet_raw: &[u8] = unsafe {
                            std::slice::from_raw_parts(
                                (*event.packet).data,
                                (*event.packet)
                                    .dataLength
                                    .try_into()
                                    .expect("packet data too long for an `usize`"),
                            )
                        };

                        match UdpMsgUpWrapper::parse_from_bytes(recv_packet_raw) {
                            Ok(udp_msg_up) => {
                                peers_msg
                                    .lock()
                                    .unwrap()
                                    .push_back((unsafe { *event.peer }.incomingPeerID, udp_msg_up));
                            }
                            Err(err) => {
                                println!("Failed to parse recv_packet_raw err: {:#?}", err);
                            }
                        }
                    }
                    _ENetEventType_ENET_EVENT_TYPE_NONE => {}
                    _ => unreachable!(),
                }
            }
        }
    });

    let enet_sender = thread::spawn(move || loop {
        for (peer_id, msg_to_send) in &rx_enet_sender {
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
    });

    enet_sender.join().unwrap();
    enet_receiver.join().unwrap();
}
