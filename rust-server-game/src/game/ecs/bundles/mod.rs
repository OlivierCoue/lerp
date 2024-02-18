pub mod enemy;
pub mod player;
pub mod projectile;
pub mod wall;

pub mod prelude {
    pub use crate::game::ecs::bundles::{enemy::*, player::*, projectile::*, wall::*};
}
