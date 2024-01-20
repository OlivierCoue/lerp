mod bundles;
mod components;
mod events;
pub mod pathfinder;
mod resources;
mod systems;

pub mod user;

use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use rust_common::helper::{get_timestamp_nanos, vector2_to_point};
use rust_common::math::get_point_from_points_and_distance;
use rust_common::proto::udp_down::{
    UdpMsgDown, UdpMsgDownGameEntityRemoved, UdpMsgDownGameEntityUpdate, UdpMsgDownType,
    UdpMsgDownWrapper,
};
use rust_common::proto::udp_up::{UdpMsgUpType, UdpMsgUpWrapper};

use crate::utils::inc_game_time_millis;
use std::cmp::max;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;
use std::{collections::HashMap, sync::mpsc::Sender};

use crate::game::{
    bundles::prelude::*, components::prelude::*, events::prelude::*, resources::prelude::*,
    systems::prelude::*, user::User,
};

const TICK_RATE_MILLIS: u128 = 30;
const TICK_RATE_NANOS: u128 = TICK_RATE_MILLIS * 1000000;
const UPDATE_USERS_EVERY_N_TICK: u32 = 1;
const GAME_TIME_TICK_DURATION_MILLIS: u32 = 30;

pub struct Game<'a> {
    users: HashMap<u32, User<'a>>,
    users_curr_id: u32,
    peer_id_user_id_map: HashMap<u16, u32>,
    tx_enet_sender: &'a Sender<(u16, UdpMsgDownWrapper)>,
    paused: bool,
    clients_msg: &'a Mutex<VecDeque<(u16, UdpMsgUpWrapper)>>,
    world: World,
    world_schedule: Schedule,
}

impl<'a> Game<'a> {
    pub fn new(
        tx_enet_sender: &'a Sender<(u16, UdpMsgDownWrapper)>,
        clients_msg: &'a Mutex<VecDeque<(u16, UdpMsgUpWrapper)>>,
    ) -> Game<'a> {
        let mut world = World::new();

        world.init_resource::<Events<UpdateVelocityTarget>>();
        world.init_resource::<Events<UpdateVelocityTargetWithPathFinder>>();
        world.init_resource::<Events<AddVelocityTarget>>();
        world.init_resource::<Events<UpdatePositionCurrent>>();
        world.init_resource::<Events<SpawnProjectile>>();
        world.init_resource::<Events<SpawnFrozenOrb>>();
        world.init_resource::<Events<VelocityReachedTarget>>();
        world.insert_resource(Time::new());
        world.insert_resource(EnemiesState::new());
        world.insert_resource(PathfinderState::new());

        let mut world_schedule = Schedule::default();

        world_schedule
            .add_systems(despawn_if_velocity_at_target.before(increase_game_entity_revision));

        world_schedule.add_systems(
            update_pathfinder_state
                .before(movement)
                .before(increase_game_entity_revision),
        );
        world_schedule.add_systems(
            on_update_velocity_target_with_pathfinder
                .before(increase_game_entity_revision)
                .after(movement),
        );
        world_schedule.add_systems(movement.before(increase_game_entity_revision));
        world_schedule.add_systems(damage_on_hit.before(increase_game_entity_revision));

        world_schedule.add_systems(enemies_spawner.before(increase_game_entity_revision));
        world_schedule.add_systems(enemies_ai.before(increase_game_entity_revision));

        world_schedule.add_systems(
            on_update_position_current
                .before(increase_game_entity_revision)
                .after(movement),
        );
        world_schedule.add_systems(
            on_update_velocity_target
                .before(increase_game_entity_revision)
                .after(movement),
        );
        world_schedule.add_systems(
            on_add_velocity_target
                .before(increase_game_entity_revision)
                .after(movement),
        );
        world_schedule.add_systems(on_spawn_projectile.before(increase_game_entity_revision));
        world_schedule.add_systems(on_spawn_frozen_orb.before(increase_game_entity_revision));
        world_schedule.add_systems(
            on_frozen_orb_velocity_reached_target.before(increase_game_entity_revision),
        );

        world_schedule.add_systems(increase_game_entity_revision);

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
            users: HashMap::new(),
            peer_id_user_id_map: HashMap::new(),
            users_curr_id: 0,
            tx_enet_sender,
            paused: false,
            clients_msg,
            world,
            world_schedule,
        }
    }

    fn add_user(&mut self, peer_id: u16, udp_tunnel: &'a Sender<(u16, UdpMsgDownWrapper)>) {
        self.users_curr_id += 1;

        let player = PlayerBundle::new();
        let player_entity = self.world.spawn(player).id();

        let new_user = User::new(self.users_curr_id, peer_id, udp_tunnel, player_entity);
        self.users.insert(self.users_curr_id, new_user);
        self.peer_id_user_id_map.insert(peer_id, self.users_curr_id);

        println!("Created new Player({})", self.users_curr_id);
    }

    fn delete_user(&mut self, user_id: u32) {
        if let Some(removed_user) = self.users.remove(&user_id) {
            self.world.despawn(removed_user.player_entity);
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
                spin_sleep::sleep(Duration::from_nanos(time_to_wait as u64));
            } else {
                println!(
                    "WARNING tick_duration ({}) in more than TICK_RATE_NANOS ({})",
                    tick_duration, TICK_RATE_NANOS
                );
            }
            self.world.get_resource_mut::<Time>().unwrap().delta =
                (get_timestamp_nanos() - started_at) as f32 / 1000000000.0;
        }
    }

    pub fn tick(&mut self, _: bool) {
        let mut users_to_delete_ids = Vec::new();
        for (user_id, user) in self.users.iter_mut() {
            if user.should_be_deleted() {
                users_to_delete_ids.push(*user_id);
            }
        }

        for user_id in users_to_delete_ids {
            self.delete_user(user_id);
        }

        if let Ok(mut clients_messages) = self.clients_msg.lock() {
            while let Some((from_enet_peer_id, udp_msg_up)) = clients_messages.pop_front() {
                self.handle_upd_msg_up(&from_enet_peer_id, &udp_msg_up);
            }
        } else {
            println!("Failed to get clients_messages lock");
        }

        if self.paused {
            return;
        }

        let mut entities_to_despawn = Vec::new();
        for (entity_id, game_entity) in self
            .world
            .query::<(Entity, &GameEntity)>()
            .iter(&self.world)
        {
            if game_entity.pending_despwan {
                entities_to_despawn.push(entity_id);
            }
        }
        for entity_to_despawn in entities_to_despawn {
            self.world.despawn(entity_to_despawn);
        }

        self.world_schedule.run(&mut self.world);
        inc_game_time_millis(GAME_TIME_TICK_DURATION_MILLIS);

        let mut player_udp_msg_down_wrapper_map = HashMap::new();

        for (entity_id, game_entity) in self
            .world
            .query::<(Entity, &GameEntity)>()
            .iter(&self.world)
        {
            let entity_ref = self.world.entity(entity_id);

            for user_mut in self.users.values_mut() {
                let opt_last_revision = user_mut
                    .entity_id_revision_map
                    .insert(game_entity.id, game_entity.revision);
                let require_update = match opt_last_revision {
                    None => true,
                    Some(last_revision) => last_revision < game_entity.revision,
                };

                if require_update {
                    let udp_msg_down_wrapper = player_udp_msg_down_wrapper_map
                        .entry(user_mut.id)
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
                                is_self: entity_id == user_mut.player_entity,
                                ..Default::default()
                            }))
                            .into(),
                            ..Default::default()
                        })
                    }
                }
            }
        }

        for user in self.users.values() {
            if let Some(udp_msg_down_wrapper) = player_udp_msg_down_wrapper_map.remove(&user.id) {
                if !udp_msg_down_wrapper.messages.is_empty() {
                    user.send_message(udp_msg_down_wrapper)
                }
            }
        }
    }

    pub fn handle_upd_msg_up(
        &mut self,
        from_enet_peer_id: &u16,
        udp_msg_up_wrapper: &UdpMsgUpWrapper,
    ) {
        let from_actor_id = match self.peer_id_user_id_map.get(from_enet_peer_id) {
            None => {
                println!("New message from unknown addr {from_enet_peer_id}");
                999
            }
            Some(player_id) => *player_id,
        };

        for udp_msg_up in udp_msg_up_wrapper.messages.iter() {
            let user = self.users.get_mut(&from_actor_id);

            match udp_msg_up._type.unwrap() {
                UdpMsgUpType::GAME_PAUSE => self.paused = !self.paused,
                UdpMsgUpType::PLAYER_INIT => {
                    if user.is_none() {
                        self.add_user(*from_enet_peer_id, self.tx_enet_sender)
                    };
                }
                UdpMsgUpType::PLAYER_MOVE => {
                    if let Some(ok_user) = user {
                        if let Some(ok_coord) = &udp_msg_up.player_move.0 {
                            self.world.send_event(UpdateVelocityTargetWithPathFinder {
                                entity: ok_user.player_entity,
                                target: world_bounded_vector2(Vector2::new(ok_coord.x, ok_coord.y)),
                            });
                        }
                    };
                }
                UdpMsgUpType::PLAYER_TELEPORT => {
                    if let Some(ok_user) = user {
                        if let Some(ok_coord) = &udp_msg_up.player_teleport.0 {
                            self.world.send_event(UpdatePositionCurrent {
                                entity: ok_user.player_entity,
                                current: world_bounded_vector2(Vector2::new(
                                    ok_coord.x, ok_coord.y,
                                )),
                                force_update_velocity_target: true,
                            });
                        }
                    };
                }
                UdpMsgUpType::PLAYER_THROW_FROZEN_ORB => {
                    if let Some(ok_user) = user {
                        if let Some(ok_coord) = &udp_msg_up.player_throw_frozen_orb.0 {
                            let player_position =
                                self.world.get::<Position>(ok_user.player_entity).unwrap();
                            self.world.send_event(SpawnFrozenOrb {
                                from_entity: ok_user.player_entity,
                                from_position: player_position.current,
                                to_target: get_point_from_points_and_distance(
                                    player_position.current,
                                    Vector2::new(ok_coord.x, ok_coord.y),
                                    600.0,
                                ),
                                ignored_entity: ok_user.player_entity,
                            });
                        }
                    };
                }
                UdpMsgUpType::PLAYER_THROW_PROJECTILE => {
                    if let Some(ok_user) = user {
                        if let Some(ok_coord) = &udp_msg_up.player_throw_projectile.0 {
                            let player_position =
                                self.world.get::<Position>(ok_user.player_entity).unwrap();
                            self.world.send_event(SpawnProjectile {
                                from_entity: ok_user.player_entity,
                                from_position: player_position.current,
                                to_target: get_point_from_points_and_distance(
                                    player_position.current,
                                    Vector2::new(ok_coord.x, ok_coord.y),
                                    600.0,
                                ),
                                ignored_entity: ok_user.player_entity,
                            });
                        }
                    };
                }
                UdpMsgUpType::PLAYER_PING => {
                    if let Some(ok_player) = user {
                        ok_player.user_ping()
                    }
                }
                UdpMsgUpType::SETTINGS_TOGGLE_ENEMIES => {
                    let mut enemmies_state = self.world.get_resource_mut::<EnemiesState>().unwrap();
                    enemmies_state.toggle_enable();
                }
                // UdpMsgUpType::PLAYER_TOGGLE_HIDDEN => {
                //     if let Some(ok_user) = user {
                //         let opt_player = self.game_entity_manager.get_player(&ok_user.player_id);
                //         if let Some(player) = opt_player {
                //             player.user_toggle_hidden();
                //         }
                //     }
                // }
                _ => println!("Not handled event"),
            }
        }
    }
}
