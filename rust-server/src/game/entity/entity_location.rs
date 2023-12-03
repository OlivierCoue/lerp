use crate::{maths::get_point_with_speed, utils::Coord};

const GRID_SIZE_X: f64 = 1000.0;
const GRID_SIZE_Y: f64 = 1000.0;

#[derive(Debug, Copy, Clone)]
pub struct GameEntityLocationParams {
    pub opt_current: Option<Coord>,
    pub opt_target: Option<Coord>,
    pub is_static: bool,
    pub delete_if_oob: bool,
    pub delete_at_target: bool,
    pub speed: f64,
}

pub struct GameEntityLocation {
    pub base: Coord,
    current: Coord,
    target: Coord,
    pub is_static: bool,
    pub delete_if_oob: bool,
    pub delete_at_target: bool,
    pub speed: f64,
    revision: u32,
}
impl GameEntityLocation {
    pub fn new(params: GameEntityLocationParams) -> GameEntityLocation {
        let GameEntityLocationParams {
            opt_current,
            opt_target,
            delete_if_oob,
            is_static,
            delete_at_target,
            speed,
        } = params;

        let current = match opt_current {
            Some(current) => current,
            None => Coord { x: 0.0, y: 0.0 },
        };

        let target = match opt_target {
            Some(target) => target,
            None => current,
        };

        GameEntityLocation {
            base: current,
            current,
            target,
            delete_if_oob,
            is_static,
            delete_at_target,
            speed,
            revision: 0,
        }
    }

    pub fn bounded_x(x: f64) -> f64 {
        f64::min(f64::max(0.0, x), GRID_SIZE_X)
    }

    pub fn bounded_y(y: f64) -> f64 {
        f64::min(f64::max(0.0, y), GRID_SIZE_Y)
    }

    pub fn get_revision(&self) -> u32 {
        self.revision
    }

    pub fn get_current(&self) -> &Coord {
        &self.current
    }

    pub fn get_target(&self) -> &Coord {
        &self.target
    }

    pub fn update_current(&mut self, x: f64, y: f64) {
        self.current.x = GameEntityLocation::bounded_x(x);
        self.current.y = GameEntityLocation::bounded_y(y);
        self.revision += 1;
    }

    pub fn update_target(&mut self, x: f64, y: f64) {
        self.target.x = GameEntityLocation::bounded_x(x);
        self.target.y = GameEntityLocation::bounded_y(y);
        self.revision += 1;
    }

    pub fn move_to_target(&mut self) {
        if self.is_static || self.is_at_target() {
            return;
        }

        let new_coord = get_point_with_speed(&self.current, &self.target, self.speed);

        let precision_up = (self.speed / 2.0).ceil() as i32;
        let precision_down = (self.speed / 2.0).floor() as i32;

        let new_x = if ((self.target.x as i32) - precision_down
            ..(self.target.x as i32) + precision_up)
            .contains(&(new_coord.x.round() as i32))
        {
            self.target.x
        } else {
            new_coord.x.round()
        };

        let new_y = if ((self.target.y as i32) - precision_down
            ..(self.target.y as i32) + precision_up)
            .contains(&(new_coord.y.round() as i32))
        {
            self.target.y
        } else {
            new_coord.y.round()
        };

        self.update_current(new_x, new_y);
    }

    // pub fn get_distance_from_base(&self) -> f64 {
    //     (f64::powf(self.current.x - self.base.x, 2.0)
    //         + f64::powf(self.current.y - self.base.y, 2.0))
    //     .sqrt()
    //     .round()
    // }

    pub fn is_oob(&self) -> bool {
        self.current.x <= 0.0
            || self.current.y <= 0.0
            || self.current.x >= GRID_SIZE_X
            || self.current.y >= GRID_SIZE_Y
    }

    pub fn should_be_delete(&self) -> bool {
        self.delete_at_target && self.is_at_target() || self.delete_if_oob && self.is_oob()
    }

    pub fn is_at_target(&self) -> bool {
        self.current.x == self.target.x && self.current.y == self.target.y
    }
}
