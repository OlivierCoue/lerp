mod enemy;
mod player;
mod projectile;
mod wall;

pub mod prelude {
    pub use crate::game::bundles::{enemy::*, player::*, projectile::*, wall::*};
}
