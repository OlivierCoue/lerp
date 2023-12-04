use godot::builtin::Vector2;

use crate::game::DELTA;

const GRID_SIZE_X_MIN: f32 = -1024.0;
const GRID_SIZE_X_MAX: f32 = 1024.0;
const GRID_SIZE_Y_MIN: f32 = -1024.0;
const GRID_SIZE_Y_MAX: f32 = 1024.0;

#[derive(Debug, Copy, Clone)]
pub struct GameEntityLocationParams {
    pub opt_current: Option<Vector2>,
    pub opt_target: Option<Vector2>,
    pub is_static: bool,
    pub delete_if_oob: bool,
    pub delete_at_target: bool,
    pub speed: f32,
}

pub struct GameEntityLocation {
    pub base: Vector2,
    current: Vector2,
    target: Vector2,
    pub is_static: bool,
    pub delete_if_oob: bool,
    pub delete_at_target: bool,
    pub speed: f32,
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
            None => Vector2::ZERO,
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

    pub fn bounded_x(x: f32) -> f32 {
        f32::min(f32::max(GRID_SIZE_X_MIN, x), GRID_SIZE_X_MAX)
    }

    pub fn bounded_y(y: f32) -> f32 {
        f32::min(f32::max(GRID_SIZE_Y_MIN, y), GRID_SIZE_Y_MAX)
    }

    pub fn get_revision(&self) -> u32 {
        self.revision
    }

    pub fn get_current(&self) -> &Vector2 {
        &self.current
    }

    pub fn get_target(&self) -> &Vector2 {
        &self.target
    }

    pub fn update_current(&mut self, x: f32, y: f32) {
        self.current.x = GameEntityLocation::bounded_x(x);
        self.current.y = GameEntityLocation::bounded_y(y);
        // self.revision += 1;
    }

    pub fn update_target(&mut self, x: f32, y: f32) {
        self.target.x = GameEntityLocation::bounded_x(x);
        self.target.y = GameEntityLocation::bounded_y(y);
        self.revision += 1;
    }

    pub fn move_to_target(&mut self) {
        if self.is_static || self.is_at_target() {
            return;
        }

        let new_coord = self.current.move_toward(self.target, self.speed * DELTA);

        self.update_current(new_coord.x, new_coord.y);
    }

    // pub fn get_distance_from_base(&self) -> f32 {
    //     (f32::powf(self.current.x - self.base.x, 2.0)
    //         + f32::powf(self.current.y - self.base.y, 2.0))
    //     .sqrt()
    //     .round()
    // }

    pub fn is_oob(&self) -> bool {
        self.current.x <= GRID_SIZE_X_MIN
            || self.current.y <= GRID_SIZE_Y_MIN
            || self.current.x >= GRID_SIZE_X_MAX
            || self.current.y >= GRID_SIZE_Y_MAX
    }

    pub fn should_be_delete(&self) -> bool {
        (self.delete_at_target && (self.is_at_target() || self.is_oob()))
            || self.delete_if_oob && self.is_oob()
    }

    pub fn is_at_target(&self) -> bool {
        self.current == self.target
    }
}
