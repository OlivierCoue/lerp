use rust_common::proto::data::{
    GameEntityBaseType, Point, UdpMsgDown, UdpMsgDownGameEntityUpdate, UdpMsgDownType,
};

use crate::utils::get_game_time;

use super::{
    entity_health::{GameEntityHealth, GameEntityHealthParams},
    entity_location::{GameEntityLocation, GameEntityLocationParams},
};

#[derive(Debug, Copy, Clone)]
pub struct GameEntityParams {
    pub location: Option<GameEntityLocationParams>,
    pub health: Option<GameEntityHealthParams>,
    pub duration: Option<u32>,
}

pub enum GameEntityLifeState {
    Alive,
    Deleted,
}

pub struct GameEntity {
    id: u32,
    object_type: GameEntityBaseType,
    pub location: Option<GameEntityLocation>,
    pub health: Option<GameEntityHealth>,
    is_hidden: bool,
    duration: Option<u32>,
    created_at_millis: u32,
    revision: u32,
    life_state: GameEntityLifeState,
    require_deletion: bool,
}
impl GameEntity {
    pub fn new(id: u32, object_type: GameEntityBaseType, params: GameEntityParams) -> GameEntity {
        let location = params.location.map(GameEntityLocation::new);
        let health = params.health.map(GameEntityHealth::new);

        GameEntity {
            id,
            object_type,
            location,
            health,
            is_hidden: false,
            duration: params.duration,
            created_at_millis: get_game_time(),
            revision: 0,
            life_state: GameEntityLifeState::Alive,
            require_deletion: false,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_revision(&self) -> u32 {
        let mut revision = 0;
        if let Some(location) = &self.location {
            revision += location.get_revision()
        }
        self.revision + revision
    }

    pub fn is_visible_for(&self, other: &GameEntity) -> bool {
        !self.is_hidden || self.id == other.get_id()
    }

    // pub fn get_age(&self) -> u32 {
    //     get_game_time() - self.created_at
    // }

    pub fn is_alive(&self) -> bool {
        !self.require_deletion && matches!(self.life_state, GameEntityLifeState::Alive)
    }

    fn should_be_delete(&self) -> bool {
        if let Some(duration) = self.duration {
            if get_game_time() >= self.created_at_millis + duration {
                return true;
            }
        }

        if let Some(location) = &self.location {
            if location.should_be_delete() {
                return true;
            }
        }

        if let Some(health) = &self.health {
            if health.should_be_delete() {
                return true;
            }
        }

        self.require_deletion
    }

    pub fn require_deletion(&mut self) {
        self.require_deletion = true;
        self.revision += 1;
    }

    pub fn toggle_is_hidden(&mut self) {
        self.is_hidden = !self.is_hidden;
        self.revision += 1;
    }

    pub fn tick_for(&mut self, _: &GameEntity) {}

    pub fn tick_self(&mut self) {
        let stop = match self.life_state {
            GameEntityLifeState::Alive => {
                let mut stop = false;
                if self.should_be_delete() {
                    self.life_state = GameEntityLifeState::Deleted;
                    self.revision += 1;
                    stop = true;
                }
                stop
            }
            GameEntityLifeState::Deleted => {
                self.revision += 1;
                true
            }
        };

        if stop {
            return;
        };
        if let Some(location) = &mut self.location {
            location.move_to_target()
        }
    }

    pub fn serialize_for(&self, for_game_entity: &GameEntity) -> Option<UdpMsgDown> {
        if !self.is_alive() {
            return None;
        }

        let mut location_current: Option<Point> = None;
        let mut location_target: Option<Point> = None;
        let mut location_speed: Option<f64> = None;
        // let mut health = None;

        if self.is_visible_for(for_game_entity) {
            location_current = self
                .location
                .as_ref()
                .map(|location| location.get_current().to_point());
            location_target = self
                .location
                .as_ref()
                .map(|location| location.get_target().to_point());
            location_speed = self.location.as_ref().map(|location| location.speed);
            // health = self.health.as_ref().map(|health| health.serialize());
        };

        Some(UdpMsgDown {
            _type: UdpMsgDownType::GAME_ENTITY_UPDATE.into(),
            game_entity_update: (Some(UdpMsgDownGameEntityUpdate {
                id: self.id,
                object_type: self.object_type.into(),
                location_current: location_current.into(),
                location_target: location_target.into(),
                location_speed,
                is_self: for_game_entity.get_id() == self.id,
                ..Default::default()
            }))
            .into(),
            ..Default::default()
        })
    }
}
