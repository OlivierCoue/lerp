use crate::math::Vec2;

pub struct Circle {
    pub rayon: f32,
}

pub struct ColliderShape {
    pub inverse: bool,
    pub rect: Option<Vec2>,
    pub poly: Option<Vec<Vec2>>,
    pub circle: Option<Circle>,
}
impl ColliderShape {
    pub fn new_rect(rect: Vec2, inverse: bool) -> Self {
        Self {
            inverse,
            rect: Some(rect),
            poly: None,
            circle: None,
        }
    }

    pub fn new_poly(poly: Vec<Vec2>, inverse: bool) -> Self {
        Self {
            inverse,
            rect: None,
            poly: Some(poly),
            circle: None,
        }
    }

    pub fn new_circle(rayon: f32, inverse: bool) -> Self {
        Self {
            inverse,
            rect: None,
            poly: None,
            circle: Some(Circle { rayon }),
        }
    }

    pub fn collide(&self, self_pos: &Vec2, to_shape: &ColliderShape, to_pos: &Vec2) -> bool {
        if let Some(rect) = &self.rect {
            if let Some(to_rect) = &to_shape.rect {
                return collide_rect_to_rect(rect, self_pos, to_rect, to_pos);
            } else if let Some(to_poly) = &to_shape.poly {
                return collid_poly_to_rect(
                    to_poly,
                    to_shape.inverse,
                    self_pos.x,
                    self_pos.y,
                    rect.x,
                    rect.y,
                );
            } else if let Some(to_circle) = &to_shape.circle {
                return collide_circle_to_rect(to_pos.x, to_pos.y, to_circle.rayon, self_pos, rect);
            }
        } else if let Some(poly) = &self.poly {
            if let Some(to_rect) = to_shape.rect {
                return collid_poly_to_rect(
                    poly,
                    self.inverse,
                    to_pos.x,
                    to_pos.y,
                    to_rect.x,
                    to_rect.y,
                );
            } else if let Some(_to_poly) = &to_shape.poly {
                println!("WARNING: collision POLYGON/POLYGON not implemented.");
                return false;
            } else if let Some(to_circle) = &to_shape.circle {
                return collide_poly_to_circle(
                    poly,
                    self.inverse,
                    to_pos.x,
                    to_pos.y,
                    to_circle.rayon,
                );
            }
        } else if let Some(circle) = &self.circle {
            if let Some(to_rect) = &to_shape.rect {
                return collide_circle_to_rect(
                    self_pos.x,
                    self_pos.y,
                    circle.rayon,
                    to_pos,
                    to_rect,
                );
            } else if let Some(to_poly) = &to_shape.poly {
                return collide_poly_to_circle(
                    to_poly,
                    to_shape.inverse,
                    self_pos.x,
                    self_pos.y,
                    circle.rayon,
                );
            } else if let Some(to_circle) = &to_shape.circle {
                return collide_circle_to_circle(
                    self_pos.x,
                    self_pos.y,
                    circle.rayon,
                    to_pos.x,
                    to_pos.y,
                    to_circle.rayon,
                );
            }
        }

        panic!("Unknown shape")
    }
}

// Taken from: https://www.jeffreythompson.org/collision-detection/rect-rect.php
// RECTANGLE/RECTANGLE
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

// CIRCLE/RECTANGLE
pub fn collide_circle_to_rect(cx: f32, cy: f32, radius: f32, r_pos: &Vec2, r_size: &Vec2) -> bool {
    // temporary variables to set edges for testing
    let mut test_x = cx;
    let mut test_y = cy;

    // which edge is closest?
    // test left edge
    if cx < r_pos.x {
        test_x = r_pos.x;
    }
    // right edge
    else if cx > r_pos.x + r_size.x {
        test_x = r_pos.x + r_size.x;
    }
    // top edge
    if cy < r_pos.y {
        test_y = r_pos.y;
    }
    // bottom edge
    else if cy > r_pos.y + r_size.y {
        test_y = r_pos.y + r_size.y;
    }

    // get distance from closest edges
    let dist_x = cx - test_x;
    let dist_y = cy - test_y;
    let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();

    // if the distance is less than the radius, collision!
    if distance <= radius {
        return true;
    }

    false
}

// Taken from: https://www.jeffreythompson.org/collision-detection/poly-point.php
// POLYGON/POINT
pub fn collide_poly_to_point(point: &Vec2, poly: &[Vec2], reversed: bool) -> bool {
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

// POLYGON/CIRCLE
pub fn collide_poly_to_circle(vertices: &[Vec2], reversed: bool, cx: f32, cy: f32, r: f32) -> bool {
    let mut next;

    for current in 0..vertices.len() {
        next = current + 1;
        if next == vertices.len() {
            next = 0;
        }

        let vc = vertices[current];
        let vn = vertices[next];

        if collide_line_to_circle(vc.x, vc.y, vn.x, vn.y, cx, cy, r) {
            return true;
        }
    }

    let center_inside = collide_poly_to_point(&Vec2::new(cx, cy), vertices, reversed);
    if center_inside {
        return true;
    }

    false
}

// LINE/CIRCLE
pub fn collide_line_to_circle(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    cx: f32,
    cy: f32,
    r: f32,
) -> bool {
    if collide_point_to_circle(x1, y1, cx, cy, r) || collide_point_to_circle(x2, y2, cx, cy, r) {
        return true;
    }

    let dist_x = x1 - x2;
    let dist_y = y1 - y2;
    let len = (dist_x * dist_x + dist_y * dist_y).sqrt();

    let dot = ((cx - x1) * (x2 - x1) + (cy - y1) * (y2 - y1)) / len.powi(2);
    let closest_x = x1 + dot * (x2 - x1);
    let closest_y = y1 + dot * (y2 - y1);

    if !collide_line_to_point(x1, y1, x2, y2, closest_x, closest_y) {
        return false;
    }

    let dist_x = closest_x - cx;
    let dist_y = closest_y - cy;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

    distance <= r
}

// LINE/POINT
pub fn collide_line_to_point(x1: f32, y1: f32, x2: f32, y2: f32, px: f32, py: f32) -> bool {
    let d1 = (px - x1).hypot(py - y1);
    let d2 = (px - x2).hypot(py - y2);
    let line_len = (x1 - x2).hypot(y1 - y2);

    let buffer = 0.1;

    (d1 + d2 >= line_len - buffer) && (d1 + d2 <= line_len + buffer)
}

// POINT/CIRCLE
pub fn collide_point_to_circle(px: f32, py: f32, cx: f32, cy: f32, r: f32) -> bool {
    let dist_x = px - cx;
    let dist_y = py - cy;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

    distance <= r
}

// CIRCLE/CIRCLE
pub fn collide_circle_to_circle(
    c1x: f32,
    c1y: f32,
    c1r: f32,
    c2x: f32,
    c2y: f32,
    c2r: f32,
) -> bool {
    // get distance between the circle's centers
    // use the Pythagorean Theorem to compute the distance
    let dist_x = c1x - c2x;
    let dist_y = c1y - c2y;
    let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();

    // if the distance is less than the sum of the circle's
    // radii, the circles are touching!
    if distance <= c1r + c2r {
        return true;
    }

    false
}

pub fn collid_poly_to_rect(
    vertices: &[Vec2],
    reversed: bool,
    rx: f32,
    ry: f32,
    rw: f32,
    rh: f32,
) -> bool {
    // Go through each of the vertices, plus the next vertex in the list
    for current in 0..vertices.len() {
        // Get the next vertex in the list, wrapping around to 0 if at the end
        let next = (current + 1) % vertices.len();

        let vc = vertices[current]; // Current vertex
        let vn = vertices[next]; // Next vertex

        // Check against all four sides of the rectangle
        if collide_line_to_rect(vc.x, vc.y, vn.x, vn.y, rx, ry, rw, rh) {
            return true;
        }

        // Optional: Test if the rectangle is inside the polygon
        if collide_poly_to_point(&Vec2::new(rx, ry), vertices, reversed) {
            return true;
        }
    }

    false
}

pub fn collide_line_to_rect(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    rx: f32,
    ry: f32,
    rw: f32,
    rh: f32,
) -> bool {
    // Check if the line has hit any of the rectangle's sides
    let left = collid_line_to_line(x1, y1, x2, y2, rx, ry, rx, ry + rh);
    let right = collid_line_to_line(x1, y1, x2, y2, rx + rw, ry, rx + rw, ry + rh);
    let top = collid_line_to_line(x1, y1, x2, y2, rx, ry, rx + rw, ry);
    let bottom = collid_line_to_line(x1, y1, x2, y2, rx, ry + rh, rx + rw, ry + rh);

    // If any side is true, the line has hit the rectangle
    left || right || top || bottom
}

pub fn collid_line_to_line(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> bool {
    // Calculate the direction of the lines
    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    if denominator.abs() < f32::EPSILON {
        return false; // Lines are parallel
    }

    let u_a = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
    let u_b = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;

    // If u_a and u_b are between 0 and 1, the lines are colliding
    (0.0..=1.0).contains(&u_a) && (0.0..=1.0).contains(&u_b)
}
