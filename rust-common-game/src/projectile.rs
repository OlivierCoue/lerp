use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Projectile;
