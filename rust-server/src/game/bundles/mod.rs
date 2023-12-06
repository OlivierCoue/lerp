mod enemy;
mod player;
mod projectile;

pub mod prelude {
    pub use crate::game::bundles::{enemy::*, player::*, projectile::*};
}
