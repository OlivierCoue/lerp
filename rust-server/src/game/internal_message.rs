use bson::oid::ObjectId;

pub enum InboundAreaMessage {
    PlayerInit(PlayerInitPayload),
}

pub struct PlayerInitPayload {
    pub user_id: ObjectId,
    pub udp_peer_id: u16,
}
