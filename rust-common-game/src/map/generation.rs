use bevy::prelude::*;

use crate::shared::RENDER_TO_NAV_TILE_MULTI;

use super::{
    designer::*,
    tile_kind::{RenderTileFloorKind, RenderTileWallKind},
    Map, NavTile, NavTileCoord,
};

pub fn generate_map(mut commands: Commands, mut map_grid: ResMut<Map>) {
    let input = create_small_map_input();
    map_grid.reset(UVec2::new(
        input.map.first().unwrap().len() as u32,
        input.map.len() as u32,
    ));

    for x_render in 0..map_grid.render_map_size.x {
        for y_render in 0..map_grid.render_map_size.y {
            let Some(tile_char) = input.get(x_render, y_render) else {
                continue;
            };

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
                        &mut commands,
                        RenderTileWallKind::LeftWall,
                        UVec2::new(x_render, y_render),
                    );
                } else if !is_top_wall && !is_bottom_wall && is_left_wall && is_right_wall {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::RightWall,
                        UVec2::new(x_render, y_render),
                    );
                } else if is_right_wall && is_bottom_wall {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::LeftPartOfNorthCornerWal,
                        UVec2::new(x_render, y_render),
                    );
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::RightPartOfNorthCornerWal,
                        UVec2::new(x_render, y_render),
                    );
                } else if !is_right_wall && is_bottom_wall {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::LeftEndWall,
                        UVec2::new(x_render, y_render),
                    );
                } else if !is_bottom_wall && is_right_wall {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::RightEndWall,
                        UVec2::new(x_render, y_render),
                    );
                } else {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::SouthCornerWall,
                        UVec2::new(x_render, y_render),
                    );
                }
            } else if *tile_char == 'D' {
                if is_right_door {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::RightWallWithDoorRight,
                        UVec2::new(x_render, y_render),
                    );
                } else if is_left_door {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::RightWallWithDoorLeft,
                        UVec2::new(x_render, y_render),
                    );
                } else if is_bottom_door {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::LefttWallWithDoorBottom,
                        UVec2::new(x_render, y_render),
                    );
                } else if is_top_door {
                    map_grid.add_tile_wall(
                        &mut commands,
                        RenderTileWallKind::LefttWallWithDoorTop,
                        UVec2::new(x_render, y_render),
                    );
                }
            }
        }
    }
}
