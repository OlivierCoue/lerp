use std::collections::HashMap;

use rust_common::proto::udp_down::UdpMsgDown;

use crate::game::controller::{player::Player, types::GameController, types::GameEntityController};

pub struct GameEntityManager {
    entities: HashMap<u32, GameEntityController>,
    entities_deleted: Vec<GameEntityController>,
}
impl GameEntityManager {
    pub fn new() -> GameEntityManager {
        GameEntityManager {
            entities: HashMap::new(),
            entities_deleted: Vec::new(),
        }
    }

    pub fn add_player(&mut self) -> u32 {
        let player = Player::create();
        let player_id = player.get_game_entity().get_id();
        self.entities.insert(player_id, player);
        player_id
    }

    pub fn remove_entity(&mut self, player_id: &u32) {
        if let Some(controller) = self.entities.get_mut(player_id) {
            controller.get_game_entity_mut().require_deletion();
        }
    }

    pub fn get_player(&mut self, player_id: &u32) -> Option<&mut Player> {
        let opt_entity = self.entities.get_mut(player_id);

        if let Some(GameEntityController::Player(player)) = opt_entity {
            return Some(player);
        }

        None
    }

    pub fn tick(&mut self, update_users: bool) -> HashMap<u32, Vec<UdpMsgDown>> {
        let entity_ids: Vec<u32> = self.entities.keys().cloned().collect();
        let mut udp_msg_downs_map: HashMap<u32, Vec<UdpMsgDown>> = HashMap::new();

        let mut entities_deleted = Vec::new();

        if update_users && !self.entities_deleted.is_empty() {
            entities_deleted = self.entities_deleted.drain(0..).collect::<Vec<_>>();
        }

        for entity_id_a in &entity_ids {
            let opt_entity_controller_a = self.entities.remove(entity_id_a);

            if let Some(mut controller_a) = opt_entity_controller_a {
                let mut udp_msg_downs = Vec::new();

                controller_a.get_game_entity_mut().tick_self();
                if controller_a.get_game_entity().is_alive() {
                    let new_controllers = controller_a.tick();
                    for new_controller in new_controllers {
                        let new_controller_id = new_controller.get_game_entity().get_id();
                        self.entities.insert(new_controller_id, new_controller);
                    }
                }

                for entity_id_b in &entity_ids {
                    let opt_entity_tu_b = self.entities.get_mut(entity_id_b);

                    if let Some(controller_b) = opt_entity_tu_b {
                        controller_a.analyze(controller_b);
                        controller_a
                            .get_game_entity_mut()
                            .tick_for(controller_b.get_game_entity_mut());

                        if update_users {
                            if let GameEntityController::Player(player) = &mut controller_a {
                                if let Some(udp_msg_down) =
                                    player.get_serialization_of(controller_b.get_game_entity())
                                {
                                    udp_msg_downs.push(udp_msg_down);
                                }
                            }
                        }
                    }
                }

                if update_users {
                    if let GameEntityController::Player(player) = &mut controller_a {
                        if let Some(udp_msg_down) = player.get_serialization_of_self() {
                            udp_msg_downs.push(udp_msg_down);
                        }
                        for entity_deleted in &entities_deleted {
                            if let Some(udp_msg_down) = player.get_serialization_of_deleted(
                                &entity_deleted.get_game_entity().get_id(),
                            ) {
                                udp_msg_downs.push(udp_msg_down);
                            }
                        }
                        if !udp_msg_downs.is_empty() {
                            udp_msg_downs_map.insert(*entity_id_a, udp_msg_downs);
                        }
                    }
                }

                if controller_a.get_game_entity().is_alive() {
                    self.entities.insert(*entity_id_a, controller_a);
                } else {
                    self.entities_deleted.push(controller_a);
                }
            }
        }

        udp_msg_downs_map
    }
}
