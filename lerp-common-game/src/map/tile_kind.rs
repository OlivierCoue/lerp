use bevy::prelude::*;

// https://d2mods.info/forum/viewtopic.php?t=65163
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RenderTileWallKind {
    LeftWall,                  // 1
    RightWall,                 // 2
    SouthCornerWall,           // 7
    LeftPartOfNorthCornerWal,  // 4
    RightPartOfNorthCornerWal, // 3
    RightEndWall,              // 6
    LeftEndWall,               // 5
    LefttWallWithDoorBottom,   // 81
    LefttWallWithDoorTop,      // 82
    RightWallWithDoorRight,    // 91
    RightWallWithDoorLeft,     // 92
}
impl RenderTileWallKind {
    pub fn atlas_index(&self) -> usize {
        match self {
            Self::LeftWall => 18,
            Self::RightWall => 17,
            Self::SouthCornerWall => 0,
            Self::LeftPartOfNorthCornerWal => 28,
            Self::RightPartOfNorthCornerWal => 27,
            Self::RightEndWall => 13,
            Self::LeftEndWall => 14,
            Self::LefttWallWithDoorBottom => 10,
            Self::LefttWallWithDoorTop => 11,
            Self::RightWallWithDoorRight => 9,
            Self::RightWallWithDoorLeft => 8,
        }
    }

    pub fn y_sort_boundaries(&self) -> [[i32; 2]; 3] {
        match self {
            RenderTileWallKind::LeftPartOfNorthCornerWal
            | RenderTileWallKind::LeftWall
            | RenderTileWallKind::LeftEndWall
            | RenderTileWallKind::LefttWallWithDoorBottom
            | RenderTileWallKind::LefttWallWithDoorTop => [[0, -32], [80, 8], [160, 48]],
            RenderTileWallKind::RightEndWall
            | RenderTileWallKind::RightWall
            | RenderTileWallKind::RightPartOfNorthCornerWal
            | RenderTileWallKind::RightWallWithDoorLeft
            | RenderTileWallKind::RightWallWithDoorRight => [[0, 48], [80, 8], [160, -32]],
            RenderTileWallKind::SouthCornerWall => [[0, 48], [80, 8], [160, 48]],
        }
    }

    pub fn y_sort_boundaries_with_offset(&self, iso_offset: Vec2) -> [Vec2; 3] {
        let y_sort_boundaries = self.y_sort_boundaries();
        [
            Vec2::new(
                iso_offset.x + y_sort_boundaries[0][0] as f32,
                iso_offset.y + y_sort_boundaries[0][1] as f32,
            ),
            Vec2::new(
                iso_offset.x + y_sort_boundaries[1][0] as f32,
                iso_offset.y + y_sort_boundaries[1][1] as f32,
            ),
            Vec2::new(
                iso_offset.x + y_sort_boundaries[2][0] as f32,
                iso_offset.y + y_sort_boundaries[2][1] as f32,
            ),
        ]
    }

    pub fn none_walkable_nav_tiles(&self) -> Vec<IVec2> {
        match self {
            RenderTileWallKind::LeftPartOfNorthCornerWal
            | RenderTileWallKind::LeftWall
            | RenderTileWallKind::LeftEndWall
            | RenderTileWallKind::LefttWallWithDoorBottom => {
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
            RenderTileWallKind::RightPartOfNorthCornerWal
            | RenderTileWallKind::RightWall
            | RenderTileWallKind::RightEndWall
            | RenderTileWallKind::RightWallWithDoorRight => {
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
            RenderTileWallKind::RightWallWithDoorLeft => vec![
                IVec2::new(3, 3),
                IVec2::new(4, 3),
                IVec2::new(3, 4),
                IVec2::new(4, 4),
            ],
            RenderTileWallKind::LefttWallWithDoorTop => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),
                IVec2::new(1, 0),
                IVec2::new(1, 1),
            ],
            RenderTileWallKind::SouthCornerWall => vec![
                IVec2::new(0, 3),
                IVec2::new(0, 4),
                IVec2::new(1, 3),
                IVec2::new(1, 4),
            ],
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RenderTileFloorKind {
    Standard,
}
