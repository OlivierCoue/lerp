use godot::builtin::Vector2;

pub fn iso_to_cart(vect2: &Vector2) -> Vector2 {
    Vector2 {
        x: vect2.x * 0.5 + vect2.y * 1.0,
        y: vect2.x * -0.5 + vect2.y * 1.0,
    }
}

pub fn cart_to_iso(vect2: &Vector2) -> Vector2 {
    Vector2 {
        x: vect2.x * 1.0 + vect2.y * -1.0,
        y: vect2.x * 0.5 + vect2.y * 0.5,
    }
}
