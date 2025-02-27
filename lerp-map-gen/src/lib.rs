use bevy::{prelude::*, utils::HashMap};
use rand::Rng;
/// Number of pixels per one meter
pub const PIXEL_METER: f32 = 32.;

pub const NAV_TILE_SIZE: f32 = PIXEL_METER / 2.;
pub const RENDER_TO_NAV_TILE_MULTI: u32 = 5;
pub const RENDER_TILE_SIZE: f32 = NAV_TILE_SIZE * RENDER_TO_NAV_TILE_MULTI as f32;

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default());
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct TilePos(pub UVec2);
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    top_left: UVec2,
    bottom_right: UVec2,
}

impl Rectangle {
    fn new(top_left: UVec2, bottom_right: UVec2) -> Self {
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    fn width(&self) -> u32 {
        self.bottom_right.x - self.top_left.x
    }

    fn height(&self) -> u32 {
        self.bottom_right.y - self.top_left.y
    }
}

#[derive(Resource, Default, Clone)]
pub struct Map {
    render_grid: HashMap<TilePos, TileKind>,
    pub render_grid_size: UVec2,
}
impl Map {
    pub fn reset(&mut self) {
        self.render_grid.clear();
        self.render_grid_size = UVec2::ZERO;
    }

    pub fn set_tile(&mut self, pos: TilePos, kind: TileKind) {
        self.render_grid.insert(pos, kind);
    }

    pub fn get_tile(&self, pos: &TilePos) -> Option<&TileKind> {
        self.render_grid.get(pos)
    }
    pub fn generate_bsp_floor(&mut self, iterations: u32, min_size: UVec2) {
        let mut regions = vec![(UVec2::ZERO, self.render_grid_size)];
        let mut rng = rand::rng();
        for _ in 0..iterations {
            let mut new_regions = Vec::new();
            for (start, size) in regions {
                if size.x < min_size.x || size.y < min_size.y {
                    new_regions.push((start, size));
                    continue;
                }

                let axis = rng.random_range(0..2);
                let first_region;
                let second_region;

                if axis == 0 {
                    let max_threshold = size.x - min_size.x;
                    if max_threshold <= min_size.x {
                        new_regions.push((start, size));
                        continue;
                    }

                    let threshold = rng.random_range(min_size.x..=max_threshold);
                    first_region = (start, UVec2::new(threshold, size.y));
                    second_region = (
                        UVec2::new(start.x + threshold, start.y),
                        UVec2::new(size.x - threshold, size.y),
                    );
                } else {
                    let max_threshold = size.y - min_size.y;
                    if max_threshold <= min_size.y {
                        new_regions.push((start, size));
                        continue;
                    }
                    let threshold = rng.random_range(min_size.y..=max_threshold);
                    first_region = (start, UVec2::new(size.x, threshold));
                    second_region = (
                        UVec2::new(start.x, start.y + threshold),
                        UVec2::new(size.x, size.y - threshold),
                    );
                }
                if rng.random_bool(0.5) {
                    new_regions.push(first_region);
                    new_regions.push(second_region);
                } else {
                    new_regions.push(second_region);
                    new_regions.push(first_region);
                }
            }
            regions = new_regions;
        }
        for (start, size) in regions.last() {
            for x in start.x..(start.x + size.x) {
                for y in start.y..(start.y + size.y) {
                    self.set_tile(TilePos(UVec2::new(x, y)), TileKind::Floor);
                }
            }
        }
    }
    pub fn generate_map(&mut self, size: UVec2) {
        self.reset();
        self.render_grid_size = size;

        for x in 0..self.render_grid_size.x {
            for y in 0..self.render_grid_size.y {
                self.set_tile(TilePos(UVec2::new(x, y)), TileKind::Floor);
            }
        }
    }
    pub fn generate_weighted_random_split(&mut self, size: UVec2) {
        let original_rect = Rectangle::new(UVec2::new(0, 0), size);
        let mut rectangles = HashMap::new();
        let min_width = size.x / 15;
        let max_width = size.x / 3;
        let min_height = size.y / 15;
        let max_height = size.y / 3;
        let weight_horizontal = 0.5; // hance of horizontal split
        let weight_vertical = 0.5; // chance of vertical split
        self.generate_map(size);
        weighted_random_split(
            original_rect,
            min_width,
            max_width,
            min_height,
            max_height,
            weight_horizontal,
            weight_vertical,
            &mut rectangles,
        );

        let mut wall_tiles = HashMap::new(); // HashMap to store wall tiles

        for (_, rect) in rectangles.iter() {
            for point in get_rectangle_lines(*rect) {
                wall_tiles.insert(point, ()); // Use empty tuple as a placeholder value
            }
        }

        for point in wall_tiles.keys() {
            self.set_tile(TilePos(*point), TileKind::Wall);
        }

        for x in 0..self.render_grid_size.x {
            self.set_tile(
                TilePos(UVec2::new(x, self.render_grid_size.y - 1)),
                TileKind::Wall,
            );
        }
        for y in 0..self.render_grid_size.y {
            self.set_tile(
                TilePos(UVec2::new(self.render_grid_size.x - 1, y)),
                TileKind::Wall,
            );
        }
    }
    pub fn generate_slice_and_dice(&mut self, size: UVec2) {
        let original_rect = Rectangle::new(UVec2::new(0, 0), size);
        let mut rectangles = HashMap::new();
        let min_width = size.x / 8;
        let max_width = size.x / 3;
        let min_height = size.y / 8;
        let max_height = size.y / 3;
        self.generate_map(size);
        slice_and_dice(
            original_rect,
            min_width,
            max_width,
            min_height,
            max_height,
            &mut rectangles,
        );

        let mut wall_tiles = HashMap::new(); // HashMap to store wall tiles

        for (_, rect) in rectangles.iter() {
            println!("{},{}", rect.top_left.x, rect.top_left.y);
            for point in get_rectangle_lines(*rect) {
                wall_tiles.insert(point, ()); // Use empty tuple as a placeholder value
            }
        }

        for point in wall_tiles.keys() {
            self.set_tile(TilePos(*point), TileKind::Wall);
        }

        for x in 0..self.render_grid_size.x {
            self.set_tile(
                TilePos(UVec2::new(x, self.render_grid_size.y - 1)),
                TileKind::Wall,
            );
        }
        for y in 0..self.render_grid_size.y {
            self.set_tile(
                TilePos(UVec2::new(self.render_grid_size.x - 1, y)),
                TileKind::Wall,
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TileKind {
    Wall,
    Water,
    Floor,
}

fn weighted_random_split(
    rect: Rectangle,
    min_width: u32,
    max_width: u32,
    min_height: u32,
    max_height: u32,
    weight_horizontal: f64,
    weight_vertical: f64,
    rectangles: &mut HashMap<UVec2, Rectangle>,
) {
    let mut rng = rand::rng();
    if rect.width() < min_width * 2 || rect.height() < min_height * 2 {
        rectangles.insert(rect.top_left, rect);
        return;
    }

    // Adjust split probability based on aspect ratio
    let aspect_ratio = rect.width() as f64 / rect.height() as f64;
    let split_horizontal_prob = if aspect_ratio > 1.0 {
        // Wider than tall, favor vertical split
        weight_vertical / (weight_horizontal + weight_vertical)
    } else {
        // Taller than wide, favor horizontal split
        weight_horizontal / (weight_horizontal + weight_vertical)
    };

    let split_horizontal = rng.random_bool(split_horizontal_prob);

    if split_horizontal && rect.height() >= min_height * 2 {
        let min_y = rect.top_left.y + min_height;

        let usable_max_height = std::cmp::min(max_height, rect.height());
        let usable_min_y =
            rect.top_left.y + std::cmp::max(min_height, rect.height() - usable_max_height);

        if usable_min_y < rect.bottom_right.y - min_height {
            let split_y = rng.random_range(usable_min_y..rect.bottom_right.y - min_height);

            // Prevent rectangles with the same height as the original
            if split_y != rect.top_left.y && split_y != rect.bottom_right.y {
                let rect1 = Rectangle::new(rect.top_left, UVec2::new(rect.bottom_right.x, split_y));
                let rect2 = Rectangle::new(UVec2::new(rect.top_left.x, split_y), rect.bottom_right);

                weighted_random_split(
                    rect1,
                    min_width,
                    max_width,
                    min_height,
                    max_height,
                    weight_horizontal,
                    weight_vertical,
                    rectangles,
                );
                weighted_random_split(
                    rect2,
                    min_width,
                    max_width,
                    min_height,
                    max_height,
                    weight_horizontal,
                    weight_vertical,
                    rectangles,
                );
            } else {
                rectangles.insert(rect.top_left, rect);
            }
        } else {
            rectangles.insert(rect.top_left, rect);
        }
    } else if rect.width() >= min_width * 2 {
        let min_x = rect.top_left.x + min_width;

        let usable_max_width = std::cmp::min(max_width, rect.width());
        let usable_min_x =
            rect.top_left.x + std::cmp::max(min_width, rect.width() - usable_max_width);

        if usable_min_x < rect.bottom_right.x - min_width {
            let split_x = rng.random_range(usable_min_x..rect.bottom_right.x - min_width);

            // Prevent rectangles with the same width as the original
            if split_x != rect.top_left.x && split_x != rect.bottom_right.x {
                let rect1 = Rectangle::new(rect.top_left, UVec2::new(split_x, rect.bottom_right.y));
                let rect2 = Rectangle::new(UVec2::new(split_x, rect.top_left.y), rect.bottom_right);

                weighted_random_split(
                    rect1,
                    min_width,
                    max_width,
                    min_height,
                    max_height,
                    weight_horizontal,
                    weight_vertical,
                    rectangles,
                );
                weighted_random_split(
                    rect2,
                    min_width,
                    max_width,
                    min_height,
                    max_height,
                    weight_horizontal,
                    weight_vertical,
                    rectangles,
                );
            } else {
                rectangles.insert(rect.top_left, rect);
            }
        } else {
            rectangles.insert(rect.top_left, rect);
        }
    } else {
        rectangles.insert(rect.top_left, rect);
    }
}

fn slice_and_dice(
    rect: Rectangle,
    min_width: u32,
    max_width: u32,
    min_height: u32,
    max_height: u32,
    rectangles: &mut HashMap<UVec2, Rectangle>,
) {
    let mut rng = rand::rng();
    let mut remaining = vec![rect];

    while let Some(current_rect) = remaining.pop() {
        if current_rect.width() >= min_width * 2 {
            // Vertical slice
            let split_start = current_rect.top_left.x + min_width;
            let split_end = current_rect.bottom_right.x - min_width;

            if split_start < split_end {
                // Check for valid range
                let split_x = rng.gen_range(split_start..split_end);
                let rect1 = Rectangle::new(
                    current_rect.top_left,
                    UVec2::new(split_x, current_rect.bottom_right.y),
                );
                let rect2 = Rectangle::new(
                    UVec2::new(split_x, current_rect.top_left.y),
                    current_rect.bottom_right,
                );
                remaining.push(rect1);
                remaining.push(rect2);
            } else {
                // Cannot slice vertically, add to results
                rectangles.insert(current_rect.top_left, current_rect);
            }
        } else if current_rect.height() >= min_height * 2 {
            // Horizontal slice
            let split_start = current_rect.top_left.y + min_height;
            let split_end = current_rect.bottom_right.y - min_height;

            if split_start < split_end {
                // Check for valid range
                let split_y = rng.gen_range(split_start..split_end);
                let rect1 = Rectangle::new(
                    current_rect.top_left,
                    UVec2::new(current_rect.bottom_right.x, split_y),
                );
                let rect2 = Rectangle::new(
                    UVec2::new(current_rect.top_left.x, split_y),
                    current_rect.bottom_right,
                );
                remaining.push(rect1);
                remaining.push(rect2);
            } else {
                // Cannot slice horizontally, add to results
                rectangles.insert(current_rect.top_left, current_rect);
            }
        } else {
            // Cannot slice further, add to results
            rectangles.insert(current_rect.top_left, current_rect);
        }
    }
}

fn get_rectangle_lines(rect: Rectangle) -> Vec<UVec2> {
    let mut points = Vec::new();

    // Top line
    for x in rect.top_left.x..=rect.bottom_right.x {
        points.push(UVec2::new(x, rect.top_left.y));
    }

    // Bottom line
    for x in rect.top_left.x..=rect.bottom_right.x {
        points.push(UVec2::new(x, rect.bottom_right.y));
    }

    // Left line (excluding top and bottom points)
    for y in rect.top_left.y + 1..rect.bottom_right.y {
        points.push(UVec2::new(rect.top_left.x, y));
    }

    // Right line (excluding top and bottom points)
    for y in rect.top_left.y + 1..rect.bottom_right.y {
        points.push(UVec2::new(rect.bottom_right.x, y));
    }

    points
}
