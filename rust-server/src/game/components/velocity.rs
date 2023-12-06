use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::helper::get_timestamp_millis;

#[derive(Component)]
pub struct Velocity {
    revision: u32,
    target: Vector2,
    speed: f32,
    timestamp_at_target: u64,
    distance_to_target: f32,
    despawn_at_target: bool,
}
impl Velocity {
    pub fn new(target: Vector2, speed: f32, despawn_at_target: bool) -> Self {
        Self {
            revision: 0,
            target,
            speed,
            timestamp_at_target: get_timestamp_millis() as u64,
            distance_to_target: 0.0,
            despawn_at_target,
        }
    }

    pub fn get_target(&self) -> &Vector2 {
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

    pub fn set_target(&mut self, from_position: Vector2, new_target: Vector2) {
        self.distance_to_target = from_position.distance_to(new_target);
        self.timestamp_at_target = if self.speed > 0.0 {
            (get_timestamp_millis()
                + (((self.distance_to_target / self.speed) * 1000.0).round() as u128))
                as u64
        } else {
            get_timestamp_millis() as u64
        };
        self.target = new_target;
        self.revision += 1;
    }
}
