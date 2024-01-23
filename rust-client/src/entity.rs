use std::collections::VecDeque;

use godot::engine::global::HorizontalAlignment;
use godot::engine::utilities::rad_to_deg;
use godot::engine::{
    AnimatedSprite2D, CharacterBody2D, CollisionShape2D, ConvexPolygonShape2D, ISprite2D, Label,
    Polygon2D, ResourceLoader, Sprite2D, Texture2D,
};
use godot::prelude::*;
use rust_common::helper::point_to_vector2;
use rust_common::proto::common::GameEntityBaseType;
use rust_common::proto::udp_down::UdpMsgDownGameEntityUpdate;

use crate::root::{Root, DEBUG};
use crate::utils::{
    angle_to_direction, cart_to_iso, get_attack_animation_for_direction,
    get_idle_animation_for_direction, get_walk_animation_for_direction, iso_to_cart, Direction,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct GameEntity {
    is_current_player: bool,
    position_init: Vector2,
    position_target_queue: VecDeque<Vector2>,
    speed: f32,
    base_type: GameEntityBaseType,
    health_label: Option<Gd<Label>>,
    is_dead: bool,
    animated_sprite_2d: Option<Gd<AnimatedSprite2D>>,
    #[base]
    base: Base<CharacterBody2D>,
    direction: Direction,
    is_casting: bool,
    casting_target: Option<Vector2>,
}

#[godot_api]
impl ISprite2D for GameEntity {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            position_init: Vector2::ZERO,
            position_target_queue: VecDeque::new(),
            speed: 0.0,
            base_type: GameEntityBaseType::CHARACTER,
            is_current_player: false,
            health_label: None,
            is_dead: false,
            animated_sprite_2d: None,
            direction: Direction::N,
            is_casting: false,
            casting_target: None,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_z_index(2);
        self.base_mut().set_y_sort_enabled(true);
        let position_init = self.position_init;
        self.base_mut().set_position(cart_to_iso(&position_init));
        match self.base_type {
            GameEntityBaseType::CHARACTER => {
                let animated_sprite_2d_scene =
                    load::<PackedScene>("res://animated_sprite_2d/warrior.tscn");
                let mut animated_sprite_2d =
                    animated_sprite_2d_scene.instantiate_as::<AnimatedSprite2D>();
                animated_sprite_2d.play();
                animated_sprite_2d.set_scale(Vector2::new(3.0, 3.0));
                self.animated_sprite_2d = Some(animated_sprite_2d.clone());
                self.base_mut().add_child(animated_sprite_2d.upcast());
            }
            GameEntityBaseType::PROJECTILE => {
                let mut sprite_2d = Sprite2D::new_alloc();
                let texture = ResourceLoader::singleton()
                    .load("res://sprite/fireball.png".into())
                    .unwrap()
                    .cast::<Texture2D>();
                sprite_2d.set_texture(texture);
                self.base_mut().add_child(sprite_2d.upcast());
            }
            GameEntityBaseType::ENEMY => {
                let animated_sprite_2d_scene =
                    load::<PackedScene>("res://animated_sprite_2d/warrior.tscn");
                let mut animated_sprite_2d =
                    animated_sprite_2d_scene.instantiate_as::<AnimatedSprite2D>();
                animated_sprite_2d.play();
                animated_sprite_2d.set_scale(Vector2::new(3.0, 3.0));
                self.animated_sprite_2d = Some(animated_sprite_2d.clone());
                self.base_mut().add_child(animated_sprite_2d.upcast());
            }
            GameEntityBaseType::WALL => {}
            GameEntityBaseType::MELEE_ATTACK => {}
        };
        if self.is_current_player {
            let mut camera = Camera2D::new_alloc();
            camera.set_enabled(true);
            camera.set_zoom(Vector2::new(1.5, 1.5));
            self.base_mut().add_child(camera.upcast());
        }

        self.update_animated_sprite();

        let mut root = self.base().get_node_as::<Root>("/root/Root");
        root.connect(
            "player_throw_fireball_start".into(),
            self.base().callable("on_player_throw_fireball_start"),
        );
        root.connect(
            "player_move_start".into(),
            self.base().callable("on_player_move_start"),
        );
    }

    fn process(&mut self, delta: f64) {
        if let Some(position_target) = self.position_target_queue.get(0) {
            let vd = *position_target - iso_to_cart(&self.base().get_position());
            let len = vd.length();
            if len <= self.speed * delta as f32 || len < real::CMP_EPSILON {
                self.position_target_queue.pop_front();
                self.update_animated_sprite();
            }
        }

        if let Some(position_target) = self.position_target_queue.get(0) {
            let is_moving = *position_target != iso_to_cart(&self.base().get_position());

            if is_moving {
                let velocity = iso_to_cart(&self.base().get_position())
                    .direction_to(*position_target)
                    * self.speed
                    * delta as f32;

                self.base_mut().move_and_collide(cart_to_iso(&velocity));
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
            self.position_target_queue.clear();
        }
    }
}

impl GameEntity {
    pub fn set_init_state(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        self.position_init = point_to_vector2(&entity_update.location_current);
        self.position_target_queue = VecDeque::from_iter(
            entity_update
                .location_target_queue
                .iter()
                .map(|p| point_to_vector2(&p))
                .collect::<Vec<_>>(),
        );

        if let Some(speed) = entity_update.velocity_speed {
            self.speed = speed;
        }
        self.base_type = entity_update.object_type.unwrap();
        self.is_current_player = entity_update.is_self;

        // Draw health
        if let Some(health_current) = entity_update.health_current {
            let mut health_label = Label::new_alloc();
            health_label.set_text(health_current.to_string().into());
            health_label.set_horizontal_alignment(HorizontalAlignment::CENTER);
            health_label.set_position(Vector2 {
                x: 0.0,
                y: -(&entity_update.collider_dmg_in_rect.y / 2.0 + 40.0),
            });
            health_label.set_z_index(3);
            self.health_label = Some(health_label.clone());
            self.base_mut().add_child(health_label.upcast());
        }

        if DEBUG {
            // Draw hitbox
            let a = Vector2::new(
                -(entity_update.collider_dmg_in_rect.x / 2.0),
                -(entity_update.collider_dmg_in_rect.y / 2.0),
            );
            let b = Vector2::new(
                entity_update.collider_dmg_in_rect.x / 2.0,
                -(entity_update.collider_dmg_in_rect.y / 2.0),
            );
            let c = Vector2::new(
                entity_update.collider_dmg_in_rect.x / 2.0,
                entity_update.collider_dmg_in_rect.y / 2.0,
            );
            let d = Vector2::new(
                -(entity_update.collider_dmg_in_rect.x / 2.0),
                entity_update.collider_dmg_in_rect.y / 2.0,
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
            self.base_mut().add_child(polygon2d.upcast());
        }

        if let Some(collider_mvt_rect) = entity_update.collider_mvt_rect.as_ref() {
            let mut collision_shape_2d = CollisionShape2D::new_alloc();
            let mut shape = ConvexPolygonShape2D::new_gd();
            let mut packed_vector2_array = PackedVector2Array::new();
            let a = Vector2::new(-(collider_mvt_rect.x / 2.0), -(collider_mvt_rect.y / 2.0));
            let b = Vector2::new(collider_mvt_rect.x / 2.0, -(collider_mvt_rect.y / 2.0));
            let c = Vector2::new(collider_mvt_rect.x / 2.0, collider_mvt_rect.y / 2.0);
            let d = Vector2::new(-(collider_mvt_rect.x / 2.0), collider_mvt_rect.y / 2.0);
            packed_vector2_array.insert(0, cart_to_iso(&a));
            packed_vector2_array.insert(1, cart_to_iso(&b));
            packed_vector2_array.insert(2, cart_to_iso(&c));
            packed_vector2_array.insert(3, cart_to_iso(&d));
            shape.set_points(packed_vector2_array);
            collision_shape_2d.set_shape(shape.upcast());
            self.base_mut().add_child(collision_shape_2d.upcast());
        }
    }

    pub fn update_from_server(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        self.speed = entity_update.velocity_speed.unwrap();
        self.position_target_queue = VecDeque::from_iter(
            entity_update
                .location_target_queue
                .iter()
                .map(|p| point_to_vector2(&p))
                .collect::<Vec<_>>(),
        );

        if entity_update.cast.is_some() {
            self.casting_target = Some(point_to_vector2(&entity_update.cast.target));
            self.is_casting = true;
        } else {
            self.casting_target = None;
            self.is_casting = false;
        }

        self.update_animated_sprite();

        let new_position_current = point_to_vector2(&entity_update.location_current);
        if iso_to_cart(&self.base().get_position()).distance_to(new_position_current) > 300.0 {
            self.base_mut()
                .set_position(cart_to_iso(&new_position_current));
        }

        if let Some(health_label) = &mut self.health_label {
            if let Some(new_health_current) = entity_update.health_current {
                health_label.set_text(new_health_current.to_string().into());
                if new_health_current == 0 {
                    self.base_mut().set_visible(false);
                    self.is_dead = true;
                } else {
                    self.base_mut().set_visible(true);
                    self.is_dead = false;
                }
            }
        }
    }

    fn update_animated_sprite(&mut self) {
        if let Some(target) = self.position_target_queue.get(0) {
            self.direction = self.get_direction(target);
            if let Some(animated_sprite_2d) = self.animated_sprite_2d.as_mut() {
                animated_sprite_2d.set_animation(
                    get_walk_animation_for_direction(&self.direction)
                        .as_str()
                        .into(),
                );
            }
        } else if let Some(target) = self.casting_target {
            self.direction = self.get_direction(&target);
            if let Some(animated_sprite_2d) = self.animated_sprite_2d.as_mut() {
                animated_sprite_2d.set_animation(
                    get_attack_animation_for_direction(&self.direction)
                        .as_str()
                        .into(),
                );
            }
        } else if let Some(animated_sprite_2d) = self.animated_sprite_2d.as_mut() {
            animated_sprite_2d.set_animation(
                get_idle_animation_for_direction(&self.direction)
                    .as_str()
                    .into(),
            );
        }
    }

    fn get_direction(&self, target: &Vector2) -> Direction {
        let location_current_cart = iso_to_cart(&self.base().get_position());
        let location_target_cart = *target;

        let angle_to_target =
            rad_to_deg(location_current_cart.angle_to_point(location_target_cart) as f64);
        angle_to_direction(angle_to_target as f32)
    }
}
