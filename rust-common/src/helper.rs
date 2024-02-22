use std::time::SystemTime;

use crate::{math::Vec2, proto::Point};

pub fn vec2_to_point(vector2: &Vec2) -> Point {
    Point {
        x: vector2.x,
        y: vector2.y,
    }
}

pub fn get_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn get_timestamp_nanos() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
