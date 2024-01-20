use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

use crate::{
    game::{
        pathfinder::{pathfinder_get_path, Node, PATHFINDER_TILE_SIZE},
        systems::prelude::{GRID_HEIGHT, GRID_WIDTH},
    },
    utils::get_game_time,
};

#[derive(Resource)]
pub struct PathfinderState {
    pub grid: Vec<Vec<Node>>,
    pub last_update_at_millis: u32,
    pub update_every_millis: u32,
}
impl PathfinderState {
    pub fn new() -> Self {
        Self {
            grid: PathfinderState::create_grid(),
            last_update_at_millis: 0,
            update_every_millis: 300,
        }
    }

    fn create_grid() -> Vec<Vec<Node>> {
        let mut grid = Vec::new();
        for x in 0..((GRID_WIDTH / PATHFINDER_TILE_SIZE as u32) + 1) {
            let mut row = Vec::new();
            for y in 0..((GRID_HEIGHT / PATHFINDER_TILE_SIZE as u32) + 1) {
                row.push(Node::new(
                    x as f32 * PATHFINDER_TILE_SIZE + PATHFINDER_TILE_SIZE / 2.0,
                    y as f32 * PATHFINDER_TILE_SIZE + PATHFINDER_TILE_SIZE / 2.0,
                ))
            }
            grid.push(row)
        }

        grid
    }

    pub fn reset(&mut self) {
        self.grid = PathfinderState::create_grid();
        self.last_update_at_millis = get_game_time();
    }

    pub fn block_nodes_in_rect(&mut self, entity: Entity, position: &Vector2, rect: &Vector2) {
        // To take in account the size of the entity moving in the grid, we enlarge every rect by the size of the moving entity
        let extra = 10;
        let rx = (f32::ceil(((position.x) - (rect.x) / 2.0 - extra as f32) / PATHFINDER_TILE_SIZE)
            * PATHFINDER_TILE_SIZE) as i32;
        let ry = (f32::ceil(((position.y) - (rect.y) / 2.0 - extra as f32) / PATHFINDER_TILE_SIZE)
            * PATHFINDER_TILE_SIZE) as i32;
        let rw = rect.x.round() as i32 + 2 * extra;
        let rh = rect.y.round() as i32 + 2 * extra;

        for x in (rx..(rx + rw)).step_by(PATHFINDER_TILE_SIZE as usize) {
            for y in (ry..(ry + rh)).step_by(PATHFINDER_TILE_SIZE as usize) {
                if let Some(row) = self
                    .grid
                    .get_mut(x as usize / PATHFINDER_TILE_SIZE as usize)
                {
                    if let Some(node) = row.get_mut(y as usize / PATHFINDER_TILE_SIZE as usize) {
                        node.set_is_blocked(true);
                        node.add_is_blocked_by(entity);
                    }
                }
            }
        }
    }

    pub fn get_path(
        &mut self,
        entity: Entity,
        from: &Vector2,
        to: &Vector2,
    ) -> Option<Vec<Vector2>> {
        pathfinder_get_path(&self.grid, entity, from, to)
    }
}
