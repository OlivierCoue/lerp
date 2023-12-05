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

use super::types::{GameController, GameEntityController};

pub struct Projectile {
    pub game_entity: GameEntity,
}
impl Projectile {
    pub fn create(source_entity_id: u32, from: Vector2, to: Vector2) -> GameEntityController {
        GameEntityController::Projectile(Projectile {
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
                        delete_at_target: true,
                        shape: Vector2 { x: 50.0, y: 50.0 },
                    }),
                    health: None,
                    dmg_on_hit: Some(GameEntityDamageOnHitParams {
                        dmg_value: 5,
                        ignored_entity_id: source_entity_id,
                    }),
                    duration: None,
                },
            ),
        })
    }
}
impl GameController for Projectile {
    fn get_game_entity_mut(&mut self) -> &mut GameEntity {
        &mut self.game_entity
    }
    fn get_game_entity(&self) -> &GameEntity {
        &self.game_entity
    }
    fn analyze(&mut self, _: &GameEntityController) {}
    fn tick(&mut self) -> Vec<GameEntityController> {
        Vec::new()
    }
}
