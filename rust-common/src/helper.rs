use std::time::SystemTime;

use godot::builtin::Vector2;

use crate::proto::Point;

pub fn vector2_to_point(vector2: &Vector2) -> Point {
    Point {
        x: vector2.x,
        y: vector2.y,
    }
}
pub fn point_to_vector2(point: &Point) -> Vector2 {
    Vector2 {
        x: point.x,
        y: point.y,
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
