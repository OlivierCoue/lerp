use rust_common::proto::TileType;

#[derive(Clone)]
pub struct FloorPattern {
    // odds: f32,
    pub rng_range_multiplicator_rectangle_size: (f32, f32),
    pub rng_range_number_of_direction_changes: (i32, i32),
    pub rng_range_direction_repeat: (i32, i32),
    pub allowed_directions: Vec<(i32, i32)>,
    pub generation_area_size: (i32, i32),
}
#[derive(Clone)]
pub struct MobPack {
    pub tile_coords: (usize, usize),
}

#[derive(Clone)]
pub struct GenTile {
    pub tile_type: TileType,
    pub scanned: bool,
    pub walkable: bool,
    pub spawnable: bool,
    pub mob_pack: Option<MobPack>,
    pub is_start: bool,
    pub is_boss: bool,
    pub layer: i32,
}

#[derive(Clone)]
pub struct Map {
    pub name: String,
    pub oob_type: TileType,
    pub biomes: Vec<FloorPattern>,
    pub density: f64,
}
pub fn define_floor_patterns() -> Vec<Map> {
    //------------------------------------------------------//
    //                Define all Floor Patterns             //
    //------------------------------------------------------//
    let large_all_dir = FloorPattern {
        // odds: 1.0,
        rng_range_multiplicator_rectangle_size: (0.04, 0.08),
        rng_range_number_of_direction_changes: (4, 5),
        rng_range_direction_repeat: (1, 3),
        allowed_directions: vec![
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, -1),
            (1, 1),
            (-1, 1),
            (1, 1),
        ],
        generation_area_size: (345, 345),
    };
    let small_cross_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.015, 0.025),
        rng_range_number_of_direction_changes: (20, 30),
        rng_range_direction_repeat: (5, 10),
        allowed_directions: vec![(0, -1), (0, 1), (-1, 0), (1, 0)],
        generation_area_size: (345, 345),
    };
    let small_all_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.015, 0.025),
        rng_range_number_of_direction_changes: (15, 25),
        rng_range_direction_repeat: (10, 15),
        allowed_directions: vec![
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, -1),
            (1, 1),
            (-1, 1),
            (-1, -1),
        ],
        generation_area_size: (345, 345),
    };
    let many_tiny_all_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.01, 0.020),
        rng_range_number_of_direction_changes: (30, 40),
        rng_range_direction_repeat: (10, 15),
        allowed_directions: vec![(1, -1), (1, 1), (-1, 1), (-1, -1)],
        generation_area_size: (345, 345),
    };
    let long_path_bottom_right_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.01, 0.020),
        rng_range_number_of_direction_changes: (20, 30),
        rng_range_direction_repeat: (10, 15),
        allowed_directions: vec![(1, -1), (1, 1), (-1, 1)],
        generation_area_size: (345, 345),
    };
    let short_path_bottom_right_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.01, 0.020),
        rng_range_number_of_direction_changes: (10, 15),
        rng_range_direction_repeat: (5, 8),
        allowed_directions: vec![(1, -1), (1, 1), (-1, 1)],

        generation_area_size: (345, 345),
    };
    //------------------------------------------------------//
    //                Define Maps Content                   //
    //------------------------------------------------------//
    // Godd Walakable tiles value [5000-10000]

    let maps: Vec<Map> = vec![
        Map {
            name: String::from("Island"),
            oob_type: TileType::Water,
            biomes: vec![many_tiny_all_dir.clone(), small_all_dir.clone()],
            density: 0.01,
        },
        Map {
            name: String::from("Ledge"),
            oob_type: TileType::Rock,
            biomes: vec![
                long_path_bottom_right_dir.clone(),
                long_path_bottom_right_dir.clone(),
            ],
            density: 0.01,
        },
        Map {
            name: String::from("Desert"),
            oob_type: TileType::Rock,
            biomes: vec![long_path_bottom_right_dir.clone(), large_all_dir.clone()],
            density: 0.01,
        },
        Map {
            name: String::from("Forest"),
            oob_type: TileType::Forest,
            biomes: vec![
                short_path_bottom_right_dir.clone(),
                small_cross_dir.clone(),
                small_cross_dir.clone(),
            ],
            density: 0.01,
        },
        Map {
            name: String::from("Quarry"),
            oob_type: TileType::Rock,
            biomes: vec![
                short_path_bottom_right_dir.clone(),
                many_tiny_all_dir.clone(),
                short_path_bottom_right_dir.clone(),
            ],
            density: 0.01,
        },
    ];

    maps
}
