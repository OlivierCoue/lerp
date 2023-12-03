use enum_dispatch::enum_dispatch;

use crate::game::entity::entity_base::GameEntity;

use super::{frozen_orb::FrozenOrb, player::Player, projectile::Projectile};

#[enum_dispatch]
pub enum GameEntityController {
    Player(Player),
    Projectile(Projectile),
    FronzenOrb(FrozenOrb),
}

#[enum_dispatch(GameEntityController)]
pub trait GameController {
    fn get_game_entity_mut(&mut self) -> &mut GameEntity;
    fn get_game_entity(&self) -> &GameEntity;
    fn analyze(&mut self, other_controller: &GameEntityController);
    fn tick(&mut self) -> Vec<GameEntityController>;
}
