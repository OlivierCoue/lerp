use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: u32,
    pub min: u32,
    pub current: u32,
    pub revision: u32,
    pub revision_checkpoint: u32,
}
impl Health {
    pub fn new(max: u32) -> Self {
        Self {
            max,
            min: 0,
            current: max,
            revision: 1,
            revision_checkpoint: 0,
        }
    }
}
