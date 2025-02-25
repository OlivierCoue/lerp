use std::collections::VecDeque;

use avian2d::prelude::Position;
use bevy::{math::UVec2, prelude::*, utils::HashMap};
use lightyear::prelude::{client::Predicted, server::ReplicationTarget};

use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum FlowFieldDirection {
    North,
    South,
    West,
    East,
    NorthWest,
    SouthWest,
    SouthEast,
    NorthEast,
}
impl FlowFieldDirection {
    pub fn to_normalized_velocity(&self) -> Vec2 {
        match self {
            FlowFieldDirection::North => Vec2::new(0.0, 1.0),
            FlowFieldDirection::South => Vec2::new(0.0, -1.0),
            FlowFieldDirection::West => Vec2::new(-1.0, 0.0),
            FlowFieldDirection::East => Vec2::new(1.0, 0.0),
            FlowFieldDirection::NorthWest => Vec2::new(-1.0, 1.0).normalize(),
            FlowFieldDirection::SouthWest => Vec2::new(-1.0, -1.0).normalize(),
            FlowFieldDirection::SouthEast => Vec2::new(1.0, -1.0).normalize(),
            FlowFieldDirection::NorthEast => Vec2::new(1.0, 1.0).normalize(),
        }
    }
}

pub struct FlowFieldNode {
    pub direction: FlowFieldDirection,
    pub distance: u32,
}

#[derive(Resource, Default)]
pub struct FlowField {
    pub map: HashMap<NavTileCoord, FlowFieldDirection>,
    pub size: UVec2,
}
impl FlowField {
    pub fn get_direction_from_position(
        &self,
        map_grid: &Map,
        position: &Position,
    ) -> Option<&FlowFieldDirection> {
        let map_node_pos = map_grid.position_to_nav_map_tile_coord(position);
        self.map.get(&map_node_pos)
    }
}

/// Directions for neighbor traversal
const DIRECTIONS: [(i32, i32, FlowFieldDirection); 8] = [
    (0, 1, FlowFieldDirection::South),
    (0, -1, FlowFieldDirection::North),
    (-1, 0, FlowFieldDirection::East),
    (1, 0, FlowFieldDirection::West),
    (-1, 1, FlowFieldDirection::SouthEast),
    (1, 1, FlowFieldDirection::SouthWest),
    (-1, -1, FlowFieldDirection::NorthEast),
    (1, -1, FlowFieldDirection::NorthWest),
];
/// Max search distance in number of nav tiles
const MAX_SEACH_DISTANCE: u32 = 40;

pub fn reset_flow_field(mut flow_field: ResMut<FlowField>) {
    flow_field.map.clear();
    flow_field.size = UVec2::ZERO;
}

pub fn update_flow_field(
    map_grid: Res<Map>,
    mut flow_field: ResMut<FlowField>,
    player_q: Query<&Position, (With<Player>, Or<(With<Predicted>, With<ReplicationTarget>)>)>,
) {
    // Get all goal positions
    let goals: Vec<Position> = player_q.iter().copied().collect();

    flow_field.map.clear();
    flow_field.size = map_grid.nav_map_size;

    // Initialize the visited map
    let mut visited = HashMap::new();

    // Create a separate queue for each goal
    let mut queues: Vec<VecDeque<(NavTileCoord, u32)>> =
        goals.iter().map(|_| VecDeque::new()).collect();

    // Initialize each goal's BFS queue with distance 0
    for (i, goal_position) in goals.iter().enumerate() {
        let goal_map_node_pos = map_grid.position_to_nav_map_tile_coord(goal_position);
        queues[i].push_back((goal_map_node_pos, 0));
        visited.insert(goal_map_node_pos, None); // None indicates this is a goal
    }

    // Process the BFS in a round-robin manner (pop once per goal)
    while queues.iter().any(|q| !q.is_empty()) {
        // Iterate over each goal's queue
        for queue in &mut queues {
            // Skip if the queue for this goal is empty
            if queue.is_empty() {
                continue;
            }

            if let Some((current, distance)) = queue.pop_front() {
                // If we've exceeded the maximum search distance, stop exploring further
                if distance >= MAX_SEACH_DISTANCE {
                    continue;
                }

                let current_pos = current.0;

                for (dx, dy, direction) in DIRECTIONS.iter() {
                    let neighbor_pos = NavTileCoord(UVec2::new(
                        (current_pos.x as i32 + dx) as u32,
                        (current_pos.y as i32 + dy) as u32,
                    ));

                    // Skip out-of-bounds or already visited nodes
                    if visited.contains_key(&neighbor_pos) {
                        continue;
                    }

                    // Check if the neighbor is walkable
                    if let Some(neighbor_node) = map_grid.nav_map.get(&neighbor_pos) {
                        if neighbor_node.walkable {
                            // Mark the neighbor as visited and record its direction
                            visited.insert(neighbor_pos, Some(*direction));
                            queue.push_back((neighbor_pos, distance + 1));
                        }
                    }
                }
            }
        }
    }

    // Populate the flow field based on the visited map from all goals
    for (&node_pos, &direction) in visited.iter() {
        if let Some(dir) = direction {
            flow_field.map.insert(node_pos, dir);
        }
    }
}
