use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use pathfinding::grid::Grid;
use pathfinding::prelude::astar;

use crate::{
    game::systems::prelude::{GRID_HEIGHT, GRID_SIZE_X_MIN, GRID_SIZE_Y_MIN, GRID_WIDTH},
    utils::get_game_time,
};

const GRID_CELL_SIZE: u32 = 16;

#[derive(Resource)]
pub struct PathfinderState {
    pub grid: Grid,
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

    fn create_grid() -> Grid {
        let mut grid = Grid::new(
            (GRID_WIDTH / GRID_CELL_SIZE).try_into().unwrap(),
            (GRID_HEIGHT / GRID_CELL_SIZE).try_into().unwrap(),
        );
        grid.enable_diagonal_mode();
        grid.fill();
        grid
    }

    pub fn reset(&mut self) {
        self.grid = PathfinderState::create_grid();
        self.last_update_at_millis = get_game_time();
    }

    pub fn remove_vertex_in_rect(&mut self, position: &Vector2, rect: &Vector2) {
        let xoffset = 0 - GRID_SIZE_X_MIN.round() as i32;
        let yoffset = 0 - GRID_SIZE_Y_MIN.round() as i32;
        let rx = (f32::ceil(((position.x - 15.0) - (rect.x) / 2.0) / GRID_CELL_SIZE as f32)
            * GRID_CELL_SIZE as f32) as i32;
        let ry = (f32::ceil(((position.y - 15.0) - (rect.y - 15.0) / 2.0) / GRID_CELL_SIZE as f32)
            * GRID_CELL_SIZE as f32) as i32;
        let rw = rect.x.round() as i32 + 15;
        let rh = rect.y.round() as i32 + 15;

        for x in (rx..(rx + rw)).step_by(GRID_CELL_SIZE as usize) {
            for y in (ry..(ry + rh)).step_by(GRID_CELL_SIZE as usize) {
                self.grid.remove_vertex((
                    ((x + xoffset) / GRID_CELL_SIZE as i32) as usize,
                    ((y + yoffset) / GRID_CELL_SIZE as i32) as usize,
                ));
            }
        }
    }

    fn round_to_vertex(&self, vec: Vector2) -> (usize, usize) {
        (
            (vec.x / GRID_CELL_SIZE as f32).round() as usize,
            (vec.y / GRID_CELL_SIZE as f32).round() as usize,
        )
    }

    pub fn get_path(&mut self, from: &Vector2, to: &Vector2) -> Option<Vec<Vector2>> {
        let xoffset = 0.0 - GRID_SIZE_X_MIN;
        let yoffset = 0.0 - GRID_SIZE_Y_MIN;

        let from_vertex = self.round_to_vertex(Vector2::new(from.x + xoffset, from.y + yoffset));
        let to_vertex = self.round_to_vertex(Vector2::new(to.x + xoffset, to.y + yoffset));
        // println!("{:#?}", from_vertex);
        // println!("{:#?}", to_vertex);
        // println!("{:#?}", self.grid.size());
        // println!("{:#?}", self.grid);
        if !self.grid.has_vertex(to_vertex) {
            println!("[PathfinderState][get_path] Cannot find to.");
            return None;
        }
        self.grid.add_vertex(from_vertex);

        let opt_path = astar(
            &from_vertex,
            |p| {
                self.grid
                    .neighbours((p.0, p.1))
                    .into_iter()
                    .map(|p| ((p.0, p.1), 1))
                    .collect::<Vec<_>>()
            },
            |&(x, y)| ((to_vertex.0.abs_diff(x) + to_vertex.1.abs_diff(y)) / 3) as i32,
            |p| *p == to_vertex,
        );

        self.grid.remove_vertex(from_vertex);

        if let Some((path, _)) = opt_path {
            // println!("{:#?}", path);
            let vec_path = path
                .into_iter()
                .map(|(x, y)| {
                    Vector2::new(
                        ((x as u32) * GRID_CELL_SIZE) as f32 - xoffset,
                        ((y as u32) * GRID_CELL_SIZE) as f32 - yoffset,
                    )
                })
                .collect();

            return Some(vec_path);
        }

        println!("[PathfinderState][get_path] Cannot find path.");
        None
    }
}
