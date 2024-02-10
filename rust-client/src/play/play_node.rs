use std::{collections::HashMap, rc::Rc};

use godot::{
    engine::{ISprite2D, InputEvent, TileMap},
    prelude::*,
};
use rust_common::proto::{
    common::Point,
    udp_down::{
        UdpMsgDownAreaInit, UdpMsgDownGameEntityRemoved, UdpMsgDownGameEntityUpdate, UdpMsgDownType,
    },
    udp_up::{MsgUp, MsgUpType, MsgUpUserJoinWorldInstance, MsgUpWrapper},
};

use crate::{
    network::prelude::*,
    root::{Root, Scenes, DEBUG, PATH_NETWORK, PATH_ROOT},
    utils::iso_to_cart,
};

use super::{entity::GameEntity, play_node_debug::PlayNodeDebug, prelude::GameServerEntity};

pub const SEND_INPUT_TICK_SEC: f64 = 0.1;

#[derive(GodotClass)]
#[class(no_init, base=Node2D)]
pub struct PlayNode {
    base: Base<Node2D>,

    root: OnReady<Gd<Root>>,
    network: OnReady<Gd<NetworkManager>>,
    entities: HashMap<u32, Gd<GameEntity>>,
    server_entities: HashMap<u32, Gd<GameServerEntity>>,
    time_since_last_input_sent: f64,
    world_instance_id: String,
    animated_sprite_2d_scene: OnReady<Gd<PackedScene>>,
}

#[godot_api]
impl INode2D for PlayNode {
    fn ready(&mut self) {
        self.root.init(self.base().get_node_as::<Root>(PATH_ROOT));
        self.network
            .init(self.base().get_node_as::<NetworkManager>(PATH_NETWORK));
        self.base_mut().set_y_sort_enabled(true);

        if DEBUG {
            let play_node_debug = Gd::<PlayNodeDebug>::from_init_fn(PlayNodeDebug::init);
            self.base_mut().add_child(play_node_debug.upcast());
        }

        self.animated_sprite_2d_scene
            .init(load::<PackedScene>("res://animated_sprite_2d/warrior.tscn"));

        self.network.bind().send_udp(MsgUpWrapper {
            messages: vec![MsgUp {
                _type: MsgUpType::USER_JOIN_WOLD_INSTANCE.into(),
                user_join_world_instance: Some(MsgUpUserJoinWorldInstance {
                    id: self.world_instance_id.clone(),
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            }],
            ..Default::default()
        })
    }

    fn process(&mut self, delta: f64) {
        let rx_enet_receiver = Rc::clone(&self.network.bind().rx_udp_receiver);
        while let Ok(udp_msg_down_wrapper) = &rx_enet_receiver.try_recv() {
            for udp_msg_down in &udp_msg_down_wrapper.messages {
                match udp_msg_down._type.unwrap() {
                    UdpMsgDownType::AREA_INIT => {
                        if let Some(area_config) = &udp_msg_down.area_init.0 {
                            self.init_tile_map(area_config);
                        }
                    }
                    UdpMsgDownType::GAME_ENTITY_UPDATE => {
                        if let Some(entity_update) = &udp_msg_down.game_entity_update.0 {
                            self.update_entity(entity_update);
                        }
                    }
                    UdpMsgDownType::GAME_ENTITY_REMOVED => {
                        if let Some(entity_removed) = &udp_msg_down.game_entity_removed.0 {
                            self.remove_entity(entity_removed);
                        }
                    }
                    UdpMsgDownType::USER_LEAVE_WORLD_INSTANCE_SUCCESS => {
                        self.root.bind_mut().change_scene(Scenes::Lobby);
                    }
                    _ => {}
                }
            }
        }

        self.time_since_last_input_sent += delta;
        if self.time_since_last_input_sent >= SEND_INPUT_TICK_SEC {
            self.time_since_last_input_sent = 0.0;
            let mut actions = Vec::new();

            if Input::singleton().is_action_pressed("key_e".into()) {
                let mouse_position = iso_to_cart(&self.base().get_global_mouse_position());
                self.base_mut()
                    .emit_signal("player_throw_fireball_start".into(), &[]);
                actions.push(MsgUp {
                    _type: MsgUpType::PLAYER_THROW_FROZEN_ORB.into(),
                    player_throw_frozen_orb: Some(Point {
                        x: mouse_position.x,
                        y: mouse_position.y,
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                });
            }

            if Input::singleton().is_action_pressed("right_mouse_button".into()) {
                let mouse_position = iso_to_cart(&self.base().get_global_mouse_position());
                self.base_mut()
                    .emit_signal("player_throw_fireball_start".into(), &[]);
                actions.push(MsgUp {
                    _type: MsgUpType::PLAYER_MELEE_ATTACK.into(),
                    player_throw_frozen_orb: Some(Point {
                        x: mouse_position.x,
                        y: mouse_position.y,
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                });
            }

            if actions.is_empty()
                && Input::singleton().is_action_pressed("left_mouse_button".into())
            {
                let mouse_position = iso_to_cart(&self.base().get_global_mouse_position());
                self.base_mut()
                    .emit_signal("player_move_start".into(), &[mouse_position.to_variant()]);
                actions.push(MsgUp {
                    _type: MsgUpType::PLAYER_MOVE.into(),
                    player_move: Some(Point {
                        x: mouse_position.x,
                        y: mouse_position.y,
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                });
            }

            if !actions.is_empty() {
                self.network.bind().send_udp(MsgUpWrapper {
                    messages: actions,
                    ..Default::default()
                })
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("left_mouse_button".into()) {
            godot_print!("Left button pressed");
            let mouse_position = iso_to_cart(&self.base().get_global_mouse_position());
            self.base_mut()
                .emit_signal("player_move_start".into(), &[mouse_position.to_variant()]);

            self.network.bind().send_udp(MsgUpWrapper {
                messages: vec![MsgUp {
                    _type: MsgUpType::PLAYER_MOVE.into(),
                    player_move: Some(Point {
                        x: mouse_position.x,
                        y: mouse_position.y,
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                }],
                ..Default::default()
            })
        } else if event.is_action_pressed("key_e".into()) {
            godot_print!("Key E pressed");
            let mouse_position = iso_to_cart(&self.base().get_global_mouse_position());
            self.base_mut()
                .emit_signal("player_throw_fireball_start".into(), &[]);

            self.network.bind().send_udp(MsgUpWrapper {
                messages: vec![MsgUp {
                    _type: MsgUpType::PLAYER_THROW_FROZEN_ORB.into(),
                    player_throw_frozen_orb: Some(Point {
                        x: mouse_position.x,
                        y: mouse_position.y,
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                }],
                ..Default::default()
            })
        } else if event.is_action_pressed("key_r".into()) {
            godot_print!("Key R pressed");
            let mouse_position = iso_to_cart(&self.base().get_global_mouse_position());

            self.network.bind().send_udp(MsgUpWrapper {
                messages: vec![MsgUp {
                    _type: MsgUpType::PLAYER_TELEPORT.into(),
                    player_teleport: Some(Point {
                        x: mouse_position.x,
                        y: mouse_position.y,
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                }],
                ..Default::default()
            })
        } else if event.is_action_pressed("key_n".into()) {
            godot_print!("Key N pressed");

            self.network.bind().send_udp(MsgUpWrapper {
                messages: vec![MsgUp {
                    _type: MsgUpType::SETTINGS_TOGGLE_ENEMIES.into(),
                    ..Default::default()
                }],
                ..Default::default()
            })
        } else if event.is_action_pressed("key_escape".into()) {
            self.network.bind().send_udp(MsgUpWrapper {
                messages: vec![MsgUp {
                    _type: MsgUpType::USER_LEAVE_WORLD_INSTANCE.into(),
                    ..Default::default()
                }],
                ..Default::default()
            })
        }
    }
}

#[godot_api]
impl PlayNode {
    #[signal]
    fn player_move_start();
    #[signal]
    fn player_throw_fireball_start();
    #[func]
    fn add_child_entity(&mut self, v: Variant) {
        let entity = v.to::<Gd<GameEntity>>();
        self.base_mut().add_child(entity.upcast());
    }
}

impl PlayNode {
    pub fn init(base: Base<Node2D>, world_instance_id: String) -> Self {
        Self {
            base,
            root: OnReady::manual(),
            network: OnReady::manual(),
            entities: HashMap::new(),
            server_entities: HashMap::new(),
            time_since_last_input_sent: 0.0,
            world_instance_id,
            animated_sprite_2d_scene: OnReady::manual(),
        }
    }

    pub fn init_tile_map(&mut self, area_init: &UdpMsgDownAreaInit) {
        let tile_map_scene = load::<PackedScene>("res://tile_map.tscn");
        let mut tile_map = tile_map_scene.instantiate_as::<TileMap>();
        tile_map.set_scale(Vector2::new(5.0, 5.0));
        tile_map.set_position(Vector2::new(0.0, 0.0));

        godot_print!("shape count: {}", area_init.oob_polygons.len());

        godot_print!("init_tile_map: {}:{}", area_init.width, area_init.height);

        for x in 0..(area_init.width as usize / 60) {
            for y in 0..(area_init.height as usize / 60) {
                let cell = tile_map.set_cell_ex(0, Vector2i::new(x as i32 - 1, y as i32));
                cell.atlas_coords(Vector2i::new(10, 2)).source_id(0).done();
            }
        }

        for i in 0..area_init.walkable_x.len() {
            let cell = tile_map.set_cell_ex(
                0,
                Vector2i::new(
                    area_init.walkable_x[i] as i32 - 1,
                    area_init.walkable_y[i] as i32,
                ),
            );
            cell.atlas_coords(Vector2i::new(0, 0)).source_id(0).done();
        }
        self.base_mut().add_child(tile_map.upcast());
    }

    pub fn update_entity(&mut self, entity_update: &UdpMsgDownGameEntityUpdate) {
        if let Some(entity) = self.entities.get_mut(&entity_update.id) {
            entity.bind_mut().update_from_server(entity_update);
        } else {
            let mut entity = Gd::<GameEntity>::from_init_fn(GameEntity::init);
            entity
                .bind_mut()
                .set_init_state(entity_update, self.animated_sprite_2d_scene.clone());
            self.entities.insert(entity_update.id, entity.clone());
            self.base_mut()
                .call_deferred_thread_group("add_child_entity".into(), &[entity.to_variant()]);
        }
        if DEBUG {
            if let Some(entity) = self.server_entities.get_mut(&entity_update.id) {
                entity.bind_mut().update_from_server(entity_update);
            } else {
                let mut entity = Gd::<GameServerEntity>::from_init_fn(GameServerEntity::init);
                entity.bind_mut().set_init_state(entity_update);
                self.server_entities
                    .insert(entity_update.id, entity.clone());
                self.base_mut().add_child(entity.upcast());
            }
        }
    }

    pub fn remove_entity(&mut self, entity_removed: &UdpMsgDownGameEntityRemoved) {
        if let Some(mut entity) = self.entities.remove(&entity_removed.id) {
            entity.queue_free()
        }
        if DEBUG {
            if let Some(mut entity) = self.server_entities.remove(&entity_removed.id) {
                entity.queue_free()
            }
        }
    }
}
