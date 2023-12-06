use std::collections::HashMap;

use godot::builtin::Vector2;
use rust_common::proto::{
    common::GameEntityBaseType,
    udp_down::{UdpMsgDown, UdpMsgDownGameEntityRemoved, UdpMsgDownType},
};

use crate::{
    game::entity::{
        entity_base::{GameEntity, GameEntityParams},
        entity_health::GameEntityHealthParams,
        entity_location::GameEntityLocationParams,
    },
    utils::{get_game_time, get_id},
};

use super::{
    action::EGameEntityAction,
    frozen_orb::FrozenOrb,
    projectile::Projectile,
    types::{GameController, GameEntityController},
};

const PLAYER_MAX_HEALTH: u32 = 100;

pub struct Player {
    pub game_entity: GameEntity,
    pub action_queue: Vec<EGameEntityAction>,
    game_entities_ids_last_revision: HashMap<u32, u32>,
    respawn_at: Option<u32>,
}
impl Player {
    pub fn create() -> GameEntityController {
        GameEntityController::Player(Player {
            game_entity: GameEntity::new(
                get_id() as u32,
                GameEntityBaseType::CHARACTER,
                GameEntityParams {
                    location: Some(GameEntityLocationParams {
                        opt_current: Some(Vector2 { x: 0.0, y: 0.0 }),
                        opt_target: Some(Vector2 { x: 0.0, y: 0.0 }),
                        speed: 600.0,
                        is_static: false,
                        delete_if_oob: false,
                        delete_at_target: false,
                        shape: Vector2 { x: 100.0, y: 200.0 },
                    }),
                    health: Some(GameEntityHealthParams {
                        max: PLAYER_MAX_HEALTH,
                        min: 0,
                        opt_current: None,
                        delete_if_dead: false,
                    }),
                    dmg_on_hit: None,
                    duration: None,
                },
            ),
            action_queue: Vec::new(),
            game_entities_ids_last_revision: HashMap::new(),
            respawn_at: None,
        })
    }

    pub fn user_update_location_target(&mut self, new_x: f32, new_y: f32) {
        if self.respawn_at.is_none() {
            self.action_queue
                .push(EGameEntityAction::UpdateLocationTarget(Vector2 {
                    x: new_x,
                    y: new_y,
                }))
        }
    }

    pub fn user_instant_update_location(&mut self, new_x: f32, new_y: f32) {
        if self.respawn_at.is_none() {
            self.action_queue
                .push(EGameEntityAction::InstantUpdateLocation(Vector2 {
                    x: new_x,
                    y: new_y,
                }))
        }
    }

    pub fn user_throw_projectile(&mut self, to_x: f32, to_y: f32) {
        if self.respawn_at.is_none() {
            if let Some(location) = &self.game_entity.get_location() {
                self.action_queue.push(EGameEntityAction::ThrowProjectile(
                    *location.get_current(),
                    Vector2 { x: to_x, y: to_y },
                ))
            }
        }
    }

    pub fn user_throw_frozen_orb(&mut self, to_x: f32, to_y: f32) {
        if self.respawn_at.is_none() {
            if let Some(location) = &self.game_entity.get_location() {
                self.action_queue.push(EGameEntityAction::ThrowFrozenOrb(
                    *location.get_current(),
                    Vector2 { x: to_x, y: to_y },
                ))
            }
        }
    }

    pub fn user_toggle_hidden(&mut self) {
        if self.respawn_at.is_none() {
            self.action_queue.push(EGameEntityAction::ToggleHidden)
        }
    }

    pub fn get_serialization_of(&mut self, of_game_entity: &GameEntity) -> Option<UdpMsgDown> {
        let opt_last_seen_revision = self
            .game_entities_ids_last_revision
            .insert(of_game_entity.get_id(), of_game_entity.get_revision());

        let require_update = match opt_last_seen_revision {
            None => true,
            Some(last_seen_revision) => of_game_entity.get_revision() > last_seen_revision,
        };

        if !require_update {
            return None;
        }

        of_game_entity.serialize_for(&self.game_entity)
    }

    pub fn get_serialization_of_self(&mut self) -> Option<UdpMsgDown> {
        let opt_last_seen_revision = self
            .game_entities_ids_last_revision
            .insert(self.game_entity.get_id(), self.game_entity.get_revision());

        let require_update = match opt_last_seen_revision {
            None => true,
            Some(last_seen_revision) => self.game_entity.get_revision() > last_seen_revision,
        };

        if !require_update {
            return None;
        }

        self.game_entity.serialize_for(&self.game_entity)
    }

    pub fn get_serialization_of_deleted(&mut self, entity_id: &u32) -> Option<UdpMsgDown> {
        if self
            .game_entities_ids_last_revision
            .remove(entity_id)
            .is_some()
        {
            return Some(UdpMsgDown {
                _type: UdpMsgDownType::GAME_ENTITY_REMOVED.into(),
                game_entity_removed: Some(UdpMsgDownGameEntityRemoved {
                    id: *entity_id,
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            });
        }

        None
    }
}

impl GameController for Player {
    fn get_game_entity_mut(&mut self) -> &mut GameEntity {
        &mut self.game_entity
    }
    fn get_game_entity(&self) -> &GameEntity {
        &self.game_entity
    }

    fn analyze(&mut self, _: &GameEntityController) {}
    fn tick(&mut self) -> Vec<GameEntityController> {
        if let Some(health) = self.game_entity.get_health() {
            if self.respawn_at.is_none() && health.is_dead() {
                self.respawn_at = Some(get_game_time() + 5000);
            }
        }
        if let Some(respawn_at) = self.respawn_at {
            if respawn_at <= get_game_time() {
                self.respawn_at = None;
                self.action_queue.push(EGameEntityAction::HealthFullHeal);
                self.action_queue
                    .push(EGameEntityAction::InstantUpdateLocation(Vector2::ZERO));
            }
        }

        let mut new_controllers: Vec<GameEntityController> = Vec::new();

        for action in &self.action_queue {
            match action {
                EGameEntityAction::UpdateLocationTarget(coor) => {
                    if let Some(location) = &mut self.game_entity.get_location_mut() {
                        location.update_target(coor.x, coor.y);
                    }
                }
                EGameEntityAction::InstantUpdateLocation(coor) => {
                    if let Some(location) = &mut self.game_entity.get_location_mut() {
                        location.update_target(coor.x, coor.y);
                        location.update_current(coor.x, coor.y);
                    }
                }
                EGameEntityAction::ToggleHidden => self.game_entity.toggle_is_hidden(),
                EGameEntityAction::ThrowProjectile(from, to) => {
                    new_controllers.push(Projectile::create(self.game_entity.get_id(), *from, *to));
                    if let Some(location) = &mut self.game_entity.get_location_mut() {
                        location.update_target(location.get_current().x, location.get_current().y);
                    }
                }
                EGameEntityAction::ThrowFrozenOrb(from, to) => {
                    new_controllers.push(FrozenOrb::create(self.game_entity.get_id(), *from, *to));
                    if let Some(location) = &mut self.game_entity.get_location_mut() {
                        location.update_target(location.get_current().x, location.get_current().y);
                    }
                }
                EGameEntityAction::HealthFullHeal => {
                    if let Some(health) = &mut self.game_entity.get_health_mut() {
                        health.full_heal();
                    }
                }
            }
        }

        self.action_queue.clear();

        new_controllers
    }
}
