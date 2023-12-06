mod damage_on_hit;
mod game_entity;
mod health;
mod position;
mod shape;
mod velocity;

pub mod prelude {
    pub use crate::game::components::{
        damage_on_hit::*, game_entity::*, health::*, position::*, shape::*, velocity::*,
    };
}
