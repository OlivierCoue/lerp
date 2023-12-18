use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::helper::get_timestamp_millis;

use crate::game::systems::prelude::world_bounded_vector2;

#[derive(Component)]
pub struct Velocity {
    revision: u32,
    target: Option<Vector2>,
    speed: f32,
    timestamp_at_target: u64,
    distance_to_target: f32,
    despawn_at_target: bool,
}
impl Velocity {
    pub fn new(target: Option<Vector2>, speed: f32, despawn_at_target: bool) -> Self {
        Self {
            revision: 0,
            target,
            speed,
            timestamp_at_target: get_timestamp_millis() as u64,
            distance_to_target: 0.0,
            despawn_at_target,
        }
    }

    pub fn get_target(&self) -> &Option<Vector2> {
        &self.target
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_timestamp_at_target(&self) -> u64 {
        self.timestamp_at_target
    }

    pub fn get_distance_to_target(&self) -> f32 {
        self.distance_to_target
    }

    pub fn get_despawn_at_target(&self) -> bool {
        self.despawn_at_target
    }

    pub fn set_target(&mut self, new_target: Option<Vector2>) {
        if let Some(target) = new_target {
            self.target = Some(world_bounded_vector2(target))
        } else {
            self.target = None;
        }
        self.revision += 1;
    }
}
