use std::collections::HashMap;

use rust_common::proto::data::{
    GameEntityBaseType, UdpMsgDown, UdpMsgDownGameEntityRemoved, UdpMsgDownType,
};

use crate::{
    game::entity::{
        entity_base::{GameEntity, GameEntityParams},
        entity_health::GameEntityHealthParams,
        entity_location::GameEntityLocationParams,
    },
    utils::{get_id, Coord},
};

use super::{
    action::EGameEntityAction,
    frozen_orb::FrozenOrb,
    projectile::Projectile,
    types::{GameController, GameEntityController},
};

pub struct Player {
    pub game_entity: GameEntity,
    pub action_queue: Vec<EGameEntityAction>,
    game_entities_ids_last_revision: HashMap<u32, u32>,
}
impl Player {
    pub fn create() -> GameEntityController {
        GameEntityController::Player(Player {
            game_entity: GameEntity::new(
                get_id() as u32,
                GameEntityBaseType::CHARACTER,
                GameEntityParams {
                    location: Some(GameEntityLocationParams {
                        opt_current: Some(Coord { x: 0.0, y: 0.0 }),
                        opt_target: Some(Coord { x: 0.0, y: 0.0 }),
                        speed: 25.0,
                        is_static: false,
                        delete_if_oob: false,
                        delete_at_target: false,
                    }),
                    health: Some(GameEntityHealthParams {
                        max: 100,
                        min: 0,
                        opt_current: None,
                        delete_if_bellow_min: true,
                    }),
                    duration: None,
                },
            ),
            action_queue: Vec::new(),
            game_entities_ids_last_revision: HashMap::new(),
        })
    }

    pub fn user_update_location_target(&mut self, new_x: f64, new_y: f64) {
        self.action_queue
            .push(EGameEntityAction::UpdateLocationTarget(Coord {
                x: new_x,
                y: new_y,
            }))
    }

    pub fn user_instant_update_location(&mut self, new_x: f64, new_y: f64) {
        self.action_queue
            .push(EGameEntityAction::InstantUpdateLocation(Coord {
                x: new_x,
                y: new_y,
            }))
    }

    pub fn user_throw_projectile(&mut self, to_x: f64, to_y: f64) {
        if let Some(location) = &self.game_entity.location {
            self.action_queue.push(EGameEntityAction::ThrowProjectile(
                *location.get_current(),
                Coord { x: to_x, y: to_y },
            ))
        }
    }

    pub fn user_throw_frozen_orb(&mut self, to_x: f64, to_y: f64) {
        if let Some(location) = &self.game_entity.location {
            self.action_queue.push(EGameEntityAction::ThrowFrozenOrb(
                *location.get_current(),
                Coord { x: to_x, y: to_y },
            ))
        }
    }

    pub fn user_toggle_hidden(&mut self) {
        self.action_queue.push(EGameEntityAction::ToggleHidden)
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
        let mut new_controllers: Vec<GameEntityController> = Vec::new();

        for action in &self.action_queue {
            match action {
                EGameEntityAction::UpdateLocationTarget(coor) => {
                    if let Some(location) = &mut self.game_entity.location {
                        location.update_target(coor.x, coor.y);
                    }
                }
                EGameEntityAction::InstantUpdateLocation(coor) => {
                    if let Some(location) = &mut self.game_entity.location {
                        location.update_target(coor.x, coor.y);
                        location.update_current(coor.x, coor.y);
                    }
                }
                EGameEntityAction::ToggleHidden => self.game_entity.toggle_is_hidden(),
                EGameEntityAction::ThrowProjectile(from, to) => {
                    new_controllers.push(Projectile::create(*from, *to));
                }
                EGameEntityAction::ThrowFrozenOrb(from, to) => {
                    new_controllers.push(FrozenOrb::create(*from, *to));
                }
            }
        }

        self.action_queue.clear();

        new_controllers
    }
}
