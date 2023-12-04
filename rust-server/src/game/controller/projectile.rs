use godot::builtin::Vector2;
use rust_common::proto::common::GameEntityBaseType;

use crate::{
    game::entity::{
        entity_base::{GameEntity, GameEntityParams},
        entity_location::GameEntityLocationParams,
    },
    utils::get_id,
};

use super::types::{GameController, GameEntityController};

pub struct Projectile {
    pub game_entity: GameEntity,
}
impl Projectile {
    pub fn create(from: Vector2, to: Vector2) -> GameEntityController {
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
                    }),
                    health: None,
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
