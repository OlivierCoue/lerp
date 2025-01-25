use std::cmp::Ordering;

use avian2d::prelude::{Collider, Position, RigidBody};
use bevy::{
    math::{UVec2, Vec2},
    prelude::*,
    utils::hashbrown::HashMap,
};

use crate::{
    shared::{NAV_TILE_SIZE, RENDER_TILE_SIZE, RENDER_TO_NAV_TILE_MULTI},
    utils::CommonPlaySceneTag,
    wall::Wall,
};

/// Size of map (counts for tiles)
pub const MAP_SIZE: UVec2 = UVec2::new(40, 40);

// https://d2mods.info/forum/viewtopic.php?t=65163
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RenderMapNodeKind {
    LeftWall,                  // 1
    RightWall,                 // 2
    SouthCornerWall,           // 7
    LeftPartOfNorthCornerWal,  // 4
    RightPartOfNorthCornerWal, // 3
    RightEndWall,              // 6
    LeftEndWall,               // 5
    RightWallWithDoorRight,    // 91
    RightWallWithDoorLeft,     // 92
}
impl RenderMapNodeKind {
    pub fn atlas_index(&self) -> usize {
        match self {
            Self::LeftWall => 18,
            Self::RightWall => 17,
            Self::SouthCornerWall => 0,
            Self::LeftPartOfNorthCornerWal => 28,
            Self::RightPartOfNorthCornerWal => 27,
            Self::RightEndWall => 13,
            Self::LeftEndWall => 14,
            Self::RightWallWithDoorRight => 9,
            Self::RightWallWithDoorLeft => 8,
        }
    }
}

/// Position a node/tile in the map grid
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash)]
pub struct NavMapPos(pub UVec2);

#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash)]
pub struct RenderMapPos(pub UVec2);

pub struct NavMapNode {
    pub walkable: bool,
}

fn cartesian_to_isometric(cart_x: f32, cart_y: f32) -> Vec2 {
    Vec2::new(
        cart_x + cart_y,         // X-axis in isometric space
        (cart_y - cart_x) / 2.0, // Y-axis in isometric space
    )
}

pub struct RenderMapNode {
    pub kind: RenderMapNodeKind,
    pub boundaries: [Vec2; 3],
    pub none_walkable_nav_tiles: Vec<IVec2>,
    pub layer: usize,
}
impl RenderMapNode {
    pub fn new(
        map_size_px: &Vec2,
        render_map_pos: &RenderMapPos,
        kind: RenderMapNodeKind,
        layer: usize,
    ) -> Self {
        let offset_x = (render_map_pos.x as f32) * RENDER_TILE_SIZE - (map_size_px.x / 2.0);
        let offset_y = (render_map_pos.y as f32) * RENDER_TILE_SIZE - (map_size_px.y / 2.0);
        let iso_offset = cartesian_to_isometric(offset_x, offset_y);

        Self {
            kind,
            layer,
            boundaries: match kind {
                RenderMapNodeKind::LeftPartOfNorthCornerWal
                | RenderMapNodeKind::LeftWall
                | RenderMapNodeKind::LeftEndWall => [
                    Vec2::new(iso_offset.x, iso_offset.y - 32.),
                    Vec2::new(iso_offset.x + 80., iso_offset.y + 8.),
                    Vec2::new(iso_offset.x + 160., iso_offset.y + 48.),
                ],
                RenderMapNodeKind::RightEndWall
                | RenderMapNodeKind::RightWall
                | RenderMapNodeKind::RightPartOfNorthCornerWal
                | RenderMapNodeKind::RightWallWithDoorLeft
                | RenderMapNodeKind::RightWallWithDoorRight => [
                    Vec2::new(iso_offset.x, iso_offset.y + 48.),
                    Vec2::new(iso_offset.x + 80., iso_offset.y + 8.),
                    Vec2::new(iso_offset.x + 160., iso_offset.y - 32.),
                ],
                RenderMapNodeKind::SouthCornerWall => [
                    Vec2::new(iso_offset.x, iso_offset.y + 48.),
                    Vec2::new(iso_offset.x + 80., iso_offset.y + 8.),
                    Vec2::new(iso_offset.x + 160., iso_offset.y + 48.),
                ],
            },
            none_walkable_nav_tiles: match kind {
                RenderMapNodeKind::LeftPartOfNorthCornerWal
                | RenderMapNodeKind::LeftWall
                | RenderMapNodeKind::LeftEndWall => {
                    vec![
                        IVec2::new(0, 0),
                        IVec2::new(0, 1),
                        IVec2::new(0, 2),
                        IVec2::new(0, 3),
                        IVec2::new(0, 4),
                        IVec2::new(1, 0),
                        IVec2::new(1, 1),
                        IVec2::new(1, 2),
                        IVec2::new(1, 3),
                        IVec2::new(1, 4),
                    ]
                }
                RenderMapNodeKind::RightPartOfNorthCornerWal
                | RenderMapNodeKind::RightWall
                | RenderMapNodeKind::RightEndWall
                | RenderMapNodeKind::RightWallWithDoorRight => {
                    vec![
                        IVec2::new(0, 3),
                        IVec2::new(1, 3),
                        IVec2::new(2, 3),
                        IVec2::new(3, 3),
                        IVec2::new(4, 3),
                        IVec2::new(0, 4),
                        IVec2::new(1, 4),
                        IVec2::new(2, 4),
                        IVec2::new(3, 4),
                        IVec2::new(4, 4),
                    ]
                }
                RenderMapNodeKind::RightWallWithDoorLeft => vec![
                    IVec2::new(3, 3),
                    IVec2::new(4, 3),
                    IVec2::new(3, 4),
                    IVec2::new(4, 4),
                ],
                RenderMapNodeKind::SouthCornerWall => vec![
                    IVec2::new(0, 3),
                    IVec2::new(0, 4),
                    IVec2::new(1, 3),
                    IVec2::new(1, 4),
                ],
            },
        }
    }

    pub fn compare_y(&self, iso_position: &Vec2) -> (f32, Ordering) {
        let [a_left, a_middle, a_right] = self.boundaries;

        // Determine which segment of the boundary to use based on x
        if iso_position.x < a_left.x {
            // Use the left boundary of A
            (a_left.y, a_left.y.partial_cmp(&iso_position.y).unwrap())
        } else if iso_position.x > a_right.x {
            // Use the right boundary of A
            (a_right.y, a_right.y.partial_cmp(&iso_position.y).unwrap())
        } else if iso_position.x <= a_middle.x {
            // Interpolate between left and middle
            let iso_position_y_at_x = a_left.y
                + (iso_position.x - a_left.x) / (a_middle.x - a_left.x) * (a_middle.y - a_left.y);
            (
                iso_position_y_at_x,
                iso_position_y_at_x.partial_cmp(&iso_position.y).unwrap(),
            )
        } else {
            // Interpolate between middle and right
            let iso_position_y_at_x = a_middle.y
                + (iso_position.x - a_middle.x) / (a_right.x - a_middle.x)
                    * (a_right.y - a_middle.y);
            (
                iso_position_y_at_x,
                iso_position_y_at_x.partial_cmp(&iso_position.y).unwrap(),
            )
        }
    }
}

#[derive(Resource, Default)]
pub struct MapGrid {
    pub nav_map: HashMap<NavMapPos, NavMapNode>,
    pub nav_map_size: UVec2,

    pub render_map: HashMap<RenderMapPos, Vec<RenderMapNode>>,
    pub render_map_size: UVec2,

    pub map_px_size: Vec2,
    pub nav_tile_offset: Vec2,
}
impl MapGrid {
    pub fn reset(&mut self, render_map_size: UVec2) {
        self.render_map.clear();
        self.render_map_size = render_map_size;

        self.nav_map.clear();
        self.nav_map_size = self.render_map_size * RENDER_TO_NAV_TILE_MULTI;

        self.map_px_size = Vec2::new(
            self.render_map_size.x as f32 * RENDER_TILE_SIZE,
            self.render_map_size.y as f32 * RENDER_TILE_SIZE,
        );

        self.nav_tile_offset = Vec2::new(
            (self.map_px_size.x / 2.0) - NAV_TILE_SIZE / 2.0,
            (self.map_px_size.y / 2.0) - NAV_TILE_SIZE / 2.0,
        );
    }

    pub fn add_tile(
        &mut self,
        commands: &mut Commands,
        kind: RenderMapNodeKind,
        render_tile_pos: UVec2,
        layer: usize,
    ) {
        let render_map_node = RenderMapNode::new(
            &self.map_px_size,
            &RenderMapPos(UVec2::new(render_tile_pos.x, render_tile_pos.y)),
            kind,
            layer,
        );

        for none_walkable_nav_tile in &render_map_node.none_walkable_nav_tiles {
            self.nav_map.insert(
                NavMapPos(UVec2::new(
                    ((render_tile_pos.x * RENDER_TO_NAV_TILE_MULTI) as i32
                        + none_walkable_nav_tile.x) as u32,
                    ((render_tile_pos.y * RENDER_TO_NAV_TILE_MULTI) as i32
                        + none_walkable_nav_tile.y) as u32,
                )),
                NavMapNode { walkable: false },
            );
            commands.spawn((
                CommonPlaySceneTag,
                Wall,
                Position::from_xy(
                    (render_tile_pos.x as i32 * RENDER_TO_NAV_TILE_MULTI as i32
                        + none_walkable_nav_tile.x) as f32
                        * NAV_TILE_SIZE
                        - self.nav_tile_offset.x,
                    (render_tile_pos.y as i32 * RENDER_TO_NAV_TILE_MULTI as i32
                        + none_walkable_nav_tile.y) as f32
                        * NAV_TILE_SIZE
                        - self.nav_tile_offset.y,
                ),
                RigidBody::Static,
                Collider::rectangle(NAV_TILE_SIZE, NAV_TILE_SIZE),
            ));
        }

        let key = RenderMapPos(UVec2::new(render_tile_pos.x, render_tile_pos.y));
        if let Some(exist) = self.render_map.get_mut(&key) {
            exist.push(render_map_node);
        } else {
            self.render_map.insert(key, vec![render_map_node]);
        }
    }

    pub fn get_nav_node_xy(&self, x: u32, y: u32) -> Option<&NavMapNode> {
        self.nav_map.get(&NavMapPos(UVec2::new(x, y)))
    }
    pub fn get_nav_node_uvec2(&self, uvec2: UVec2) -> Option<&NavMapNode> {
        self.nav_map.get(&NavMapPos(uvec2))
    }

    pub fn get_render_node_xy(&self, x: u32, y: u32) -> Option<&Vec<RenderMapNode>> {
        self.render_map.get(&RenderMapPos(UVec2::new(x, y)))
    }
    pub fn get_render_node_from_world_position(
        &self,
        position: &Vec2,
    ) -> Option<&Vec<RenderMapNode>> {
        self.render_map.get(&world_position_to_render_map_node_pos(
            &self.render_map_size,
            position,
        ))
    }
}

pub fn world_position_to_nav_map_node_pos(nav_map_size: &UVec2, position: &Vec2) -> NavMapPos {
    let half_width = (nav_map_size.x as f32 * NAV_TILE_SIZE) / 2.;
    let half_height = (nav_map_size.y as f32 * NAV_TILE_SIZE) / 2.;

    // Add half of the map width/height to the position to get its absolute (positive) position
    // Example:
    // Map of 3000/3000 have an half width/height of 1500 and go from -1500 to 1500
    // Given a position of 0/0 position_abs will be 0 + 1500 / 0 + 1500 so 1500/1500
    let position_abs = position + Vec2::new(half_width, half_height);

    // Divide position_abs by the size of a tile to get the tile coordonates
    NavMapPos(UVec2::new(
        (position_abs.x / NAV_TILE_SIZE) as u32,
        (position_abs.y / NAV_TILE_SIZE) as u32,
    ))
}

pub fn world_position_to_render_map_node_pos(
    render_map_size: &UVec2,
    position: &Vec2,
) -> RenderMapPos {
    let half_width = (render_map_size.x as f32 * RENDER_TILE_SIZE) / 2.;
    let half_height = (render_map_size.y as f32 * RENDER_TILE_SIZE) / 2.;

    // Add half of the map width/height to the position to get its absolute (positive) position
    // Example:
    // Map of 3000/3000 have an half width/height of 1500 and go from -1500 to 1500
    // Given a position of 0/0 position_abs will be 0 + 1500 / 0 + 1500 so 1500/1500
    let position_abs = position + Vec2::new(half_width, half_height);

    // Divide position_abs by the size of a tile to get the tile coordonates
    RenderMapPos(UVec2::new(
        (position_abs.x / RENDER_TILE_SIZE) as u32,
        (position_abs.y / RENDER_TILE_SIZE) as u32,
    ))
}

pub fn generate_map(mut commands: Commands, mut map_grid: ResMut<MapGrid>) {
    map_grid.reset(MAP_SIZE);

    // map_grid.render_map.clear();
    // map_grid.render_map_size = MAP_SIZE;

    // map_grid.nav_map.clear();
    // map_grid.nav_map_size = map_grid.render_map_size * RENDER_TO_NAV_TILE_MULTI;

    // let width_px = map_grid.render_map_size.x as f32 * RENDER_TILE_SIZE;
    // let height_px = map_grid.render_map_size.y as f32 * RENDER_TILE_SIZE;
    // let map_size_px = Vec2::new(width_px, height_px);

    // let node_offest_x = (width_px / 2.0) - NAV_TILE_SIZE / 2.0;
    // let node_offest_y = (height_px / 2.0) - NAV_TILE_SIZE / 2.0;

    for x_render in 0..map_grid.render_map_size.x {
        for y_render in 0..map_grid.render_map_size.y {
            // let cart_coord = Vec2::new(
            //     x_render as f32 * PIXEL_METER - (width_px / 2.0) + node_offest,
            //     y_render as f32 * PIXEL_METER - (height_px / 2.0) + node_offest,
            // );

            // Insert an mark all nav tiles as walkable by default
            for x_nav in x_render * RENDER_TO_NAV_TILE_MULTI
                ..x_render * RENDER_TO_NAV_TILE_MULTI + RENDER_TO_NAV_TILE_MULTI
            {
                for y_nav in y_render * RENDER_TO_NAV_TILE_MULTI
                    ..y_render * RENDER_TO_NAV_TILE_MULTI + RENDER_TO_NAV_TILE_MULTI
                {
                    map_grid.nav_map.insert(
                        NavMapPos(UVec2::new(x_nav, y_nav)),
                        NavMapNode { walkable: true },
                    );
                }
            }

            // Top left
            if x_render == 0 && y_render == (map_grid.render_map_size.y - 1) {
                map_grid.add_tile(
                    &mut commands,
                    RenderMapNodeKind::LeftPartOfNorthCornerWal,
                    UVec2::new(x_render, y_render),
                    0,
                );
                map_grid.add_tile(
                    &mut commands,
                    RenderMapNodeKind::RightPartOfNorthCornerWal,
                    UVec2::new(x_render, y_render),
                    0,
                );
            }
            // Top Right
            else if x_render == (map_grid.render_map_size.x - 1)
                && y_render == (map_grid.render_map_size.y - 1)
            {
                map_grid.add_tile(
                    &mut commands,
                    RenderMapNodeKind::LeftEndWall,
                    UVec2::new(x_render, y_render),
                    0,
                );
            }
            // Bottom Right
            else if x_render == (map_grid.render_map_size.x - 1) && y_render == 0 {
                map_grid.add_tile(
                    &mut commands,
                    RenderMapNodeKind::SouthCornerWall,
                    UVec2::new(x_render, y_render),
                    0,
                );
            }
            // Bottom Left
            else if x_render == 0 && y_render == 0 {
                map_grid.add_tile(
                    &mut commands,
                    RenderMapNodeKind::RightEndWall,
                    UVec2::new(x_render, y_render),
                    0,
                );
            }
            // Top/Bottom
            else if y_render == (map_grid.render_map_size.y - 1) || y_render == 0 {
                map_grid.add_tile(
                    &mut commands,
                    RenderMapNodeKind::RightWall,
                    UVec2::new(x_render, y_render),
                    0,
                );
            }
            // Left/Right
            else if x_render == (map_grid.render_map_size.x - 1) || x_render == 0 {
                map_grid.add_tile(
                    &mut commands,
                    RenderMapNodeKind::LeftWall,
                    UVec2::new(x_render, y_render),
                    0,
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
        //
        [20, 10],
        [20, 30],
        //
        [28, 10],
        [28, 20],
        [28, 30],
    ] {
        let t = [
            [[4, 3], [2, 0], [2, 0], [2, 0], [5, 0]],   //
            [[1, 0], [0, 0], [0, 0], [0, 0], [1, 0]],   //
            [[1, 0], [0, 0], [0, 0], [0, 0], [1, 0]],   //
            [[1, 0], [0, 0], [0, 0], [0, 0], [1, 0]],   //
            [[1, 0], [0, 0], [0, 0], [0, 0], [1, 0]],   //
            [[6, 0], [2, 0], [91, 0], [92, 0], [7, 0]], //
        ];

        for (y, row) in t.iter().enumerate() {
            for (x, tiles) in row.iter().enumerate() {
                for v in tiles {
                    if *v == 0 {
                        continue;
                    }
                    let kind = match v {
                        1 => RenderMapNodeKind::LeftWall,
                        2 => RenderMapNodeKind::RightWall,
                        3 => RenderMapNodeKind::RightPartOfNorthCornerWal,
                        4 => RenderMapNodeKind::LeftPartOfNorthCornerWal,
                        5 => RenderMapNodeKind::LeftEndWall,
                        6 => RenderMapNodeKind::RightEndWall,
                        7 => RenderMapNodeKind::SouthCornerWall,
                        91 => RenderMapNodeKind::RightWallWithDoorRight,
                        92 => RenderMapNodeKind::RightWallWithDoorLeft,
                        _ => panic!("Invalid tile id"),
                    };

                    map_grid.add_tile(
                        &mut commands,
                        kind,
                        UVec2::new(offset[0] + x as u32, offset[1] - y as u32),
                        1,
                    );
                }
            }
        }
    }

    // // // TOP
    // commands.spawn((
    //     CommonPlaySceneTag,
    //     Wall,
    //     Position::from_xy(0., (height_px / 2.0) - node_center_offest),
    //     RigidBody::Static,
    //     Collider::rectangle(width_px, PIXEL_METER),
    // ));
    // // // BOTOM
    // commands.spawn((
    //     CommonPlaySceneTag,
    //     Wall,
    //     Position::from_xy(0., -(height_px / 2.0) + node_center_offest),
    //     RigidBody::Static,
    //     Collider::rectangle(width_px, PIXEL_METER),
    // ));
    // // // LEFT
    // commands.spawn((
    //     CommonPlaySceneTag,
    //     Wall,
    //     Position::from_xy(-(width_px / 2.0) + node_center_offest, 0.),
    //     RigidBody::Static,
    //     Collider::rectangle(PIXEL_METER, height_px),
    // ));
    // // // RIGHT
    // commands.spawn((
    //     CommonPlaySceneTag,
    //     Wall,
    //     Position::from_xy(width_px / 2.0 - node_center_offest, 0.),
    //     RigidBody::Static,
    //     Collider::rectangle(PIXEL_METER, height_px),
    // ));
}
