use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default, PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
    #[default]
    Neutral,
}
