use crate::map::{TileFloor, TileMapFloor};

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileTextureIndex};
use rust_common_game::flow_field::{FlowField, FlowFieldDirection};
use rust_common_game::map::NavMapPos;

use super::{DebugConfig, RenderConfig, RenderMode};

pub fn debug_render_flow_field(
    debug_config: Res<DebugConfig>,
    render_config: Res<RenderConfig>,
    mut tilemap_q: Query<&TileStorage, With<TileMapFloor>>,
    mut tile_q: Query<&mut TileTextureIndex, With<TileFloor>>,
    flow_field: Res<FlowField>,
) {
    if !debug_config.show_flow_field || render_config.mode == RenderMode::Cart {
        return;
    }

    let Ok(tile_storage) = tilemap_q.get_single_mut() else {
        return;
    };

    for x in 0..flow_field.size.x {
        for y in 0..flow_field.size.y {
            let map_node_pos = NavMapPos(UVec2::new(x, y));
            let Some(flow_field_direction) = flow_field.map.get(&map_node_pos) else {
                continue;
            };

            let Some(tile_entity) = tile_storage.get(&TilePos::new(map_node_pos.x, map_node_pos.y))
            else {
                error!(
                    "[debug_render_flow_field] Missing tile {}:{} in TileStorage (Floor)",
                    x, y
                );
                continue;
            };

            let Ok(mut tile_texture_index) = tile_q.get_mut(tile_entity) else {
                error!(
                    "[debug_render_flow_field] Missing tile texture index {}:{} in TileTextureIndex (Floor)",
                    x, y
                );
                continue;
            };

            tile_texture_index.0 = match flow_field_direction {
                FlowFieldDirection::North => 13,
                FlowFieldDirection::South => 14,
                FlowFieldDirection::West => 16,
                FlowFieldDirection::East => 17,
                FlowFieldDirection::NorthWest => 18,
                FlowFieldDirection::SouthWest => 19,
                FlowFieldDirection::SouthEast => 20,
                FlowFieldDirection::NorthEast => 21,
            }
        }
    }
}
