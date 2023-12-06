use godot::builtin::Vector2;
use rust_common::proto::common::GameEntityBaseType;

use crate::{
    game::entity::{
        entity_base::{GameEntity, GameEntityParams},
        entity_damage_on_hit::GameEntityDamageOnHitParams,
        entity_location::GameEntityLocationParams,
    },
    utils::get_id,
};

use super::{
    projectile::Projectile,
    types::{GameController, GameEntityController},
};

pub struct FrozenOrb {
    pub game_entity: GameEntity,
    source_entity_id: u32,
}
impl FrozenOrb {
    pub fn create(source_entity_id: u32, from: Vector2, to: Vector2) -> GameEntityController {
        GameEntityController::FronzenOrb(FrozenOrb {
            game_entity: GameEntity::new(
                get_id() as u32,
                GameEntityBaseType::PROJECTILE,
                GameEntityParams {
                    location: Some(GameEntityLocationParams {
                        opt_current: Some(from),
                        opt_target: Some(to),
                        speed: 1000.0,
                        is_static: false,
                        delete_if_oob: true,
                        delete_at_target: false,
                        shape: Vector2 { x: 50.0, y: 50.0 },
                    }),
                    health: None,
                    dmg_on_hit: Some(GameEntityDamageOnHitParams {
                        dmg_value: 10,
                        ignored_entity_id: source_entity_id,
                    }),
                    duration: None,
                },
            ),
            source_entity_id,
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

        if let Some(location) = &self.game_entity.get_location() {
            if location.is_at_target() {
                let location_current = location.get_current();
                new_controllers.push(Projectile::create(
                    self.source_entity_id,
                    *location_current,
                    Vector2 {
                        x: location_current.x + 600.0,
                        y: location_current.y,
                    },
                ));
                new_controllers.push(Projectile::create(
                    self.source_entity_id,
                    *location_current,
                    Vector2 {
                        x: location_current.x - 600.0,
                        y: location_current.y,
                    },
                ));
                new_controllers.push(Projectile::create(
                    self.source_entity_id,
                    *location_current,
                    Vector2 {
                        x: location_current.x,
                        y: location_current.y + 600.0,
                    },
                ));
                new_controllers.push(Projectile::create(
                    self.source_entity_id,
                    *location_current,
                    Vector2 {
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
