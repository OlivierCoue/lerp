use crate::map::{TileFlowField, TileMapFlowField};

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileTextureIndex};
use rust_common_game::flow_field::{FlowField, FlowFieldDirection};
use rust_common_game::map::map::NavTileCoord;

use super::DebugConfig;

pub(super) fn debug_render_flow_field(
    debug_config: Res<DebugConfig>,
    mut tilemap_q: Query<&TileStorage, With<TileMapFlowField>>,
    mut tile_q: Query<&mut TileTextureIndex, With<TileFlowField>>,
    flow_field: Res<FlowField>,
) {
    if !debug_config.show_flow_field {
        return;
    }

    let Ok(tile_storage) = tilemap_q.get_single_mut() else {
        return;
    };

    for x in 0..flow_field.size.x {
        for y in 0..flow_field.size.y {
            let map_node_pos = NavTileCoord(UVec2::new(x, y));
            let flow_field_direction = flow_field.map.get(&map_node_pos);

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
                Some(FlowFieldDirection::North) => 5,
                Some(FlowFieldDirection::South) => 6,
                Some(FlowFieldDirection::West) => 4,
                Some(FlowFieldDirection::East) => 7,
                Some(FlowFieldDirection::NorthWest) => 2,
                Some(FlowFieldDirection::SouthWest) => 0,
                Some(FlowFieldDirection::SouthEast) => 3,
                Some(FlowFieldDirection::NorthEast) => 1,
                None => 8,
            }
        }
    }
}
