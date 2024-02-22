use std::collections::VecDeque;

use bevy_ecs::prelude::*;
use rust_common::math::Vec2;

use crate::game::ecs::{resources::prelude::AreaConfig, systems::prelude::*};

#[derive(Component)]
pub struct Velocity {
    pub revision: u32,
    pub revision_checkpoint: u32,
    target_queue: VecDeque<Vec2>,
    speed: f32,
    despawn_at_target: bool,
}
impl Velocity {
    pub fn new(target: Option<Vec2>, speed: f32, despawn_at_target: bool) -> Self {
        let target_queue = match target {
            Some(t) => VecDeque::from_iter(Some(t)),
            None => VecDeque::new(),
        };

        Self {
            revision: 1,
            revision_checkpoint: 0,
            target_queue,
            speed,
            despawn_at_target,
        }
    }

    pub fn get_target(&self) -> Option<&Vec2> {
        self.target_queue.get(0)
    }

    pub fn get_target_queue(&self) -> Vec<Vec2> {
        Vec::from_iter(self.target_queue.clone())
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_despawn_at_target(&self) -> bool {
        self.despawn_at_target
    }

    pub fn set_target(&mut self, area_config: &AreaConfig, new_target: Option<Vec2>) {
        if let Some(target) = new_target {
            self.target_queue =
                VecDeque::from_iter(Some(world_bounded_vector2(area_config, target)))
        } else {
            self.target_queue = VecDeque::new();
        }
        self.revision += 1;
    }

    pub fn set_targets(&mut self, new_targets: Vec<Vec2>) {
        self.target_queue = VecDeque::from_iter(new_targets);
        self.revision += 1;
    }

    pub fn add_target(&mut self, area_config: &AreaConfig, new_target: Vec2) {
        self.target_queue
            .push_front(world_bounded_vector2(area_config, new_target));
    }

    pub fn remove_current_target(&mut self) -> bool {
        self.target_queue.pop_front();
        if self.target_queue.is_empty() {
            self.revision += 1;

            return true;
        }
        false
    }
}
