use std::collections::{hash_map, HashMap};

use super::entity_base::GameEntity;

#[derive(Debug, Copy, Clone)]
pub struct GameEntityDamageOnHitParams {
    pub dmg_value: u32,
    pub ignored_entity_id: u32,
}

pub struct GameEntityDamageOnHit {
    dmg_value: u32,
    entity_id_hitted: HashMap<u32, bool>,
    ignored_entity_id: u32,
}
impl GameEntityDamageOnHit {
    pub fn new(params: GameEntityDamageOnHitParams) -> GameEntityDamageOnHit {
        let GameEntityDamageOnHitParams {
            dmg_value,
            ignored_entity_id,
        } = params;

        GameEntityDamageOnHit {
            dmg_value,
            entity_id_hitted: HashMap::new(),
            ignored_entity_id,
        }
    }

    pub fn get_dmg_value_for(&mut self, other: &GameEntity) -> Option<u32> {
        if other.get_id() == self.ignored_entity_id {
            return None;
        }

        if let hash_map::Entry::Vacant(entity_id_hitted_entry) =
            self.entity_id_hitted.entry(other.get_id())
        {
            entity_id_hitted_entry.insert(true);
            return Some(self.dmg_value);
        }
        None
    }
}
