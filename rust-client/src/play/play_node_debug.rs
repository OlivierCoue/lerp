use godot::prelude::*;

use crate::utils::cart_to_iso;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayNodeDebug {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for PlayNodeDebug {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("PlayNodeDebug init");

        Self { base }
    }

    fn ready(&mut self) {
        self.base_mut().set_y_sort_enabled(true);
    }

    fn draw(&mut self) {
        let min = 0.0;
        let max = 2048.0;
        let cell_size = 30.0;
        self.base_mut().draw_line(
            cart_to_iso(&Vector2::new(min, min)),
            cart_to_iso(&Vector2::new(max, min)),
            Color::from_rgb(255.0, min, min),
        );
        self.base_mut().draw_line(
            cart_to_iso(&Vector2::new(max, min)),
            cart_to_iso(&Vector2::new(max, max)),
            Color::from_rgb(255.0, min, min),
        );
        self.base_mut().draw_line(
            cart_to_iso(&Vector2::new(min, max)),
            cart_to_iso(&Vector2::new(max, max)),
            Color::from_rgb(255.0, min, min),
        );
        self.base_mut().draw_line(
            cart_to_iso(&Vector2::new(min, min)),
            cart_to_iso(&Vector2::new(min, max)),
            Color::from_rgb(255.0, min, min),
        );

        for i in 0..((max / cell_size) as i32) {
            self.base_mut().draw_line(
                cart_to_iso(&Vector2::new(i as f32 * cell_size, min)),
                cart_to_iso(&Vector2::new(i as f32 * cell_size, max)),
                Color::from_rgb(255.0, min, min),
            );
            self.base_mut().draw_line(
                cart_to_iso(&Vector2::new(min, i as f32 * cell_size)),
                cart_to_iso(&Vector2::new(max, i as f32 * cell_size)),
                Color::from_rgb(255.0, min, min),
            );
        }
        self.base_mut().set_z_index(2);
    }
}
