use crate::utils::Coord;

pub fn get_point_with_speed(from: &Coord, to: &Coord, speed: f64) -> Coord {
    let vector = points_to_vector(from, to);
    let v_length = get_vector_length(&vector);

    if v_length == 0.0 {
        return *from;
    }

    let factor = speed / v_length;

    Coord {
        x: from.x + factor * vector.x,
        y: from.y + factor * vector.y,
    }
}

pub fn points_to_vector(p1: &Coord, p2: &Coord) -> Coord {
    let x_dist = p2.x - p1.x;
    let y_dist = p2.y - p1.y;
    Coord {
        x: x_dist,
        y: y_dist,
    }
}

pub fn get_vector_length(coord: &Coord) -> f64 {
    (coord.x * coord.x + coord.y * coord.y).sqrt()
}
