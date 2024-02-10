mod http_client;
mod network_manager;
mod udp_client;

pub mod prelude {
    pub use crate::network::{http_client::*, network_manager::*, udp_client::*};
}
