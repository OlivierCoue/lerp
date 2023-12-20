use godot::engine::{ISprite2D, Panel, Sprite2D, StyleBoxFlat};
use godot::prelude::*;
use rust_common::helper::point_to_vector2;

use rust_common::proto::udp_down::UdpMsgDownGameEntityUpdate;

use crate::utils::cart_to_iso;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct GameServerEntity {
    position_init: Vector2,
    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for GameServerEntity {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
            position_init: Vector2::ZERO,
        }
    }

    fn ready(&mut self) {
        self.base.set_position(cart_to_iso(&self.position_init));
    }
}

impl GameServerEntity {
    pub fn set_init_state(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        self.position_init = point_to_vector2(&entity_update.location_current);

        // Draw shape outline
        let mut shape_pannel = Panel::new_alloc();
        shape_pannel.set_size(point_to_vector2(&entity_update.collider_dmg_in_rect));
        shape_pannel.set_position(Vector2 {
            x: -(&entity_update.collider_dmg_in_rect.x / 2.0),
            y: -(&entity_update.collider_dmg_in_rect.y / 2.0),
        });

        let mut stylebox_outline: Gd<StyleBoxFlat> = shape_pannel
            .get_theme_stylebox("panel".into())
            .unwrap()
            .cast();
        stylebox_outline.set_draw_center(false);
        stylebox_outline.set_border_width_all(2);
        stylebox_outline.set_border_color(Color::from_rgb(0.0, 0.0, 255.0));

        shape_pannel.add_theme_stylebox_override("panel".into(), stylebox_outline.upcast());

        self.base.add_child(shape_pannel.upcast());
    }

    pub fn update_from_server(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        let new_position_current = point_to_vector2(&entity_update.location_current);

        self.base.set_position(cart_to_iso(&new_position_current));
    }
}
