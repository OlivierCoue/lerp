use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}
impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }
}
