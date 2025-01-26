use bevy::prelude::*;

use crate::shared::RENDER_TO_NAV_TILE_MULTI;

use super::{tile_kind::RenderTileKind, Map, NavTile, NavTileCoord};

pub fn generate_map(mut commands: Commands, mut map_grid: ResMut<Map>) {
    map_grid.reset(UVec2::new(40, 40));

    for x_render in 0..map_grid.render_map_size.x {
        for y_render in 0..map_grid.render_map_size.y {
            // Insert an mark all nav tiles as walkable by default
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

            // Top left
            if x_render == 0 && y_render == (map_grid.render_map_size.y - 1) {
                map_grid.add_tile(
                    &mut commands,
                    RenderTileKind::LeftPartOfNorthCornerWal,
                    UVec2::new(x_render, y_render),
                );
                map_grid.add_tile(
                    &mut commands,
                    RenderTileKind::RightPartOfNorthCornerWal,
                    UVec2::new(x_render, y_render),
                );
            }
            // Top Right
            else if x_render == (map_grid.render_map_size.x - 1)
                && y_render == (map_grid.render_map_size.y - 1)
            {
                map_grid.add_tile(
                    &mut commands,
                    RenderTileKind::LeftEndWall,
                    UVec2::new(x_render, y_render),
                );
            }
            // Bottom Right
            else if x_render == (map_grid.render_map_size.x - 1) && y_render == 0 {
                map_grid.add_tile(
                    &mut commands,
                    RenderTileKind::SouthCornerWall,
                    UVec2::new(x_render, y_render),
                );
            }
            // Bottom Left
            else if x_render == 0 && y_render == 0 {
                map_grid.add_tile(
                    &mut commands,
                    RenderTileKind::RightEndWall,
                    UVec2::new(x_render, y_render),
                );
            }
            // Top/Bottom
            else if y_render == (map_grid.render_map_size.y - 1) || y_render == 0 {
                map_grid.add_tile(
                    &mut commands,
                    RenderTileKind::RightWall,
                    UVec2::new(x_render, y_render),
                );
            }
            // Left/Right
            else if x_render == (map_grid.render_map_size.x - 1) || x_render == 0 {
                map_grid.add_tile(
                    &mut commands,
                    RenderTileKind::LeftWall,
                    UVec2::new(x_render, y_render),
                );
            }
        }
    }

    for offset in [
        [4, 10],
        [4, 20],
        [4, 30],
        //
        [12, 10],
        [12, 20],
        [12, 30],
        [20, 10],
        [20, 30],
        //
        [28, 10],
        [28, 20],
        [28, 30],
    ] {
        let t = [
            [[0, 0], [0, 0], [0, 0], [0, 0], [5, 0], [0, 0], [0, 0]], //
            [[4, 3], [2, 0], [2, 0], [2, 0], [4, 3], [2, 0], [7, 0]], //
            [[1, 0], [0, 0], [0, 0], [0, 0], [1, 0], [0, 0], [0, 0]], //
            [[1, 0], [0, 0], [0, 0], [0, 0], [1, 0], [0, 0], [0, 0]], //
            [[1, 0], [0, 0], [0, 0], [0, 0], [1, 0], [0, 0], [0, 0]], //
            [[1, 0], [0, 0], [0, 0], [0, 0], [1, 0], [0, 0], [0, 0]], //
            [[6, 0], [2, 0], [91, 0], [92, 0], [7, 0], [0, 0], [0, 0]], //
        ];

        for (y, row) in t.iter().enumerate() {
            for (x, tiles) in row.iter().enumerate() {
                for v in tiles {
                    if *v == 0 {
                        continue;
                    }
                    let kind = match v {
                        1 => RenderTileKind::LeftWall,
                        2 => RenderTileKind::RightWall,
                        3 => RenderTileKind::RightPartOfNorthCornerWal,
                        4 => RenderTileKind::LeftPartOfNorthCornerWal,
                        5 => RenderTileKind::LeftEndWall,
                        6 => RenderTileKind::RightEndWall,
                        7 => RenderTileKind::SouthCornerWall,
                        91 => RenderTileKind::RightWallWithDoorRight,
                        92 => RenderTileKind::RightWallWithDoorLeft,
                        _ => panic!("Invalid tile id"),
                    };

                    map_grid.add_tile(
                        &mut commands,
                        kind,
                        UVec2::new(offset[0] + x as u32, offset[1] - y as u32),
                    );
                }
            }
        }
    }
}
