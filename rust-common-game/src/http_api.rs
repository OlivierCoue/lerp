use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct HttpStartServerResponse {
    pub instance_port: u16,
    pub instance_uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct HttpStopServerInput {
    pub instance_port: u16,
    pub instance_uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct HttpStopServerResponse {
    pub succcess: bool,
}
