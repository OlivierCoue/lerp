use std::ops;

pub fn get_point_from_points_and_distance(from: Vec2, to: Vec2, distance: f32) -> Vec2 {
    let angle = from.angle_to_point(to);
    Vec2::new(
        from.x + distance * f32::cos(angle),
        from.y + distance * f32::sin(angle),
    )
}

const CMP_EPSILON: f32 = 0.00001;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn move_toward(self, to: Self, delta: f32) -> Self {
        let vd = to - self;
        let len = vd.length();
        if len <= delta || len < CMP_EPSILON {
            to
        } else {
            self + vd / len * delta
        }
    }

    pub fn length(self) -> f32 {
        self.to_glam().length()
    }

    pub fn distance_to(self, to: Self) -> f32 {
        (to - self).length()
    }

    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn angle_to_point(self, to: Self) -> f32 {
        (to - self).angle()
    }

    fn to_glam(self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.y)
    }
}
impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
