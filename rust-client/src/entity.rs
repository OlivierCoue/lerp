use godot::engine::global::HorizontalAlignment;
use godot::engine::{ISprite2D, Label, Panel, ResourceLoader, Sprite2D, StyleBoxFlat, Texture2D};
use godot::prelude::*;
use rust_common::helper::point_to_vector2;
use rust_common::proto::common::GameEntityBaseType;
use rust_common::proto::udp_down::UdpMsgDownGameEntityUpdate;

use crate::root::Root;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct GameEntity {
    is_current_player: bool,
    position_init: Vector2,
    position_target: Vector2,
    speed: f32,
    base_type: GameEntityBaseType,
    health_label: Option<Gd<Label>>,
    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for GameEntity {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
            position_init: Vector2::ZERO,
            position_target: Vector2::ZERO,
            speed: 0.0,
            base_type: GameEntityBaseType::CHARACTER,
            is_current_player: false,
            health_label: None,
        }
    }

    fn ready(&mut self) {
        self.base.set_position(self.position_init);
        match self.base_type {
            GameEntityBaseType::CHARACTER => {
                let texture = ResourceLoader::singleton()
                    .load("res://wizard.png".into())
                    .unwrap()
                    .cast::<Texture2D>();
                self.base.set_texture(texture);
                // self.base.set_scale(Vector2 { x: 0.5, y: 0.5 });
            }
            GameEntityBaseType::PROJECTILE => {
                let texture = ResourceLoader::singleton()
                    .load("res://fireball.png".into())
                    .unwrap()
                    .cast::<Texture2D>();
                self.base.set_texture(texture);
            }
        };

        if self.is_current_player {
            let mut camera = Camera2D::new_alloc();
            camera.set_enabled(true);
            self.base.add_child(camera.upcast());
        }

        let mut root = self.base.get_node_as::<Root>("/root/Root");
        root.connect(
            "player_throw_fireball_start".into(),
            self.base.callable("on_player_throw_fireball_start"),
        );
        root.connect(
            "player_move_start".into(),
            self.base.callable("on_player_move_start"),
        );
    }

    fn physics_process(&mut self, delta: f64) {
        if self.position_target != self.base.get_position() {
            let new_position = self
                .base
                .get_position()
                .move_toward(self.position_target, self.speed * delta as f32);
            self.base.set_position(new_position);
        }
    }
}

#[godot_api]
impl GameEntity {
    #[func]
    fn on_player_move_start(&mut self, target: Vector2) {
        if self.is_current_player {
            // Reduce speed if player start to move
            // this reduce the diff between server en client position
            if self.base.get_position() == self.position_target {
                self.speed /= 4.0;
            }
            self.position_target = target;
        }
    }
    #[func]
    fn on_player_throw_fireball_start(&mut self) {
        if self.is_current_player {
            self.position_target = self.base.get_position();
        }
    }
}

impl GameEntity {
    pub fn set_init_state(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        self.position_init = point_to_vector2(&entity_update.location_current);
        self.position_target = point_to_vector2(&entity_update.location_target);
        if let Some(speed) = entity_update.location_speed {
            self.speed = speed;
        }
        self.base_type = entity_update.object_type.unwrap();
        self.is_current_player = entity_update.is_self;

        // Draw health
        if let Some(health_current) = entity_update.health_current {
            let mut health_label = Label::new_alloc();
            health_label.set_text(health_current.to_string().into());
            health_label.set_horizontal_alignment(HorizontalAlignment::HORIZONTAL_ALIGNMENT_CENTER);
            health_label.set_position(Vector2 {
                x: 0.0,
                y: -(&entity_update.location_shape.y / 2.0 + 40.0),
            });
            self.health_label = Some(health_label.clone());
            self.base.add_child(health_label.upcast());
        }

        // Draw shape outline
        let mut shape_pannel = Panel::new_alloc();
        shape_pannel.set_size(point_to_vector2(&entity_update.location_shape));
        shape_pannel.set_position(Vector2 {
            x: -(&entity_update.location_shape.x / 2.0),
            y: -(&entity_update.location_shape.y / 2.0),
        });

        let mut stylebox_outline: Gd<StyleBoxFlat> = shape_pannel
            .get_theme_stylebox("panel".into())
            .unwrap()
            .cast();
        stylebox_outline.set_draw_center(false);
        stylebox_outline.set_border_width_all(2);
        stylebox_outline.set_border_color(Color::from_rgb(255.0, 0.0, 0.0));

        shape_pannel.add_theme_stylebox_override("panel".into(), stylebox_outline.upcast());

        self.base.add_child(shape_pannel.upcast());
    }

    pub fn update_from_server(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        let new_position_target = point_to_vector2(&entity_update.location_target);
        let new_position_current = point_to_vector2(&entity_update.location_current);
        self.speed = entity_update.location_speed.unwrap();
        if self.position_target.distance_to(new_position_target) > 100.0 {
            self.position_target = new_position_target;
        }
        if self.base.get_position().distance_to(new_position_current) > 200.0 {
            self.base.set_position(new_position_current);
        }
        if let Some(health_label) = &mut self.health_label {
            if let Some(new_health_current) = entity_update.health_current {
                health_label.set_text(new_health_current.to_string().into());
            }
        }
    }
}
