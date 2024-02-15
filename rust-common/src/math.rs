use godot::builtin::Vector2;

pub fn get_point_from_points_and_distance(from: Vector2, to: Vector2, distance: f32) -> Vector2 {
    let angle = from.angle_to_point(to);
    Vector2::new(
        from.x + distance * f32::cos(angle),
        from.y + distance * f32::sin(angle),
    )
}
