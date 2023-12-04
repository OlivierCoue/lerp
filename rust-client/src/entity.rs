use godot::engine::{ISprite2D, ResourceLoader, Sprite2D, Texture2D};
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
                self.base.set_scale(Vector2 { x: 0.5, y: 0.5 });
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
            self.position_target = target;
            self.speed /= 4.0;
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
    }

    pub fn update_from_server(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        let new_position_target = point_to_vector2(&entity_update.location_target);
        let new_position_current = point_to_vector2(&entity_update.location_current);

        self.speed = entity_update.location_speed.unwrap();
        if self.position_target.distance_to(new_position_target) > 60.0 {
            self.position_target = new_position_target;
        }
        if self.base.get_position().distance_to(new_position_current) > 200.0 {
            self.base.set_position(new_position_current);
        }
    }
}
