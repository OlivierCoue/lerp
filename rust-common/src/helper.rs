use godot::builtin::Vector2;

use crate::proto::common::Point;

pub fn vector2_to_point(vector2: &Vector2) -> Point {
    Point {
        x: vector2.x,
        y: vector2.y,
        ..Default::default()
    }
}
pub fn point_to_vector2(point: &Point) -> Vector2 {
    Vector2 {
        x: point.x,
        y: point.y,
    }
}
