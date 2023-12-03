use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub enum UdpMsgUpTypes {
    PlayerInit,
    PlayerMove,
    PlayerTeleport,
    PlayerPing,
    PlayerToggleHidden,
    PlayerThrowProjectile,
    PlayerThrowFrozenOrb,
    GamePause,
}

#[derive(Deserialize, Debug)]
pub struct UdpMsgUp {
    pub msg_type: UdpMsgUpTypes,
    pub msg_payload: String,
}
