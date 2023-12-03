use godot::engine::ISprite2D;
use godot::engine::ResourceLoader;
use godot::engine::Sprite2D;
use godot::engine::Texture2D;
use godot::prelude::*;
use rust_common::proto::data::GameEntityBaseType;
use rust_common::proto::data::UdpMsgDownGameEntityUpdate;

use crate::network::Network;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct GameEntity {
    position_init: Vector2,
    position_target: Vector2,
    base_type: GameEntityBaseType,
    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for GameEntity {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Player init");

        Self {
            base,
            position_init: Vector2::ZERO,
            position_target: Vector2::ZERO,
            base_type: GameEntityBaseType::CHARACTER,
        }
    }

    fn ready(&mut self) {
        self.base.set_position(self.position_init);
        let texture = match self.base_type {
            GameEntityBaseType::CHARACTER => ResourceLoader::singleton()
                .load("res://icon.svg".into())
                .unwrap()
                .cast::<Texture2D>(),
            GameEntityBaseType::PROJECTILE => ResourceLoader::singleton()
                .load("res://fireball.png".into())
                .unwrap()
                .cast::<Texture2D>(),
        };
        self.base.set_texture(texture);
        let mut network = self.base.get_node_as::<Network>("/root/Root/Network");
        network.connect(
            "udp_msg_down_received".into(),
            self.base.callable("on_udp_msg_down_received"),
        );

        godot_print!("GameEntity ready");
    }

    fn physics_process(&mut self, delta: f64) {
        // GDScript code:
        //
        // rotation += angular_speed * delta
        // var velocity = Vector2.UP.rotated(rotation) * speed
        // position += velocity * delta

        // self.base.rotate((self.angular_speed * delta) as f32);

        // let rotation = self.base.get_rotation();
        // let velocity = Vector2:: * self.speed as f32;
        // self.base.translate(velocity * delta as f32);

        let speed: f32 = 1000.0;
        // or verbose:
        if self.position_target != self.base.get_position() {
            let new_position = self
                .base
                .get_position()
                .move_toward(self.position_target, speed * delta as f32);
            self.base.set_position(new_position);
        }

        // if self.base.get_position().distance_to(self.position_target) <= 10.0 {
        //     godot_print!("Reach destination");
        //     self.base.set_position(self.position_target);
        //     self.t = 0.0;
        // }
    }
}

#[godot_api]
impl GameEntity {
    #[func]
    fn on_udp_msg_down_received(&mut self) {}
}

impl GameEntity {
    pub fn set_init_state(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        self.position_init = Vector2 {
            x: entity_update.location_current.x as f32,
            y: entity_update.location_current.y as f32,
        };
        self.position_target = Vector2 {
            x: entity_update.location_target.x as f32,
            y: entity_update.location_target.y as f32,
        };
        self.base_type = entity_update.object_type.unwrap();
    }
    pub fn set_position_target(&mut self, vector2: &Vector2) {
        self.position_target.x = vector2.x;
        self.position_target.y = vector2.y;
    }
}
