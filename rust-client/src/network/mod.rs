mod network_manager;
mod udp_client;

pub mod prelude {
    pub use crate::network::{network_manager::*, udp_client::*};
}
