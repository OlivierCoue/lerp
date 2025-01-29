use avian2d::prelude::{Collider, Position, RigidBody};
use bevy::{prelude::*, utils::HashMap};

use crate::{
    shared::{NAV_TILE_SIZE, RENDER_TO_NAV_TILE_MULTI},
    utils::CommonPlaySceneTag,
    wall::Wall,
};

use super::{
    input::InputMap,
    map::*,
    tile_kind::{RenderTileFloorKind, RenderTileWallKind},
};

#[derive(Clone, Copy, Debug)]
struct WallSegment {
    start_y: u32,
    height: u32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct WallKey {
    start_x: u32,
    end_x: u32,
}

/// Reset the given Map and load the given InputMap in it
pub fn load_map(commands: &mut Commands, map_grid: &mut Map, input: InputMap) {
    // Reset the map with the input size
    map_grid.reset(UVec2::new(
        input.map.first().unwrap().len() as u32,
        input.map.len() as u32,
    ));

    for x_render in 0..map_grid.render_map_size.x {
        for y_render in 0..map_grid.render_map_size.y {
            let Some(tile_char) = input.get(x_render, y_render) else {
                continue;
            };

            // Insert and mark all nav tiles as walkable by default
            for x_nav in x_render * RENDER_TO_NAV_TILE_MULTI
                ..x_render * RENDER_TO_NAV_TILE_MULTI + RENDER_TO_NAV_TILE_MULTI
            {
                for y_nav in y_render * RENDER_TO_NAV_TILE_MULTI
                    ..y_render * RENDER_TO_NAV_TILE_MULTI + RENDER_TO_NAV_TILE_MULTI
                {
                    map_grid.nav_map.insert(
                        NavTileCoord(UVec2::new(x_nav, y_nav)),
                        NavTile { walkable: true },
                    );
                }
            }

            // Insert tiles based on the given input char
            // This a bit tricky because of how the tiles system works
            let is_top_wall = input
                .get_top(x_render, y_render)
                .is_some_and(|t| *t == 'W' || *t == 'D');
            let is_bottom_wall = input
                .get_bottom(x_render, y_render)
                .is_some_and(|t| *t == 'W' || *t == 'D');
            let is_left_wall = input
                .get_left(x_render, y_render)
                .is_some_and(|t| *t == 'W' || *t == 'D');
            let is_right_wall = input
                .get_right(x_render, y_render)
                .is_some_and(|t| *t == 'W' || *t == 'D');

            let is_top_door = input.get_top(x_render, y_render).is_some_and(|t| *t == 'D');
            let is_bottom_door = input
                .get_bottom(x_render, y_render)
                .is_some_and(|t| *t == 'D');
            let is_left_door = input
                .get_left(x_render, y_render)
                .is_some_and(|t| *t == 'D');
            let is_right_door = input
                .get_right(x_render, y_render)
                .is_some_and(|t| *t == 'D');

            let _is_top_floor = input.get_top(x_render, y_render).is_some_and(|t| *t == 'F');
            let is_bottom_floor = input
                .get_bottom(x_render, y_render)
                .is_some_and(|t| *t == 'F');
            let _is_left_floor = input
                .get_left(x_render, y_render)
                .is_some_and(|t| *t == 'F');
            let is_right_floor = input
                .get_right(x_render, y_render)
                .is_some_and(|t| *t == 'F');
            let is_bottom_right_floor = input
                .get_bottom_right(x_render, y_render)
                .is_some_and(|t| *t == 'F');

            if *tile_char == 'F'
                || *tile_char == 'D'
                || (*tile_char == 'W' && (is_bottom_floor || is_right_floor))
                || (*tile_char == 'W' && is_bottom_right_floor)
            {
                map_grid.add_tile_floor(
                    RenderTileFloorKind::Standard,
                    UVec2::new(x_render, y_render),
                );
            }

            if *tile_char == 'W' {
                if !is_left_wall && !is_right_wall && is_top_wall && is_bottom_wall {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::LeftWall,
                        UVec2::new(x_render, y_render),
                    );
                } else if !is_top_wall && !is_bottom_wall && is_left_wall && is_right_wall {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::RightWall,
                        UVec2::new(x_render, y_render),
                    );
                } else if is_right_wall && is_bottom_wall {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::LeftPartOfNorthCornerWal,
                        UVec2::new(x_render, y_render),
                    );
                    map_grid.add_tile_wall(
                        RenderTileWallKind::RightPartOfNorthCornerWal,
                        UVec2::new(x_render, y_render),
                    );
                } else if !is_right_wall && is_bottom_wall {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::LeftEndWall,
                        UVec2::new(x_render, y_render),
                    );
                } else if !is_bottom_wall && is_right_wall {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::RightEndWall,
                        UVec2::new(x_render, y_render),
                    );
                } else {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::SouthCornerWall,
                        UVec2::new(x_render, y_render),
                    );
                }
            } else if *tile_char == 'D' {
                if is_right_door {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::RightWallWithDoorRight,
                        UVec2::new(x_render, y_render),
                    );
                } else if is_left_door {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::RightWallWithDoorLeft,
                        UVec2::new(x_render, y_render),
                    );
                } else if is_bottom_door {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::LefttWallWithDoorBottom,
                        UVec2::new(x_render, y_render),
                    );
                } else if is_top_door {
                    map_grid.add_tile_wall(
                        RenderTileWallKind::LefttWallWithDoorTop,
                        UVec2::new(x_render, y_render),
                    );
                }
            }
        }
    }

    // Add coliders based on the nav grid state of the map.
    // Nav grid is updated automatically when adding tiles with add_tile_wall for example
    //
    // This done in 2 step:
    //     - Step 1: Identify horizontal wall segments
    //     - Step 2: Merge contiguous vertical walls (aka wall sgements from step 1 with the same start x and end x)

    let mut active_walls: HashMap<WallKey, Vec<WallSegment>> = HashMap::new();

    // Step 1: Identify horizontal wall segments
    for y in 0..map_grid.nav_map_size.y {
        let mut wall_start_at = None;
        for x in 0..map_grid.nav_map_size.x {
            let is_wall = map_grid
                .get_nav_tile(UVec2::new(x, y))
                .map_or(false, |t| !t.walkable);

            if !is_wall {
                continue;
            }

            // Set wall_start_at to current x:y if it is not already set
            wall_start_at.get_or_insert(UVec2::new(x, y));

            // Check if this is the end of a continuous horizontal wall segment
            let next_tile_x = x + 1;
            let next_tile_is_wall = map_grid
                .get_nav_tile(UVec2::new(next_tile_x, y))
                .map_or(true, |t| !t.walkable);

            // We found the end of a segment, we store it
            if !next_tile_is_wall {
                if let Some(wall_start_at) = wall_start_at {
                    active_walls
                        .entry(WallKey {
                            start_x: wall_start_at.x,
                            end_x: next_tile_x,
                        })
                        .or_default()
                        .push(WallSegment {
                            start_y: wall_start_at.y,
                            height: 1,
                        });
                }

                // Reset for the next potential wall
                wall_start_at = None;
            }
        }
    }

    // Step 2: Merge contiguous vertical walls
    for (key, walls) in active_walls.iter_mut() {
        // Sort walls by their y start position (so that we can merge them)
        walls.sort_by(|a, b| a.start_y.cmp(&b.start_y));

        // Merge contiguous walls by checking if their y heights are contiguous
        let mut merged_walls = Vec::new();
        let mut current_wall_merge = walls[0];

        for &current_wall in walls.iter().skip(1) {
            if current_wall.start_y == current_wall_merge.start_y + current_wall_merge.height {
                // Merge by extending the current_wall_merge's height
                // The current_wall is just ignored and not pushed to merged_walls
                current_wall_merge.height += current_wall.height;
            } else {
                // Add the current_wall_merge to merged walls and move to the next wall
                merged_walls.push(current_wall_merge);
                current_wall_merge = current_wall;
            }
        }

        // Add the last wall in the sequence
        merged_walls.push(current_wall_merge);

        // Spawn the merged walls
        for wall in merged_walls.iter() {
            let wall_width = key.end_x - key.start_x;
            let position_x = (key.start_x as f32 + wall_width as f32 / 2.) * NAV_TILE_SIZE
                - map_grid.map_px_half_size.x;
            let position_y = (wall.start_y as f32 + wall.height as f32 / 2.) * NAV_TILE_SIZE
                - map_grid.map_px_half_size.y;

            let position = Position::from_xy(position_x, position_y);
            let collider = Collider::rectangle(
                wall_width as f32 * NAV_TILE_SIZE,
                wall.height as f32 * NAV_TILE_SIZE,
            );

            commands.spawn((
                CommonPlaySceneTag,
                Wall,
                position,
                RigidBody::Static,
                collider,
            ));
        }
    }
}
