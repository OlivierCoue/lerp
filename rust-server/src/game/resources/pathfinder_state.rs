use std::{
    array,
    thread::{self, JoinHandle},
};

use bevy_ecs::prelude::*;
use godot::builtin::Vector2;

use crate::{
    game::{
        pathfinder::{pathfinder_get_path, Grid, Node, PATHFINDER_GRID_SIZE, PATHFINDER_TILE_SIZE},
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
            update_every_millis: 1000,
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
        let extra = 20;
        let rx = (f32::floor(((position.x) - (rect.x) / 2.0 - extra as f32) / PATHFINDER_TILE_SIZE)
            * PATHFINDER_TILE_SIZE) as i32;
        let ry = (f32::floor(((position.y) - (rect.y) / 2.0 - extra as f32) / PATHFINDER_TILE_SIZE)
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

    pub fn get_path_async(
        &mut self,
        entity: Entity,
        from: Vector2,
        to: Vector2,
    ) -> JoinHandle<Option<Vec<Vector2>>> {
        // The global grid is the grid of the global map, in order to find a path we only work with a grid of PATHFINDER_GRID_SIZE * PATHFINDER_GRID_SIZE size (60x60)
        // So we find the node between the from and to (center_node), and then we create the sub grid with the center_node in the center of the sub grid
        let center_node = (
            f32::ceil(((from.x + to.x) / 2.0) / PATHFINDER_TILE_SIZE) as i32,
            f32::ceil(((from.y + to.y) / 2.0) / PATHFINDER_TILE_SIZE) as i32,
        );
        let top_left_node = (
            i32::min(
                i32::max(center_node.0 - PATHFINDER_GRID_SIZE as i32 / 2, 0),
                PATHFINDER_GRID_SIZE as i32 - 1,
            ) as usize,
            i32::min(
                i32::max(center_node.1 - PATHFINDER_GRID_SIZE as i32 / 2, 0),
                PATHFINDER_GRID_SIZE as i32 - 1,
            ) as usize,
        );

        let grid: Grid = array::from_fn(|x| {
            array::from_fn(|y| {
                let mut opt_node = None;
                if let Some(row) = self.grid.get(x + top_left_node.0) {
                    if let Some(n) = row.get(y + top_left_node.1) {
                        opt_node = Some(n);
                    }
                }
                if let Some(node) = opt_node {
                    return Box::new(Node::new_blocked(
                        node.x,
                        node.y,
                        node.is_blocked,
                        node.blocked_by.clone(),
                    ));
                }

                Box::new(Node::new_blocked(0.0, 0.0, true, Vec::new()))
            })
        });

        thread::spawn(move || pathfinder_get_path(grid, entity, from, to, top_left_node))
    }
}
