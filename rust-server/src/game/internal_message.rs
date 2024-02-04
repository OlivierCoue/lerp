use bson::oid::ObjectId;

pub enum InboundAreaMessage {
    PlayerInit(PlayerInitPayload),
    PlayerLeave(PlayerLeavePayload),
}

pub struct PlayerInitPayload {
    pub user_id: ObjectId,
    pub udp_peer_id: u16,
}

pub struct PlayerLeavePayload {
    pub user_id: ObjectId,
}
