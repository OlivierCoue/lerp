use std::time::Duration;

use bevy::prelude::*;

use lightyear::prelude::*;
use lightyear::shared::config::Mode;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const REPLICATION_INTERVAL: Duration = Duration::from_millis(40);

pub const ENTITY_SIZE: f32 = 32.;

pub fn shared_config(mode: Mode) -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
        mode,
    }
}
