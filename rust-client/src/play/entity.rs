use std::collections::VecDeque;

use godot::engine::collision_polygon_2d::BuildMode;
use godot::engine::global::HorizontalAlignment;
use godot::engine::utilities::rad_to_deg;
use godot::engine::{
    AnimatedSprite2D, CharacterBody2D, CollisionPolygon2D, CollisionShape2D, ConvexPolygonShape2D,
    ISprite2D, Label, Polygon2D, ResourceLoader, Sprite2D, Texture2D,
};
use godot::prelude::*;
use rust_common::proto::{GameEntityBaseType, UdpMsgDownGameEntityUpdate};

use crate::root::{DEBUG, PATH_PLAY};
use crate::utils::{
    angle_to_direction, cart_to_iso, get_attack_animation_for_direction,
    get_idle_animation_for_direction, get_walk_animation_for_direction, iso_to_cart,
    point_to_vector2, Direction,
};

use super::play_node::PlayNode;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct GameEntity {
    is_current_player: bool,
    position_init: Vector2,
    position_target_queue: VecDeque<Vector2>,
    server_position_current: Option<Vector2>,
    allow_server_position_sync: bool,
    speed: f32,
    base_type: GameEntityBaseType,
    health_label: Option<Gd<Label>>,
    is_dead: bool,
    animated_splite_2d_scene: Option<Gd<PackedScene>>,
    animated_sprite_2d: Option<Gd<AnimatedSprite2D>>,
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
            server_position_current: None,
            allow_server_position_sync: false,
            speed: 0.0,
            base_type: GameEntityBaseType::Character,
            is_current_player: false,
            health_label: None,
            is_dead: false,
            animated_splite_2d_scene: None,
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
            GameEntityBaseType::Character => {
                let mut animated_sprite_2d = self
                    .animated_splite_2d_scene
                    .clone()
                    .unwrap()
                    .instantiate_as::<AnimatedSprite2D>();
                animated_sprite_2d.play();
                animated_sprite_2d.set_scale(Vector2::new(3.0, 3.0));
                self.animated_sprite_2d = Some(animated_sprite_2d.clone());
                self.base_mut().add_child(animated_sprite_2d.upcast());
            }
            GameEntityBaseType::Projectile => {
                let mut sprite_2d = Sprite2D::new_alloc();
                let texture = ResourceLoader::singleton()
                    .load("res://sprite/fireball.png".into())
                    .unwrap()
                    .cast::<Texture2D>();
                sprite_2d.set_texture(texture);
                self.base_mut().add_child(sprite_2d.upcast());
            }
            GameEntityBaseType::Enemy => {
                let mut animated_sprite_2d = self
                    .animated_splite_2d_scene
                    .clone()
                    .unwrap()
                    .instantiate_as::<AnimatedSprite2D>();
                animated_sprite_2d.play();
                animated_sprite_2d.set_scale(Vector2::new(3.0, 3.0));
                self.animated_sprite_2d = Some(animated_sprite_2d.clone());
                self.base_mut().add_child(animated_sprite_2d.upcast());
            }
            GameEntityBaseType::Wall => {}
            GameEntityBaseType::MeleeAttack => {}
        };
        if self.is_current_player {
            let mut camera = Camera2D::new_alloc();
            camera.set_enabled(true);
            camera.set_zoom(Vector2::new(1.8, 1.8));
            self.base_mut().add_child(camera.upcast());
        }

        self.update_animated_sprite();

        let mut play_node = self.base().get_node_as::<PlayNode>(PATH_PLAY);

        play_node.connect(
            "player_throw_fireball_start".into(),
            self.base().callable("on_player_throw_fireball_start"),
        );
        play_node.connect(
            "player_move_start".into(),
            self.base().callable("on_player_move_start"),
        );
    }

    fn process(&mut self, delta: f64) {
        // Remove first element of position_target_queue if current position is aprox equal to it
        if let Some(position_target) = self.position_target_queue.get(0) {
            let vd = *position_target - iso_to_cart(&self.base().get_position());
            let len = vd.length();
            if len <= self.speed * delta as f32 || len < real::CMP_EPSILON {
                self.position_target_queue.pop_front();
                self.update_animated_sprite();
            }
        }

        let mut is_moving = false;
        // Move to target if some exist
        if let Some(position_target) = self.position_target_queue.get(0) {
            is_moving = *position_target != iso_to_cart(&self.base().get_position());

            if is_moving {
                let velocity = iso_to_cart(&self.base().get_position())
                    .direction_to(*position_target)
                    * self.speed
                    * delta as f32;

                self.base_mut().move_and_collide(cart_to_iso(&velocity));
            }
        }

        // Slide to the exact server position to reduce desync
        if !is_moving && self.allow_server_position_sync {
            if let Some(server_position_current) = self.server_position_current {
                let position_current = iso_to_cart(&self.base().get_position());
                if position_current != server_position_current {
                    let new_position = position_current
                        .move_toward(server_position_current, self.speed * delta as f32);
                    self.base_mut().set_position(cart_to_iso(&new_position));
                }
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
    pub fn set_init_state(
        &mut self,
        entity_update: &UdpMsgDownGameEntityUpdate,
        animated_splite_2d_scene: Gd<PackedScene>,
    ) {
        self.animated_splite_2d_scene = Some(animated_splite_2d_scene);
        self.position_init = point_to_vector2(&entity_update.location_current.clone().unwrap());
        self.position_target_queue = VecDeque::from_iter(
            entity_update
                .location_target_queue
                .iter()
                .map(point_to_vector2)
                .collect::<Vec<_>>(),
        );

        self.speed = entity_update.velocity_speed;

        self.base_type = GameEntityBaseType::try_from(entity_update.object_type).unwrap();
        self.is_current_player = entity_update.is_self;

        // Draw health
        if entity_update.health_current > 0 {
            let mut health_label = Label::new_alloc();
            health_label.set_text(entity_update.health_current.to_string().into());
            health_label.set_horizontal_alignment(HorizontalAlignment::CENTER);
            if let Some(collider_dmg_in_rect) = &entity_update.collider_dmg_in_rect {
                health_label.set_position(Vector2 {
                    x: 0.0,
                    y: -(collider_dmg_in_rect.y / 2.0 + 40.0),
                });
            };
            health_label.set_z_index(3);
            self.health_label = Some(health_label.clone());
            self.base_mut().add_child(health_label.upcast());
        }

        if DEBUG {
            if let Some(collider_dmg_in_rect) = &entity_update.collider_dmg_in_rect {
                // Draw hitbox
                let a = Vector2::new(
                    -(collider_dmg_in_rect.x / 2.0),
                    -(collider_dmg_in_rect.y / 2.0),
                );
                let b = Vector2::new(
                    collider_dmg_in_rect.x / 2.0,
                    -(collider_dmg_in_rect.y / 2.0),
                );
                let c = Vector2::new(collider_dmg_in_rect.x / 2.0, collider_dmg_in_rect.y / 2.0);
                let d = Vector2::new(
                    -(collider_dmg_in_rect.x / 2.0),
                    collider_dmg_in_rect.y / 2.0,
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
        }

        if let Some(collider_mvt) = entity_update.collider_mvt.as_ref() {
            if let Some(rect) = collider_mvt.rect.as_ref() {
                let mut collision_shape_2d = CollisionShape2D::new_alloc();
                let mut shape = ConvexPolygonShape2D::new_gd();
                let mut packed_vector2_array = PackedVector2Array::new();
                let a = Vector2::new(-(rect.x / 2.0), -(rect.y / 2.0));
                let b = Vector2::new(rect.x / 2.0, -(rect.y / 2.0));
                let c = Vector2::new(rect.x / 2.0, rect.y / 2.0);
                let d = Vector2::new(-(rect.x / 2.0), rect.y / 2.0);
                packed_vector2_array.insert(0, cart_to_iso(&a));
                packed_vector2_array.insert(1, cart_to_iso(&b));
                packed_vector2_array.insert(2, cart_to_iso(&c));
                packed_vector2_array.insert(3, cart_to_iso(&d));
                shape.set_points(packed_vector2_array);
                collision_shape_2d.set_shape(shape.upcast());
                self.base_mut().add_child(collision_shape_2d.upcast());
            } else if !collider_mvt.poly.is_empty() && collider_mvt.reversed {
                // For reverse polygon, we have to create a polygon which cover all the map minus the shape of the given polygon
                // This case is only used for the shape of the global map atm
                let mut collision_shape_2d = CollisionPolygon2D::new_alloc();
                let mut packed_vector2_array = PackedVector2Array::new();
                for (index, point) in collider_mvt.poly.iter().enumerate() {
                    packed_vector2_array.insert(index, cart_to_iso(&point_to_vector2(point)));
                }
                let start_point = point_to_vector2(&collider_mvt.poly[0]);

                packed_vector2_array.insert(
                    packed_vector2_array.len(),
                    cart_to_iso(&point_to_vector2(&collider_mvt.poly[0])),
                );
                packed_vector2_array.insert(
                    packed_vector2_array.len(),
                    cart_to_iso(&Vector2::new(0.0, start_point.y)),
                );
                packed_vector2_array.insert(
                    packed_vector2_array.len(),
                    cart_to_iso(&Vector2::new(0.0, 0.0)),
                );
                packed_vector2_array.insert(
                    packed_vector2_array.len(),
                    cart_to_iso(&Vector2::new(100000.0, 0.0)),
                );
                packed_vector2_array.insert(
                    packed_vector2_array.len(),
                    cart_to_iso(&Vector2::new(100000.0, 100000.0)),
                );
                packed_vector2_array.insert(
                    packed_vector2_array.len(),
                    cart_to_iso(&Vector2::new(0.0, 100000.0)),
                );
                packed_vector2_array.insert(
                    packed_vector2_array.len(),
                    cart_to_iso(&Vector2::new(0.0, start_point.y)),
                );

                collision_shape_2d.set_build_mode(BuildMode::SEGMENTS);
                collision_shape_2d.set_polygon(packed_vector2_array);
                self.base_mut().add_child(collision_shape_2d.upcast());
            } else if !collider_mvt.poly.is_empty() {
                let mut collision_shape_2d = CollisionPolygon2D::new_alloc();
                let mut packed_vector2_array = PackedVector2Array::new();
                for (index, point) in collider_mvt.poly.iter().enumerate() {
                    packed_vector2_array.insert(index, cart_to_iso(&point_to_vector2(point)));
                }
                collision_shape_2d.set_polygon(packed_vector2_array);
                self.base_mut().add_child(collision_shape_2d.upcast());
            }
        }
    }

    pub fn update_from_server(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        self.speed = entity_update.velocity_speed;
        self.position_target_queue = VecDeque::from_iter(
            entity_update
                .location_target_queue
                .iter()
                .map(point_to_vector2)
                .collect::<Vec<_>>(),
        );

        self.allow_server_position_sync = self.position_target_queue.is_empty();

        if let Some(cast) = &entity_update.cast {
            self.casting_target = Some(point_to_vector2(&cast.target.clone().unwrap()));
            self.is_casting = true;
        } else {
            self.casting_target = None;
            self.is_casting = false;
        }

        self.update_animated_sprite();

        let new_position_current =
            point_to_vector2(&entity_update.location_current.clone().unwrap());
        self.server_position_current = Some(new_position_current);
        if iso_to_cart(&self.base().get_position()).distance_to(new_position_current) > 300.0 {
            self.base_mut()
                .set_position(cart_to_iso(&new_position_current));
        }

        if let Some(health_label) = &mut self.health_label {
            health_label.set_text(entity_update.health_current.to_string().into());
            if entity_update.health_current == 0 {
                self.base_mut().set_visible(false);
                self.is_dead = true;
            } else {
                self.base_mut().set_visible(true);
                self.is_dead = false;
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
