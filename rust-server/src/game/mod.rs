mod controller;
mod entity;
pub mod serialize;
pub mod user;

use rust_common::proto::data::UdpMsgDownWrapper;

use crate::utils::{get_timestamp_millis, inc_game_time_millis, Coord};
use std::cmp::max;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, sync::mpsc::Sender};

use crate::game::user::User;

use self::entity::entity_manager::GameEntityManager;
use self::serialize::udp_msg_up::{UdpMsgUp, UdpMsgUpTypes};

const TICK_RATE_MILLIS: u128 = 30;
const UPDATE_USERS_EVERY_N_TICK: u32 = 1;
const GAME_TIME_TICK_DURATION_MILLIS: u32 = 30;

pub struct Game<'a> {
    users: HashMap<u32, User<'a>>,
    users_curr_id: u32,
    peer_id_user_id_map: HashMap<u16, u32>,
    tx_enet_sender: &'a Sender<(u16, UdpMsgDownWrapper)>,
    paused: bool,
    game_entity_manager: GameEntityManager,
    clients_msg: &'a Mutex<VecDeque<(u16, UdpMsgUp)>>,
}

impl<'a> Game<'a> {
    pub fn new(
        tx_enet_sender: &'a Sender<(u16, UdpMsgDownWrapper)>,
        clients_msg: &'a Mutex<VecDeque<(u16, UdpMsgUp)>>,
    ) -> Game<'a> {
        Game {
            users: HashMap::new(),
            peer_id_user_id_map: HashMap::new(),
            users_curr_id: 0,
            tx_enet_sender,
            paused: false,
            game_entity_manager: GameEntityManager::new(),
            clients_msg,
        }
    }

    fn add_user(&mut self, peer_id: u16, udp_tunnel: &'a Sender<(u16, UdpMsgDownWrapper)>) {
        self.users_curr_id += 1;

        let player_id = self.game_entity_manager.add_player();

        let new_user = User::new(self.users_curr_id, peer_id, udp_tunnel, player_id);
        self.users.insert(self.users_curr_id, new_user);
        self.peer_id_user_id_map.insert(peer_id, self.users_curr_id);

        println!("Created new Player({})", self.users_curr_id);
    }

    fn delete_user(&mut self, user_id: u32) {
        if let Some(removed_user) = self.users.remove(&user_id) {
            self.game_entity_manager
                .remove_entity(&removed_user.player_id);
        }
    }

    pub fn start(&mut self) {
        let mut tick_count = 0;
        loop {
            let started_at: u128 = get_timestamp_millis();
            tick_count += 1;
            self.tick(tick_count == UPDATE_USERS_EVERY_N_TICK);
            if tick_count == UPDATE_USERS_EVERY_N_TICK {
                tick_count = 0;
            }

            let tick_duration = get_timestamp_millis() - started_at;
            if tick_duration < TICK_RATE_MILLIS {
                let time_to_wait = max(TICK_RATE_MILLIS - tick_duration, 0);
                thread::sleep(Duration::from_millis(time_to_wait as u64));
            } else {
                println!(
                    "WARNING tick_duration ({}) in more than TICK_RATE_MILLIS ({})",
                    tick_duration, TICK_RATE_MILLIS
                );
            }
        }
    }

    pub fn tick(&mut self, update_users: bool) {
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

        let mut player_msg_down_map = self.game_entity_manager.tick(update_users);
        inc_game_time_millis(GAME_TIME_TICK_DURATION_MILLIS);

        for user in self.users.values() {
            if let Some(messages) = player_msg_down_map.remove(&user.player_id) {
                user.send_message(UdpMsgDownWrapper {
                    server_time: get_timestamp_millis() as u64,
                    messages,
                    ..Default::default()
                })
            }
        }
    }

    pub fn handle_upd_msg_up(&mut self, from_enet_peer_id: &u16, udp_msg_up: &UdpMsgUp) {
        let from_actor_id = match self.peer_id_user_id_map.get(from_enet_peer_id) {
            None => {
                println!("New message from unknown addr {from_enet_peer_id}");
                999
            }
            Some(player_id) => *player_id,
        };

        let user = self.users.get_mut(&from_actor_id);

        match udp_msg_up.msg_type {
            UdpMsgUpTypes::GamePause => self.paused = !self.paused,
            UdpMsgUpTypes::PlayerInit => {
                if user.is_none() {
                    self.add_user(*from_enet_peer_id, self.tx_enet_sender)
                };
            }
            UdpMsgUpTypes::PlayerMove => {
                if let Some(ok_user) = user {
                    let opt_player = self.game_entity_manager.get_player(&ok_user.player_id);

                    if let (Ok(ok_coord), Some(player)) = (
                        serde_json::from_str::<Coord>(&udp_msg_up.msg_payload),
                        opt_player,
                    ) {
                        player.user_update_location_target(ok_coord.x, ok_coord.y)
                    }
                };
            }
            UdpMsgUpTypes::PlayerTeleport => {
                if let Some(ok_user) = user {
                    let opt_player = self.game_entity_manager.get_player(&ok_user.player_id);

                    if let (Ok(ok_coord), Some(player)) = (
                        serde_json::from_str::<Coord>(&udp_msg_up.msg_payload),
                        opt_player,
                    ) {
                        player.user_instant_update_location(ok_coord.x, ok_coord.y)
                    }
                };
            }
            UdpMsgUpTypes::PlayerThrowFrozenOrb => {
                if let Some(ok_user) = user {
                    let opt_player = self.game_entity_manager.get_player(&ok_user.player_id);

                    if let (Ok(ok_coord), Some(player)) = (
                        serde_json::from_str::<Coord>(&udp_msg_up.msg_payload),
                        opt_player,
                    ) {
                        player.user_throw_frozen_orb(ok_coord.x, ok_coord.y)
                    }
                };
            }
            UdpMsgUpTypes::PlayerThrowProjectile => {
                if let Some(ok_user) = user {
                    let opt_player = self.game_entity_manager.get_player(&ok_user.player_id);

                    if let (Ok(ok_coord), Some(player)) = (
                        serde_json::from_str::<Coord>(&udp_msg_up.msg_payload),
                        opt_player,
                    ) {
                        player.user_throw_projectile(ok_coord.x, ok_coord.y)
                    }
                };
            }
            UdpMsgUpTypes::PlayerPing => {
                if let Some(ok_player) = user {
                    ok_player.user_ping()
                }
            }
            UdpMsgUpTypes::PlayerToggleHidden => {
                if let Some(ok_user) = user {
                    let opt_player = self.game_entity_manager.get_player(&ok_user.player_id);
                    if let Some(player) = opt_player {
                        player.user_toggle_hidden();
                    }
                }
            }
        }
    }
}
