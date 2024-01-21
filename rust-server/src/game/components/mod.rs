mod cast;
mod collider_dmg_in;
mod collider_mvt;
mod damage_on_hit;
mod game_entity;
mod health;
mod position;
mod velocity;

pub mod prelude {
    pub use crate::game::components::{
        cast::*, collider_dmg_in::*, collider_mvt::*, damage_on_hit::*, game_entity::*, health::*,
        position::*, velocity::*,
    };
}
