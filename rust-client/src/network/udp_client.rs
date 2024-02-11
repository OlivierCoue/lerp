use enet_cs_sys::*;
use godot::log::godot_print;
use rust_common::proto::{MsgUpWrapper, UdpMsgDownWrapper};
use std::{
    ffi::CString,
    mem::MaybeUninit,
    ptr::null,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use prost::Message;
pub struct ENetPeerPtrWrapper(*mut _ENetPeer);

unsafe impl Sync for ENetPeerPtrWrapper {}
unsafe impl Send for ENetPeerPtrWrapper {}

const ADDRESS: &str = "127.0.0.1";
// const ADDRESS: &str = "35.181.43.91";

const PORT: u16 = 34254;

pub fn udp_client_start(
    rx_udp_sender: Receiver<MsgUpWrapper>,
    tx_udp_receiver: Sender<UdpMsgDownWrapper>,
) {
    let peers: Arc<Mutex<Option<ENetPeerPtrWrapper>>> = Arc::new(Mutex::new(None));
    let peers_for_manage = Arc::clone(&peers);
    let peers_for_send = Arc::clone(&peers);

    let enet_receiver = thread::spawn(move || {
        if unsafe { enet_initialize() } != 0 {
            panic!("[ENet] Could not initialize.");
        }

        godot_print!("[ENet] initialized.");

        let address: MaybeUninit<ENetAddress> = MaybeUninit::uninit();
        let mut address = unsafe { address.assume_init() };
        address.port = PORT;

        let address_hostname = CString::new(ADDRESS).unwrap();

        if unsafe { enet_address_set_hostname(&mut address, address_hostname.as_ptr()) } != 0 {
            panic!("[ENet] Invalid hostname \"{}\".", ADDRESS);
        }

        let host = unsafe { enet_host_create(null(), 1, 2, 0, 0, 0) };

        *peers_for_manage.lock().unwrap() = Some(ENetPeerPtrWrapper(unsafe {
            enet_host_connect(host, &address, 2, 0)
        }));

        if host.is_null() {
            panic!("[ENet] Failed to create host.");
        }

        let mut event: _ENetEvent = unsafe { MaybeUninit::zeroed().assume_init() };

        loop {
            if unsafe { enet_host_service(host, &mut event, 5) } > 0 {
                #[allow(non_upper_case_globals)]
                match event.type_ {
                    _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                        godot_print!("[ENet] Connection to server succeeded.");
                    }
                    _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                        godot_print!("[ENet] Server denied connection.",);
                    }
                    _ENetEventType_ENET_EVENT_TYPE_DISCONNECT_TIMEOUT => {
                        godot_print!("[ENet] Server connection timed out.",);
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
                        // println!("{}", unsafe { (*event.packet).dataLength });

                        let udp_msg_down_wrapper = UdpMsgDownWrapper::decode(recv_packet_raw)
                            .expect("Failed to parse UdpMsgDownWrapper");
                        tx_udp_receiver.send(udp_msg_down_wrapper).unwrap();
                    }
                    _ENetEventType_ENET_EVENT_TYPE_NONE => {}
                    _ => unreachable!(),
                }
            }
        }
    });

    let enet_sender = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));

        for msg_to_send in &rx_udp_sender {
            if let Some(peer) = &*peers_for_send.lock().unwrap() {
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
    });

    enet_sender.join().unwrap();
    enet_receiver.join().unwrap();
}
