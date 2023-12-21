use std::collections::HashMap;

use arrayvec::ArrayVec;
use godot::builtin::Vector2;
use rust_common::helper::get_timestamp_millis;

struct Node {
    pub g: f32,
    pub h: f32,
    pub f: f32,
    pub is_open: bool,
    pub is_closed: bool,
    pub is_path: bool,
    pub is_blocked: bool,
    pub parent: Option<(usize, usize)>,
    pub previous_open: Option<(usize, usize)>,
    pub next_open: Option<(usize, usize)>,
}
impl Node {
    pub fn new() -> Self {
        Self {
            g: 0.0,
            h: 0.0,
            f: 0.0,
            is_open: false,
            is_closed: false,
            is_path: false,
            is_blocked: false,
            parent: None,
            previous_open: None,
            next_open: None,
        }
    }
}

const PATHFINDER_GRID_SIZE: usize = 30;
const PATHFINDER_TILE_SIZE: f32 = 60.0;
type Grid = ArrayVec<ArrayVec<Node, PATHFINDER_GRID_SIZE>, PATHFINDER_GRID_SIZE>;

pub fn test_pathfinder() {
    println!("Test pathfinder");
    let mut grid: Grid =
        ArrayVec::<ArrayVec<Node, PATHFINDER_GRID_SIZE>, PATHFINDER_GRID_SIZE>::new();
    // Init grid
    for x in 0..PATHFINDER_GRID_SIZE {
        grid.insert(x, ArrayVec::<Node, PATHFINDER_GRID_SIZE>::new());
        for y in 0..PATHFINDER_GRID_SIZE {
            grid[x].insert(y, Node::new());
        }
    }

    grid[15][10].is_blocked = true;
    grid[15][11].is_blocked = true;
    grid[15][12].is_blocked = true;
    grid[15][13].is_blocked = true;
    grid[15][14].is_blocked = true;
    grid[15][15].is_blocked = true;
    grid[15][16].is_blocked = true;
    grid[15][17].is_blocked = true;
    grid[15][18].is_blocked = true;
    grid[15][19].is_blocked = true;

    grid[14][19].is_blocked = true;
    grid[13][19].is_blocked = true;
    grid[12][19].is_blocked = true;
    grid[11][19].is_blocked = true;
    grid[10][19].is_blocked = true;
    grid[9][19].is_blocked = true;
    grid[8][19].is_blocked = true;
    grid[7][19].is_blocked = true;
    grid[6][19].is_blocked = true;
    grid[5][19].is_blocked = true;

    let start = (0, 0);
    let goal = (13, 29);

    let start_node = &mut grid[start.0][start.1];
    start_node.is_open = true;

    let mut current_open;
    let mut opt_open_tail = Some(start);

    let neighbours_coords: [(i32, i32); 8] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let t = get_timestamp_millis();

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
            panic!("no path")
        }

        if current_open == goal {
            println!("Path found!");
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
            if grid[neighbour.0][neighbour.1].is_closed || grid[neighbour.0][neighbour.1].is_blocked
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
                // let h = (neighbour.0.abs_diff(goal.0) + neighbour.1.abs_diff(goal.1)) as f32;
                // let h = i32::pow(neighbour.0 as i32 - goal.0 as i32, 2) as f32
                //     + i32::pow(neighbour.1 as i32 - goal.1 as i32, 2) as f32;
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
                }
            }
        }
        // display_grid_open(&grid);
    }

    let mut current_node = &grid[goal.0][goal.1];
    let mut path = HashMap::new();
    let mut i = 0;
    path.insert(i, goal);
    while let Some((x, y)) = current_node.parent {
        i += 1;
        current_node = &grid[x][y];
        path.insert(i, (x, y));
    }

    let mut check_point: usize = 0;
    let mut current_point: usize = 1;
    while path.get(&(current_point + 1)).is_some() {
        if is_walkable(
            &grid,
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

    println!("{}", get_timestamp_millis() - t);

    for (x, y) in path.values() {
        grid[*x][*y].is_path = true;
    }

    display_grid_path(&grid);
}

fn is_walkable(grid: &Grid, from_tile: (usize, usize), to_tile: (usize, usize)) -> bool {
    let from = Vector2::new(
        (from_tile.0 as f32) * (PATHFINDER_TILE_SIZE - 1.0) + PATHFINDER_TILE_SIZE * 0.5,
        (from_tile.1 as f32) * (PATHFINDER_TILE_SIZE - 1.0) + PATHFINDER_TILE_SIZE * 0.5,
    );
    let to = Vector2::new(
        (to_tile.0 as f32) * (PATHFINDER_TILE_SIZE - 1.0) + PATHFINDER_TILE_SIZE * 0.5,
        (to_tile.1 as f32) * (PATHFINDER_TILE_SIZE - 1.0) + PATHFINDER_TILE_SIZE * 0.5,
    );
    let mut points = vec![from];
    while *points.last().unwrap() != to {
        points.push(
            points
                .last()
                .unwrap()
                .move_toward(to, PATHFINDER_TILE_SIZE / 5.0),
        );
    }
    for point in points {
        let x = f32::floor(point.x / PATHFINDER_TILE_SIZE) as usize;
        let y = f32::floor(point.y / PATHFINDER_TILE_SIZE) as usize;
        if grid[x][y].is_blocked {
            return false;
        }
    }

    true
}

fn display_grid_path(grid: &Grid) {
    let mut str_grid = String::new();
    for x in 0..PATHFINDER_GRID_SIZE {
        for y in 0..PATHFINDER_GRID_SIZE {
            if grid[x][y].is_path {
                str_grid.push('░');
            } else if grid[x][y].is_blocked {
                str_grid.push('▓');
            } else {
                str_grid.push(' ');
            }
        }
        str_grid.push('\n');
    }
    println!("{}", str_grid);
}

// fn display_grid_open(grid: &Grid) {
//     let mut str_grid = String::new();
//     for x in 0..PATHFINDER_GRID_SIZE {
//         for y in 0..PATHFINDER_GRID_SIZE {
//             if grid[x][y].is_open {
//                 str_grid.push('▓');
//             } else if grid[x][y].is_closed {
//                 str_grid.push('░');
//             } else {
//                 str_grid.push(' ');
//             }
//         }
//         str_grid.push('\n');
//     }
//     println!("{}", str_grid);
// }
