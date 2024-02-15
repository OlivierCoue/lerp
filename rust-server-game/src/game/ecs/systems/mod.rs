mod damage_on_hit;
mod enemies;
mod frozen_rob;
mod movement;
mod spell;

pub mod prelude {
    pub use crate::game::ecs::systems::{
        damage_on_hit::*, enemies::*, frozen_rob::*, movement::*, spell::*,
    };
}
