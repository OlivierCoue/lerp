use enet_sys::*;
use godot::log::godot_print;
use rust_common::proto::{MsgUpHandshake, MsgUpWrapper, UdpMsgDownWrapper};
use std::{
    ffi::CString,
    mem::MaybeUninit,
    ptr::null,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::sync::oneshot;

use prost::Message;
pub struct ENetPeerPtrWrapper(*mut _ENetPeer);

unsafe impl Sync for ENetPeerPtrWrapper {}
unsafe impl Send for ENetPeerPtrWrapper {}

const ADDRESS: &str = env!("SERVER_GAME_IP");

#[allow(clippy::too_many_arguments)]
pub fn udp_client_start(
    tx_receiver_ready: oneshot::Sender<()>,
    tx_sender_ready: oneshot::Sender<()>,
    tx_sender_handshake_ready: oneshot::Sender<()>,

    mut rx_sender_stop: oneshot::Receiver<()>,
    mut rx_sender_handshake_stop: oneshot::Receiver<()>,

    rx_udp_sender: crossbeam_channel::Receiver<MsgUpWrapper>,
    rx_udp_handshake_sender: crossbeam_channel::Receiver<MsgUpHandshake>,
    tx_udp_receiver: crossbeam_channel::Sender<UdpMsgDownWrapper>,

    server_port: u16,
) {
    let peers: Arc<Mutex<Option<ENetPeerPtrWrapper>>> = Arc::new(Mutex::new(None));
    let peers_for_manage = Arc::clone(&peers);
    let peers_for_send = Arc::clone(&peers);
    let peers_for_handshake_send = Arc::clone(&peers);

    let enet_receiver = thread::spawn(move || {
        if unsafe { enet_initialize() } != 0 {
            panic!("[ENet] Could not initialize.");
        }

        godot_print!("[ENet] initialized.");

        let address: MaybeUninit<ENetAddress> = MaybeUninit::uninit();
        let mut address = unsafe { address.assume_init() };
        address.port = server_port;

        let address_hostname = CString::new(ADDRESS).unwrap();

        if unsafe { enet_address_set_host(&mut address, address_hostname.as_ptr()) } != 0 {
            panic!("[ENet] Invalid hostname \"{}\".", ADDRESS);
        }

        let host = unsafe { enet_host_create(null(), 1, 2, 0, 0) };

        *peers_for_manage.lock().unwrap() = Some(ENetPeerPtrWrapper(unsafe {
            enet_host_connect(host, &address, 2, 0)
        }));

        if host.is_null() {
            panic!("[ENet] Failed to create host.");
        }

        let mut event: _ENetEvent = unsafe { MaybeUninit::zeroed().assume_init() };

        // Wait for connect success or fail
        loop {
            if unsafe { enet_host_service(host, &mut event, 5) } > 0 {
                #[allow(non_upper_case_globals)]
                match event.type_ {
                    _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                        godot_print!("[ENet] Connection to server succeeded.");
                        tx_receiver_ready.send(()).unwrap();
                        break;
                    }
                    _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                        godot_print!("[ENet] Server denied connection.",);
                        break;
                    }
                    _ => unreachable!(),
                }
            }
        }

        // Listen for all incoming events until _ENetEventType_ENET_EVENT_TYPE_DISCONNECT event is received
        loop {
            if unsafe { enet_host_service(host, &mut event, 5) } > 0 {
                #[allow(non_upper_case_globals)]
                match event.type_ {
                    _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                        godot_print!("[ENet] Connection to server succeeded.");
                    }
                    _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                        godot_print!("[ENet] Server denied connection.",);
                        break;
                    }
                    _ENetEventType_ENET_EVENT_TYPE_RECEIVE => {
                        let recv_packet_raw: &[u8] = unsafe {
                            std::slice::from_raw_parts(
                                (*event.packet).data,
                                (*event.packet).dataLength,
                            )
                        };
                        // println!("{}", unsafe { (*event.packet).dataLength });

                        let udp_msg_down_wrapper = UdpMsgDownWrapper::decode(recv_packet_raw)
                            .expect("Failed to parse UdpMsgDownWrapper");
                        tx_udp_receiver.send(udp_msg_down_wrapper).unwrap();
                    }
                    _ENetEventType_ENET_EVENT_TYPE_NONE => {
                        godot_print!("[ENet] _ENetEventType_ENET_EVENT_TYPE_NONE.",);
                    }
                    _ => unreachable!(),
                }
            }
        }

        unsafe {
            enet_host_destroy(host);
            enet_deinitialize();
        }
    });

    let enet_sender = thread::spawn(move || {
        tx_sender_ready.send(()).unwrap();

        loop {
            if rx_sender_stop.try_recv().is_ok() {
                break;
            }

            let Ok(msg_to_send) = rx_udp_sender.recv_timeout(Duration::from_millis(500)) else {
                continue;
            };

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
        if let Some(peer) = &*peers_for_send.lock().unwrap() {
            if !peer.0.is_null() {
                unsafe { enet_peer_disconnect(peer.0, 0) };
            }
        }
    });

    let enet_handshake_sender = thread::spawn(move || {
        tx_sender_handshake_ready.send(()).unwrap();

        loop {
            if rx_sender_handshake_stop.try_recv().is_ok() {
                break;
            }

            let Ok(msg_to_send) = rx_udp_handshake_sender.recv_timeout(Duration::from_millis(500))
            else {
                continue;
            };

            if let Some(peer) = &*peers_for_handshake_send.lock().unwrap() {
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
                        enet_peer_send(peer.0, 1, packet);
                    }
                }
            }
        }
        if let Some(peer) = &*peers_for_handshake_send.lock().unwrap() {
            if !peer.0.is_null() {
                unsafe { enet_peer_disconnect(peer.0, 0) };
            }
        }
    });

    enet_handshake_sender.join().unwrap();
    enet_sender.join().unwrap();
    enet_receiver.join().unwrap();
}
