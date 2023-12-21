mod enemies_state;
mod pathfinder_state;
mod time;

pub mod prelude {
    pub use crate::game::resources::{enemies_state::*, pathfinder_state::*, time::*};
}
