use bevy_ecs::prelude::*;

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Team {
    Player,
    Enemy,
}
