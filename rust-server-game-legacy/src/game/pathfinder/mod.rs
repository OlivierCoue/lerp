use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use rust_common::math::Vec2;

pub struct Node {
    pub x: f32,
    pub y: f32,
    pub g: f32,
    pub h: f32,
    pub f: f32,
    pub is_open: bool,
    pub is_closed: bool,
    pub is_blocked: bool,
    pub blocked_by: Vec<Entity>,
    pub parent: Option<(usize, usize)>,
    pub previous_open: Option<(usize, usize)>,
    pub next_open: Option<(usize, usize)>,
    pub is_path: bool,
    pub display: Option<char>,
}
impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            g: 0.0,
            h: 0.0,
            f: 0.0,
            is_open: false,
            is_closed: false,
            is_blocked: false,
            blocked_by: Vec::new(),
            parent: None,
            previous_open: None,
            next_open: None,
            is_path: false,
            display: None,
        }
    }

    pub fn new_blocked(x: f32, y: f32, blocked: bool, blocked_by: Vec<Entity>) -> Self {
        Self {
            x,
            y,
            g: 0.0,
            h: 0.0,
            f: 0.0,
            is_open: false,
            is_closed: false,
            is_blocked: blocked,
            blocked_by,
            parent: None,
            previous_open: None,
            next_open: None,
            is_path: false,
            display: None,
        }
    }
}
impl Node {
    pub fn is_blocked_for(&self, entity: &Entity) -> bool {
        self.is_blocked && !self.blocked_by.contains(entity)
    }
    pub fn set_is_blocked(&mut self, is_blocked: bool) {
        self.is_blocked = is_blocked
    }
    pub fn add_is_blocked_by(&mut self, entity: Entity) {
        self.blocked_by.push(entity)
    }
}

pub const PATHFINDER_GRID_SIZE: usize = 40;
pub const PATHFINDER_TILE_SIZE: f32 = 30.0;
pub type Grid = [[Box<Node>; PATHFINDER_GRID_SIZE]; PATHFINDER_GRID_SIZE];

pub fn pathfinder_get_path(
    mut grid: Grid,
    entity: Entity,
    from: Vec2,
    to: Vec2,
    grid_to_left_node: (usize, usize),
) -> Option<Vec<Vec2>> {
    let unsafe_start = (
        f32::floor(from.x) as i32 / PATHFINDER_TILE_SIZE as i32 - grid_to_left_node.0 as i32,
        f32::floor(from.y) as i32 / PATHFINDER_TILE_SIZE as i32 - grid_to_left_node.1 as i32,
    );
    let unsafe_goal = (
        f32::floor(to.x) as i32 / PATHFINDER_TILE_SIZE as i32 - grid_to_left_node.0 as i32,
        f32::floor(to.y) as i32 / PATHFINDER_TILE_SIZE as i32 - grid_to_left_node.1 as i32,
    );

    if unsafe_start.0 < 0
        || unsafe_start.0 > PATHFINDER_GRID_SIZE as i32 - 1
        || unsafe_start.1 < 0
        || unsafe_start.1 > PATHFINDER_GRID_SIZE as i32 - 1
        || unsafe_goal.0 < 0
        || unsafe_goal.0 > PATHFINDER_GRID_SIZE as i32 - 1
        || unsafe_goal.1 < 0
        || unsafe_goal.1 > PATHFINDER_GRID_SIZE as i32 - 1
    {
        return Some(vec![to]);
    }

    let start = (unsafe_start.0 as usize, unsafe_start.1 as usize);
    let goal = (unsafe_goal.0 as usize, unsafe_goal.1 as usize);

    let goal_node = &mut grid[goal.0][goal.1];
    goal_node.display = Some('G');

    let start_node = &mut grid[start.0][start.1];
    start_node.is_open = true;
    start_node.display = Some('S');
    let start_x = start_node.x;
    let start_y = start_node.y;

    let mut current_open;
    let mut opt_open_tail = Some(start);

    let neighbours_coords: [(i32, i32); 4] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        // (-1, -1),
        // (-1, 1),
        // (1, -1),
        // (1, 1),
    ];

    let mut closest_to_goal = start;
    let mut clostest_to_goal_h = 99999.0;

    #[allow(clippy::while_let_loop)]
    loop {
        if let Some(open_tail) = opt_open_tail {
            current_open = open_tail;
            let mut looper: (usize, usize) = open_tail;
            while let Some((x, y)) = grid[looper.0][looper.1].previous_open {
                if grid[x][y].f < grid[current_open.0][current_open.1].f {
                    current_open = (x, y);
                }
                looper = (x, y);
            }

            if current_open == open_tail {
                opt_open_tail = grid[current_open.0][current_open.1].previous_open;
            }
        } else {
            // If no path is found, we return only the destination as a path.
            break;
        }

        // Stop if path is found
        if current_open == goal {
            break;
        }

        // Remove current_open from the open list
        let n_current_open = &mut grid[current_open.0][current_open.1];
        n_current_open.is_open = false;
        n_current_open.is_closed = true;
        if let (Some(previous), Some(next)) =
            (n_current_open.previous_open, n_current_open.next_open)
        {
            grid[previous.0][previous.1].next_open = Some(next);
            grid[next.0][next.1].previous_open = Some(previous);
        } else if let Some(previous) = n_current_open.previous_open {
            grid[previous.0][previous.1].next_open = None;
        } else if let Some(next) = n_current_open.next_open {
            grid[next.0][next.1].previous_open = None;
        }
        grid[current_open.0][current_open.1].previous_open = None;
        grid[current_open.0][current_open.1].next_open = None;

        // Generate neighbours
        let mut neighbours = Vec::new();
        for neighbour_coord in neighbours_coords {
            let neighbour_i32 = (
                current_open.0 as i32 + neighbour_coord.0,
                current_open.1 as i32 + neighbour_coord.1,
            );
            if neighbour_i32.0 < 0
                || neighbour_i32.0 > PATHFINDER_GRID_SIZE as i32 - 1
                || neighbour_i32.1 < 0
                || neighbour_i32.1 > PATHFINDER_GRID_SIZE as i32 - 1
            {
                continue;
            }

            neighbours.push((neighbour_i32.0 as usize, neighbour_i32.1 as usize));
        }

        // Loop over neighbours
        for neighbour in neighbours {
            if grid[neighbour.0][neighbour.1].is_closed
                || grid[neighbour.0][neighbour.1].is_blocked_for(&entity)
            {
                continue;
            }

            let g: f32 = grid[current_open.0][current_open.1].g + 1.0;

            if grid[neighbour.0][neighbour.1].g <= grid[current_open.0][current_open.1].g {
                let dx = f32::abs(neighbour.0 as f32 - goal.0 as f32);
                let dy = f32::abs(neighbour.1 as f32 - goal.1 as f32);
                let d = 1.0;
                let d2 = std::f32::consts::SQRT_2;

                let mut h = d * (dx + dy) + (d2 - 2.0 * d) * f32::min(dx, dy);

                let dx1 = (current_open.0 as i32 - goal.0 as i32) as f32;
                let dy1 = (current_open.1 as i32 - goal.1 as i32) as f32;
                let dx2 = (start.0 as i32 - goal.0 as i32) as f32;
                let dy2 = (start.1 as i32 - goal.1 as i32) as f32;
                h += f32::abs(dx1 * dy2 - dx2 * dy1) * 0.001;

                let f = g + h;

                grid[neighbour.0][neighbour.1].g = g;
                grid[neighbour.0][neighbour.1].h = h;
                grid[neighbour.0][neighbour.1].f = f;
                grid[neighbour.0][neighbour.1].parent = Some(current_open);

                if !grid[neighbour.0][neighbour.1].is_open {
                    grid[neighbour.0][neighbour.1].is_open = true;
                    if let Some(open_tail) = opt_open_tail {
                        grid[neighbour.0][neighbour.1].previous_open = Some(open_tail);
                        grid[open_tail.0][open_tail.1].next_open = Some(neighbour);
                    }
                    opt_open_tail = Some(neighbour);
                    if h < clostest_to_goal_h {
                        clostest_to_goal_h = h;
                        closest_to_goal = neighbour;
                    }
                }
            }
        }
    }

    if closest_to_goal == start {
        return Some(vec![]);
    }

    // Create path in a hashmap
    let mut current_node = &grid[closest_to_goal.0][closest_to_goal.1];
    let mut path = HashMap::new();
    let mut i: usize = 0;
    path.insert(i, closest_to_goal);
    while let Some((x, y)) = current_node.parent {
        i += 1;
        current_node = &grid[x][y];
        path.insert(i, (x, y));
    }

    // Smooth path (only keep required points)
    let mut check_point: usize = 0;
    let mut current_point: usize = 1;
    while path.contains_key(&(current_point + 1)) {
        if is_walkable(
            &grid,
            &entity,
            *path.get(&check_point).unwrap(),
            *path.get(&(current_point + 1)).unwrap(),
        ) {
            let temp = current_point;
            current_point += 1;
            path.remove(&temp);
        } else {
            check_point = current_point;
            current_point += 1;
        }
    }

    // Remove the starting point
    // if start_x != from.x && start_y != from.y {
    //     path.remove(&i);
    // }

    path.remove(&i);

    for (x, y) in path.values() {
        grid[*x][*y].is_path = true;
    }

    // Transform hashmap path to vector (in the correct order)
    let mut path_vec = path.iter().collect::<Vec<_>>();
    path_vec.sort_by(|a, b| b.0.cmp(a.0));

    let path_vec_vector_2d = path_vec
        .iter()
        .map(|(_, (x, y))| Vec2::new(grid[*x][*y].x, grid[*x][*y].y))
        .collect::<Vec<_>>();

    // let to_coord = Vec2::new(
    //     (goal.0 as f32 + grid_to_left_node.0 as f32 + 0.5) * PATHFINDER_TILE_SIZE,
    //     (goal.1 as f32 + grid_to_left_node.1 as f32 + 0.5) * PATHFINDER_TILE_SIZE,
    // );

    // println!("{:#?}", path_vec_vector_2d);
    // Update the last point to the exact goal coordonate
    // let len = path_vec_vector_2d.len();
    // if len > 0 && path_vec_vector_2d.get(len - 1).is_some() {
    //     path_vec_vector_2d[len - 1] = to_coord;
    // } else {
    //     path_vec_vector_2d.push(to_coord);
    // }
    // path_vec_vector_2d.push(to);

    Some(path_vec_vector_2d)
}

fn is_walkable(
    grid: &Grid,
    entity: &Entity,
    from_tile: (usize, usize),
    to_tile: (usize, usize),
) -> bool {
    let from: Vec2 = Vec2::new(
        (from_tile.0 as f32) * (PATHFINDER_TILE_SIZE) + PATHFINDER_TILE_SIZE / 2.0,
        (from_tile.1 as f32) * (PATHFINDER_TILE_SIZE) + PATHFINDER_TILE_SIZE / 2.0,
    );
    let to = Vec2::new(
        (to_tile.0 as f32) * (PATHFINDER_TILE_SIZE) + PATHFINDER_TILE_SIZE / 2.0,
        (to_tile.1 as f32) * (PATHFINDER_TILE_SIZE) + PATHFINDER_TILE_SIZE / 2.0,
    );

    let dx = to.x - from.x;
    let dy = to.y - from.y;

    // Normalize direction
    let length = (dx * dx + dy * dy).sqrt();
    if length == 0.0 {
        return !grid[from_tile.0][from_tile.1].is_blocked_for(entity);
    }

    // Compute the offsets for the object's width
    let object_width = 28.0;
    let offset_x = -dy / length * (object_width / 2.0);
    let offset_y = dx / length * (object_width / 2.0);

    let start_left = Vec2::new(from.x + offset_x, from.y + offset_y);
    let end_left = Vec2::new(to.x + offset_x, to.y + offset_y);

    let mut points_1 = vec![start_left];

    while *points_1.last().unwrap() != end_left {
        points_1.push(
            points_1
                .last()
                .unwrap()
                .move_toward(end_left, PATHFINDER_TILE_SIZE / 5.0),
        );
    }
    for point in points_1 {
        let x: usize = f32::floor(point.x / PATHFINDER_TILE_SIZE) as usize;
        let y = f32::floor(point.y / PATHFINDER_TILE_SIZE) as usize;
        if grid[x][y].is_blocked_for(entity) {
            return false;
        }
    }

    let start_right = Vec2::new(from.x - offset_x, from.y - offset_y);
    let end_right = Vec2::new(to.x - offset_x, to.y - offset_y);

    let mut points_2 = vec![start_right];

    while *points_2.last().unwrap() != end_right {
        points_2.push(
            points_2
                .last()
                .unwrap()
                .move_toward(end_right, PATHFINDER_TILE_SIZE / 5.0),
        );
    }
    for point in points_2 {
        let x: usize = f32::floor(point.x / PATHFINDER_TILE_SIZE) as usize;
        let y = f32::floor(point.y / PATHFINDER_TILE_SIZE) as usize;
        if grid[x][y].is_blocked_for(entity) {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
fn display_grid_path(grid: &Grid) {
    let mut str_grid = String::new();
    for row in grid.iter().take(PATHFINDER_GRID_SIZE) {
        for cell in row.iter().take(PATHFINDER_GRID_SIZE) {
            if let Some(display) = cell.display {
                str_grid.push(display);
            } else if cell.is_path {
                str_grid.push('░');
            } else if cell.is_blocked {
                str_grid.push('▓');
            } else {
                str_grid.push(' ');
            }
        }
        str_grid.push('\n');
    }
    println!("{}", str_grid);
}
