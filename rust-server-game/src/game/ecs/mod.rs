mod bundles;
mod components;
mod events;
mod resources;
mod systems;

pub mod prelude {
    pub use crate::game::ecs::{
        bundles::prelude::*, components::prelude::*, events::prelude::*, resources::prelude::*,
        systems::prelude::*,
    };
}
