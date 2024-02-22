use crate::math::Vec2;

pub struct ColliderShape {
    pub rect: Option<Vec2>,
    pub poly: Option<Vec<Vec2>>,
}
impl ColliderShape {
    pub fn new_rect(rect: Vec2) -> Self {
        Self {
            rect: Some(rect),
            poly: None,
        }
    }

    pub fn new_poly(poly: Vec<Vec2>) -> Self {
        Self {
            rect: None,
            poly: Some(poly),
        }
    }
}

// Taken from: https://www.jeffreythompson.org/collision-detection/rect-rect.php
pub fn collide_rect_to_rect(r1_size: &Vec2, r1_pos: &Vec2, r2_size: &Vec2, r2_pos: &Vec2) -> bool {
    let r1x = r1_pos.x - r1_size.x / 2.0;
    let r1y = r1_pos.y - r1_size.y / 2.0;
    let r1w = r1_size.x;
    let r1h = r1_size.y;
    let r2x = r2_pos.x - r2_size.x / 2.0;
    let r2y = r2_pos.y - r2_size.y / 2.0;
    let r2w = r2_size.x;
    let r2h = r2_size.y;

    r1x + r1w >= r2x && r1x <= r2x + r2w && r1y + r1h >= r2y && r1y <= r2y + r2h
}

// Taken from: https://www.jeffreythompson.org/collision-detection/poly-point.php
pub fn collide_point_to_poly(point: &Vec2, poly: &[Vec2], reversed: bool) -> bool {
    let mut collision = false;

    let px = point.x;
    let py = point.y;

    // go through each of the vertices, plus
    // the next vertex in the list
    let mut next;
    for current in 0..poly.len() {
        // get next vertex in list
        // if we've hit the end, wrap around to 0
        next = current + 1;
        if next == poly.len() {
            next = 0;
        }

        // get the PVectors at our current position
        // this makes our if statement a little cleaner
        let vc = poly[current]; // c for "current"
        let vn = poly[next]; // n for "next"

        // compare position, flip 'collision' variable
        // back and forth

        if ((vc.y >= py && vn.y < py) || (vc.y < py && vn.y >= py))
            && (px < (vn.x - vc.x) * (py - vc.y) / (vn.y - vc.y) + vc.x)
        {
            collision = !collision;
        }
    }

    if reversed {
        return !collision;
    }

    collision
}
