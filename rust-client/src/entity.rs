use godot::engine::global::HorizontalAlignment;
use godot::engine::utilities::rad_to_deg;
use godot::engine::{
    AnimatedSprite2D, ISprite2D, Label, Polygon2D, ResourceLoader, Sprite2D, Texture2D,
};
use godot::prelude::*;
use rust_common::helper::point_to_vector2;
use rust_common::proto::common::GameEntityBaseType;
use rust_common::proto::udp_down::UdpMsgDownGameEntityUpdate;

use crate::root::{Root, DEBUG};
use crate::utils::{
    angle_to_direction, cart_to_iso, get_idle_animation_for_direction,
    get_walk_animation_for_direction, iso_to_cart, Direction,
};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct GameEntity {
    is_current_player: bool,
    position_init: Vector2,
    position_target: Vector2,
    speed: f32,
    base_type: GameEntityBaseType,
    health_label: Option<Gd<Label>>,
    is_dead: bool,
    animated_sprite_2d: Option<Gd<AnimatedSprite2D>>,
    #[base]
    base: Base<Sprite2D>,
    direction: Direction,
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
            is_dead: false,
            animated_sprite_2d: None,
            direction: Direction::N,
        }
    }

    fn ready(&mut self) {
        self.base.set_position(cart_to_iso(&self.position_init));
        match self.base_type {
            GameEntityBaseType::CHARACTER => {
                let animated_sprite_2d_scene =
                    load::<PackedScene>("res://warrior_animated_sprite_2d.tscn");
                let mut animated_sprite_2d =
                    animated_sprite_2d_scene.instantiate_as::<AnimatedSprite2D>();
                animated_sprite_2d.set_scale(Vector2::new(3.0, 3.0));
                self.animated_sprite_2d = Some(animated_sprite_2d.clone());
                self.base.add_child(animated_sprite_2d.upcast());
            }
            GameEntityBaseType::PROJECTILE => {
                let texture = ResourceLoader::singleton()
                    .load("res://fireball.png".into())
                    .unwrap()
                    .cast::<Texture2D>();
                self.base.set_texture(texture);
            }
            GameEntityBaseType::ENEMY => {
                let animated_sprite_2d_scene =
                    load::<PackedScene>("res://warrior_animated_sprite_2d.tscn");
                let mut animated_sprite_2d =
                    animated_sprite_2d_scene.instantiate_as::<AnimatedSprite2D>();
                animated_sprite_2d.set_scale(Vector2::new(3.0, 3.0));
                self.animated_sprite_2d = Some(animated_sprite_2d.clone());
                self.base.add_child(animated_sprite_2d.upcast());
            }
        };
        if self.is_current_player {
            let mut camera = Camera2D::new_alloc();
            camera.set_enabled(true);
            self.base.add_child(camera.upcast());
        }

        self.set_position_target(iso_to_cart(&self.base.get_position()), self.position_target);

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

    fn process(&mut self, delta: f64) {
        let is_moving = self.position_target != iso_to_cart(&self.base.get_position());

        if is_moving {
            let new_position = iso_to_cart(&self.base.get_position())
                .move_toward(self.position_target, self.speed * delta as f32);
            self.base.set_position(cart_to_iso(&new_position));
        }

        if let Some(animated_sprite_2d) = self.animated_sprite_2d.as_mut() {
            if is_moving {
                if !animated_sprite_2d.is_playing() {
                    animated_sprite_2d.play();
                }
            } else {
                animated_sprite_2d.set_animation(
                    get_idle_animation_for_direction(&self.direction)
                        .as_str()
                        .into(),
                );
            }
        }
    }
}

#[godot_api]
impl GameEntity {
    #[func]
    fn on_player_move_start(&mut self, _: Vector2) {}
    #[func]
    fn on_player_throw_fireball_start(&mut self) {
        if self.is_current_player {
            self.position_target = iso_to_cart(&self.base.get_position());
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

        if DEBUG {
            let a = Vector2::new(
                -(entity_update.location_shape.x / 2.0),
                -(entity_update.location_shape.y / 2.0),
            );
            let b = Vector2::new(
                entity_update.location_shape.x / 2.0,
                -(entity_update.location_shape.y / 2.0),
            );
            let c = Vector2::new(
                entity_update.location_shape.x / 2.0,
                entity_update.location_shape.y / 2.0,
            );
            let d = Vector2::new(
                -(entity_update.location_shape.x / 2.0),
                entity_update.location_shape.y / 2.0,
            );
            let mut polygon2d = Polygon2D::new_alloc();
            let mut packed_vector2_array = PackedVector2Array::new();
            packed_vector2_array.insert(0, cart_to_iso(&a));
            packed_vector2_array.insert(1, cart_to_iso(&b));
            packed_vector2_array.insert(2, cart_to_iso(&c));
            packed_vector2_array.insert(3, cart_to_iso(&d));
            polygon2d.set_polygon(packed_vector2_array);
            polygon2d.set_color(Color::from_rgba(255.0, 0.0, 0.0, 0.5));
            polygon2d.set_z_index(10);
            self.base.add_child(polygon2d.upcast());
        }
    }

    pub fn update_from_server(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        let new_position_target = point_to_vector2(&entity_update.location_target);
        let new_position_current = point_to_vector2(&entity_update.location_current);

        self.speed = entity_update.location_speed.unwrap();

        if self.position_target != new_position_target {
            self.set_position_target(iso_to_cart(&self.base.get_position()), new_position_target);
        }

        if iso_to_cart(&self.base.get_position()).distance_to(new_position_current) > 300.0 {
            self.base.set_position(cart_to_iso(&new_position_current));
        }
        if let Some(health_label) = &mut self.health_label {
            if let Some(new_health_current) = entity_update.health_current {
                health_label.set_text(new_health_current.to_string().into());
                if new_health_current == 0 {
                    self.base.set_visible(false);
                    self.is_dead = true;
                } else {
                    self.base.set_visible(true);
                    self.is_dead = false;
                }
            }
        }
    }

    fn set_position_target(
        &mut self,
        location_current_cart: Vector2,
        location_target_cart: Vector2,
    ) {
        self.position_target = location_target_cart;

        if let Some(animated_sprite_2d) = self.animated_sprite_2d.as_mut() {
            let angle_to_target =
                rad_to_deg(location_current_cart.angle_to_point(self.position_target) as f64);

            self.direction = angle_to_direction(angle_to_target as f32);

            animated_sprite_2d.set_animation(
                get_walk_animation_for_direction(&self.direction)
                    .as_str()
                    .into(),
            );
        }
    }
}
