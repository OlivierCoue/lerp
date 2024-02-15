mod area_config;
mod enemies_state;
mod pathfinder_state;
mod time;

pub mod prelude {
    pub use crate::game::ecs::resources::{
        area_config::*, enemies_state::*, pathfinder_state::*, time::*,
    };
}
