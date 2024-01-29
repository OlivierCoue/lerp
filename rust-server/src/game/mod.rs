mod area_gen;
mod bundles;
mod components;
mod events;
mod pathfinder;
mod resources;
mod systems;

pub mod internal_message;
pub mod player;

use bevy_ecs::prelude::*;
use bson::oid::ObjectId;
use godot::builtin::Vector2;
use rust_common::helper::{get_timestamp_nanos, vector2_to_point};
use rust_common::math::get_point_from_points_and_distance;
use rust_common::proto::udp_down::{
    UdpMsgDown, UdpMsgDownAreaInit, UdpMsgDownGameEntityRemoved, UdpMsgDownGameEntityUpdate,
    UdpMsgDownType, UdpMsgDownWrapper,
};
use rust_common::proto::udp_up::{UdpMsgUp, UdpMsgUpType};
use tokio::sync::mpsc;

use std::cmp::max;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::game::{
    bundles::prelude::*, components::prelude::*, events::prelude::*, player::Player,
    resources::prelude::*, systems::prelude::*,
};

use self::area_gen::generate_area;
use self::internal_message::InboundAreaMessage;

const TICK_RATE_MILLIS: u128 = 30;
const TICK_RATE_NANOS: u128 = TICK_RATE_MILLIS * 1000000;
const UPDATE_USERS_EVERY_N_TICK: u32 = 1;
const GAME_TIME_TICK_DURATION_MILLIS: u32 = 30;

pub struct Game {
    players: HashMap<ObjectId, Player>,
    peer_id_player_id_map: HashMap<u16, ObjectId>,
    tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
    paused: bool,
    internal_messages_in: Arc<Mutex<VecDeque<InboundAreaMessage>>>,
    received_udp_messages: Arc<Mutex<VecDeque<(u16, UdpMsgUp)>>>,
    ecs_world: World,
    ecs_world_schedule: Schedule,
}

impl Game {
    pub fn new(
        tx_udp_sender: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
        internal_messages_in: Arc<Mutex<VecDeque<InboundAreaMessage>>>,
        received_udp_messages: Arc<Mutex<VecDeque<(u16, UdpMsgUp)>>>,
    ) -> Game {
        let area_gen = generate_area();
        let mut world = World::new();

        world.init_resource::<Events<UpdateVelocityTarget>>();
        world.init_resource::<Events<UpdateVelocityTargetWithPathFinder>>();
        world.init_resource::<Events<AddVelocityTarget>>();
        world.init_resource::<Events<UpdatePositionCurrent>>();
        world.init_resource::<Events<CastSpell>>();
        world.init_resource::<Events<VelocityReachedTarget>>();
        world.insert_resource(Time::new());
        world.insert_resource(EnemiesState::new());
        let area_config = AreaConfig {
            area_width: area_gen.width as f32 * 60.0,
            area_height: area_gen.height as f32 * 60.0,
            walkable_x: area_gen.walkable_x,
            walkable_y: area_gen.walkable_y,
        };
        world.insert_resource(PathfinderState::new(&area_config));
        world.insert_resource(area_config);

        let mut world_schedule = Schedule::default();

        world_schedule
            .add_systems(despawn_if_velocity_at_target.before(inc_revision_updated_component));

        world_schedule.add_systems(
            update_pathfinder_state
                .before(movement)
                .before(inc_revision_updated_component),
        );
        world_schedule.add_systems(
            on_update_velocity_target_with_pathfinder
                .before(inc_revision_updated_component)
                .after(movement),
        );
        world_schedule.add_systems(movement.before(inc_revision_updated_component));
        world_schedule.add_systems(damage_on_hit.before(inc_revision_updated_component));
        world_schedule.add_systems(create_casted_spells.before(inc_revision_updated_component));

        world_schedule.add_systems(enemies_spawner.before(inc_revision_updated_component));
        world_schedule.add_systems(enemies_ai.before(inc_revision_updated_component));

        world_schedule.add_systems(
            on_update_position_current
                .before(inc_revision_updated_component)
                .after(movement),
        );
        world_schedule.add_systems(
            on_update_velocity_target
                .before(inc_revision_updated_component)
                .after(movement),
        );
        world_schedule.add_systems(
            on_add_velocity_target
                .before(inc_revision_updated_component)
                .after(movement),
        );
        world_schedule.add_systems(on_cast_spell.before(inc_revision_updated_component));
        world_schedule.add_systems(
            on_frozen_orb_velocity_reached_target.before(inc_revision_updated_component),
        );

        world_schedule
            .add_systems(inc_revision_updated_component.before(inc_revision_removed_component));
        world_schedule.add_systems(inc_revision_removed_component);

        // Add Walls
        let wall: WallBundle = WallBundle::new(
            Vector2::new(1050.0, 1080.0),
            Vector2 {
                x: 1020.0,
                y: 120.0,
            },
        );
        world.spawn(wall);
        let wall2: WallBundle =
            WallBundle::new(Vector2::new(510.0, 510.0), Vector2 { x: 300.0, y: 300.0 });
        world.spawn(wall2);
        let wall3: WallBundle =
            WallBundle::new(Vector2::new(1050.0, 510.0), Vector2 { x: 300.0, y: 300.0 });
        world.spawn(wall3);

        Game {
            players: HashMap::new(),
            peer_id_player_id_map: HashMap::new(),
            tx_udp_sender,
            paused: false,
            internal_messages_in,
            received_udp_messages,
            ecs_world: world,
            ecs_world_schedule: world_schedule,
        }
    }

    fn add_player(
        &mut self,
        user_id: ObjectId,
        peer_id: u16,
        udp_tunnel: mpsc::Sender<(u16, UdpMsgDownWrapper)>,
    ) {
        let player_entity = self.ecs_world.spawn(PlayerBundle::new()).id();

        let area_config = self.ecs_world.get_resource::<AreaConfig>().unwrap();
        let new_player = Player::new(user_id, peer_id, udp_tunnel, player_entity);
        new_player.send_message(UdpMsgDownWrapper {
            messages: vec![UdpMsgDown {
                _type: UdpMsgDownType::AREA_INIT.into(),
                area_init: Some(UdpMsgDownAreaInit {
                    width: area_config.area_width,
                    height: area_config.area_height,
                    walkable_x: area_config.walkable_x.to_vec(),
                    walkable_y: area_config.walkable_y.to_vec(),
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            }],
            ..Default::default()
        });
        self.players.insert(user_id, new_player);
        self.peer_id_player_id_map.insert(peer_id, user_id);

        println!("Created new Player({})", user_id);
    }

    fn delete_player(&mut self, player_id: ObjectId) {
        if let Some(removed_player) = self.players.remove(&player_id) {
            self.ecs_world.despawn(removed_player.player_entity);
        }
    }

    pub fn start(&mut self) {
        let mut tick_count = 0;
        loop {
            let started_at: u128 = get_timestamp_nanos();
            tick_count += 1;
            self.tick(tick_count == UPDATE_USERS_EVERY_N_TICK);
            if tick_count == UPDATE_USERS_EVERY_N_TICK {
                tick_count = 0;
            }

            let tick_duration = get_timestamp_nanos() - started_at;
            if tick_duration < TICK_RATE_NANOS {
                let time_to_wait = max(TICK_RATE_NANOS - tick_duration, 0);
                // tokio::time::sleep(Duration::from_nanos(time_to_wait as u64)).await;
                spin_sleep::sleep(Duration::from_nanos(time_to_wait as u64));
            } else {
                println!(
                    "WARNING tick_duration ({}) in more than TICK_RATE_NANOS ({})",
                    tick_duration, TICK_RATE_NANOS
                );
            }
            self.ecs_world.get_resource_mut::<Time>().unwrap().delta =
                (get_timestamp_nanos() - started_at) as f32 / 1000000000.0;
        }
    }

    fn tick(&mut self, _: bool) {
        let mut internal_messages = VecDeque::new();
        if let Ok(mut received_udp_messages) = self.internal_messages_in.lock() {
            while let Some(message) = received_udp_messages.pop_front() {
                internal_messages.push_back(message);
            }
        } else {
            println!("Failed to get internal_messages_in lock");
        }

        while let Some(message) = internal_messages.pop_front() {
            self.handle_internal_message(message);
        }

        let mut udp_messages = VecDeque::new();
        if let Ok(mut received_udp_messages) = self.received_udp_messages.lock() {
            while let Some((from_udp_peer_id, udp_msg_up)) = received_udp_messages.pop_front() {
                udp_messages.push_back((from_udp_peer_id, udp_msg_up));
            }
        } else {
            println!("Failed to get received_udp_messages lock");
        }

        while let Some((from_udp_peer_id, udp_msg_up)) = udp_messages.pop_front() {
            self.handle_upd_msg_up(from_udp_peer_id, udp_msg_up);
        }

        if self.paused {
            return;
        }

        let mut entities_to_despawn = Vec::new();
        for (entity_id, game_entity) in self
            .ecs_world
            .query::<(Entity, &GameEntity)>()
            .iter(&self.ecs_world)
        {
            if game_entity.pending_despwan {
                entities_to_despawn.push(entity_id);
            }
        }
        for entity_to_despawn in entities_to_despawn {
            self.ecs_world.despawn(entity_to_despawn);
        }

        self.ecs_world_schedule.run(&mut self.ecs_world);
        self.ecs_world.clear_trackers();
        self.ecs_world
            .get_resource_mut::<Time>()
            .unwrap()
            .inc_current_millis();

        let mut player_udp_msg_down_wrapper_map = HashMap::new();

        for (entity_id, game_entity) in self
            .ecs_world
            .query::<(Entity, &GameEntity)>()
            .iter(&self.ecs_world)
        {
            let entity_ref = self.ecs_world.entity(entity_id);

            for player_mut in self.players.values_mut() {
                let opt_last_revision = player_mut
                    .entity_id_revision_map
                    .insert(game_entity.id, game_entity.revision);
                let require_update = match opt_last_revision {
                    None => true,
                    Some(last_revision) => last_revision < game_entity.revision,
                };

                if require_update {
                    let udp_msg_down_wrapper = player_udp_msg_down_wrapper_map
                        .entry(player_mut.id)
                        .or_insert(UdpMsgDownWrapper {
                            messages: Vec::new(),
                            ..Default::default()
                        });

                    if game_entity.pending_despwan {
                        udp_msg_down_wrapper.messages.push(UdpMsgDown {
                            _type: UdpMsgDownType::GAME_ENTITY_REMOVED.into(),
                            game_entity_removed: (Some(UdpMsgDownGameEntityRemoved {
                                id: game_entity.id,
                                ..Default::default()
                            }))
                            .into(),
                            ..Default::default()
                        })
                    } else {
                        let location_current = entity_ref
                            .get::<Position>()
                            .map(|position| vector2_to_point(&position.current));

                        let (location_target_queue, velocity_speed) =
                            match entity_ref.get::<Velocity>() {
                                Some(velocity) => (
                                    Some(
                                        velocity
                                            .get_target_queue()
                                            .into_iter()
                                            .map(|x| vector2_to_point(&x))
                                            .collect::<Vec<_>>(),
                                    ),
                                    Some(velocity.get_speed()),
                                ),
                                None => (None, None),
                            };
                        let collider_dmg_in_rect = entity_ref
                            .get::<ColliderDmgIn>()
                            .map(|x| vector2_to_point(&x.rect));
                        let collider_mvt_rect = entity_ref
                            .get::<ColliderMvt>()
                            .map(|x| vector2_to_point(&x.rect));
                        let health_current = entity_ref
                            .get::<Health>()
                            .map(|health| health.get_current());
                        let cast = entity_ref.get::<Cast>().map(|cast| cast.to_proto());

                        udp_msg_down_wrapper.messages.push(UdpMsgDown {
                            _type: UdpMsgDownType::GAME_ENTITY_UPDATE.into(),
                            game_entity_update: (Some(UdpMsgDownGameEntityUpdate {
                                id: game_entity.id,
                                object_type: game_entity._type.into(),
                                location_current: location_current.into(),
                                location_target_queue: match location_target_queue {
                                    Some(x) => x,
                                    None => Vec::new(),
                                },
                                collider_dmg_in_rect: collider_dmg_in_rect.into(),
                                collider_mvt_rect: collider_mvt_rect.into(),
                                velocity_speed,
                                health_current,
                                is_self: entity_id == player_mut.player_entity,
                                cast: cast.into(),
                                ..Default::default()
                            }))
                            .into(),
                            ..Default::default()
                        })
                    }
                }
            }
        }

        for player in self.players.values() {
            if let Some(udp_msg_down_wrapper) = player_udp_msg_down_wrapper_map.remove(&player.id) {
                if !udp_msg_down_wrapper.messages.is_empty() {
                    player.send_message(udp_msg_down_wrapper);
                }
            }
        }
    }

    fn handle_internal_message(&mut self, message: InboundAreaMessage) {
        match message {
            InboundAreaMessage::PlayerInit(payload) => self.add_player(
                payload.user_id,
                payload.udp_peer_id,
                self.tx_udp_sender.clone(),
            ),
        }
    }

    fn handle_upd_msg_up(&mut self, from_udp_peer_id: u16, udp_msg_up: UdpMsgUp) {
        let Some(from_player_id) = self.peer_id_player_id_map.get(&from_udp_peer_id) else {
            return;
        };

        let Some(player) = self.players.get_mut(from_player_id) else {
            return;
        };

        match udp_msg_up._type.unwrap() {
            UdpMsgUpType::GAME_PAUSE => self.paused = !self.paused,
            UdpMsgUpType::PLAYER_MOVE => {
                if let Some(ok_coord) = &udp_msg_up.player_move.0 {
                    let area_config = self.ecs_world.get_resource::<AreaConfig>().unwrap();
                    self.ecs_world
                        .send_event(UpdateVelocityTargetWithPathFinder {
                            entity: player.player_entity,
                            target: world_bounded_vector2(
                                area_config,
                                Vector2::new(ok_coord.x, ok_coord.y),
                            ),
                        });
                }
            }
            UdpMsgUpType::PLAYER_TELEPORT => {
                if let Some(ok_coord) = &udp_msg_up.player_teleport.0 {
                    let area_config = self.ecs_world.get_resource::<AreaConfig>().unwrap();
                    self.ecs_world.send_event(UpdatePositionCurrent {
                        entity: player.player_entity,
                        current: world_bounded_vector2(
                            area_config,
                            Vector2::new(ok_coord.x, ok_coord.y),
                        ),
                        force_update_velocity_target: true,
                    });
                }
            }
            UdpMsgUpType::PLAYER_THROW_FROZEN_ORB => {
                if let Some(ok_coord) = &udp_msg_up.player_throw_frozen_orb.0 {
                    let player_position = self
                        .ecs_world
                        .get::<Position>(player.player_entity)
                        .unwrap();
                    let player_team = self.ecs_world.get::<Team>(player.player_entity).unwrap();
                    self.ecs_world.send_event(CastSpell {
                        from_entity: player.player_entity,
                        spell: Spell::FrozenOrb(
                            player.player_entity,
                            player_position.current,
                            get_point_from_points_and_distance(
                                player_position.current,
                                Vector2::new(ok_coord.x, ok_coord.y),
                                600.0,
                            ),
                            *player_team,
                        ),
                    });
                }
            }
            UdpMsgUpType::PLAYER_THROW_PROJECTILE => {
                if let Some(ok_coord) = &udp_msg_up.player_throw_projectile.0 {
                    let player_position = self
                        .ecs_world
                        .get::<Position>(player.player_entity)
                        .unwrap();
                    let player_team = self.ecs_world.get::<Team>(player.player_entity).unwrap();
                    self.ecs_world.send_event(CastSpell {
                        from_entity: player.player_entity,
                        spell: Spell::Projectile(
                            player.player_entity,
                            player_position.current,
                            get_point_from_points_and_distance(
                                player_position.current,
                                Vector2::new(ok_coord.x, ok_coord.y),
                                600.0,
                            ),
                            *player_team,
                        ),
                    });
                }
            }
            UdpMsgUpType::PLAYER_MELEE_ATTACK => {
                if let Some(ok_coord) = &udp_msg_up.player_throw_frozen_orb.0 {
                    let player_position = self
                        .ecs_world
                        .get::<Position>(player.player_entity)
                        .unwrap();
                    let player_team = self.ecs_world.get::<Team>(player.player_entity).unwrap();
                    self.ecs_world.send_event(CastSpell {
                        from_entity: player.player_entity,
                        spell: Spell::MeleeAttack(
                            player.player_entity,
                            get_point_from_points_and_distance(
                                player_position.current,
                                Vector2::new(ok_coord.x, ok_coord.y),
                                40.0,
                            ),
                            *player_team,
                        ),
                    });
                }
            }
            UdpMsgUpType::SETTINGS_TOGGLE_ENEMIES => {
                let mut enemmies_state = self.ecs_world.get_resource_mut::<EnemiesState>().unwrap();
                enemmies_state.toggle_enable();
            }
            _ => println!("Not handled event"),
        }
    }
}
