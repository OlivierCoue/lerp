use uuid::Uuid;

pub enum InboundAreaMessage {
    PlayerInit(PlayerInitPayload),
    PlayerLeave(PlayerLeavePayload),
}

pub struct PlayerInitPayload {
    pub user_uuid: Uuid,
    pub udp_peer_id: u16,
}

pub struct PlayerLeavePayload {
    pub user_uuid: Uuid,
}

pub enum OutboundAreaMessage {
    AreaClosing(AreaClosingPayload),
}

pub struct AreaClosingPayload {
    pub area_uuid: Uuid,
}
