use rust_common::proto::data::GameEntityBaseType;

use crate::{
    game::entity::{
        entity_base::{GameEntity, GameEntityParams},
        entity_location::GameEntityLocationParams,
    },
    utils::{get_id, Coord},
};

use super::{
    projectile::Projectile,
    types::{GameController, GameEntityController},
};

pub struct FrozenOrb {
    pub game_entity: GameEntity,
}
impl FrozenOrb {
    pub fn create(from: Coord, to: Coord) -> GameEntityController {
        GameEntityController::FronzenOrb(FrozenOrb {
            game_entity: GameEntity::new(
                get_id() as u32,
                GameEntityBaseType::PROJECTILE,
                GameEntityParams {
                    location: Some(GameEntityLocationParams {
                        opt_current: Some(from),
                        opt_target: Some(to),
                        speed: 30.0,
                        is_static: false,
                        delete_if_oob: true,
                        delete_at_target: false,
                    }),
                    health: None,
                    duration: None,
                },
            ),
        })
    }
}

impl GameController for FrozenOrb {
    fn get_game_entity_mut(&mut self) -> &mut GameEntity {
        &mut self.game_entity
    }
    fn get_game_entity(&self) -> &GameEntity {
        &self.game_entity
    }
    fn analyze(&mut self, _: &GameEntityController) {}
    fn tick(&mut self) -> Vec<GameEntityController> {
        let mut new_controllers: Vec<GameEntityController> = Vec::new();

        if let Some(location) = &self.game_entity.location {
            if location.is_at_target() {
                let location_current = location.get_current();
                new_controllers.push(Projectile::create(
                    *location_current,
                    Coord {
                        x: location_current.x + 600.0,
                        y: location_current.y,
                    },
                ));
                new_controllers.push(Projectile::create(
                    *location_current,
                    Coord {
                        x: location_current.x - 600.0,
                        y: location_current.y,
                    },
                ));
                new_controllers.push(Projectile::create(
                    *location_current,
                    Coord {
                        x: location_current.x,
                        y: location_current.y + 600.0,
                    },
                ));
                new_controllers.push(Projectile::create(
                    *location_current,
                    Coord {
                        x: location_current.x,
                        y: location_current.y - 600.0,
                    },
                ));
                self.game_entity.require_deletion();
            }
        }
        new_controllers
    }
}
