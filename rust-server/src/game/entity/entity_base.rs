use rust_common::{
    helper::vector2_to_point,
    proto::{
        common::{GameEntityBaseType, Point},
        udp_down::{UdpMsgDown, UdpMsgDownGameEntityUpdate, UdpMsgDownType},
    },
};

use crate::utils::get_game_time;

use super::{
    entity_damage_on_hit::{GameEntityDamageOnHit, GameEntityDamageOnHitParams},
    entity_health::{GameEntityHealth, GameEntityHealthParams},
    entity_location::{GameEntityLocation, GameEntityLocationParams},
};

#[derive(Debug, Copy, Clone)]
pub struct GameEntityParams {
    pub location: Option<GameEntityLocationParams>,
    pub health: Option<GameEntityHealthParams>,
    pub dmg_on_hit: Option<GameEntityDamageOnHitParams>,
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
    health: Option<GameEntityHealth>,
    dmg_on_hit: Option<GameEntityDamageOnHit>,
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
        let dmg_on_hit = params.dmg_on_hit.map(GameEntityDamageOnHit::new);

        GameEntity {
            id,
            object_type,
            location,
            health,
            dmg_on_hit,
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
        if let Some(health) = &self.health {
            revision += health.get_revision()
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

    pub fn tick_for(&mut self, other: &mut GameEntity) {
        if self.health.is_some() {
            if let (Some(location), Some(other_location)) = (&self.location, &other.location) {
                let r1_pos = location.get_current();
                let r1_size = location.get_shape();
                let r2_pos = other_location.get_current();
                let r2_size = other_location.get_shape();

                let r1x = r1_pos.x - r1_size.x / 2.0;
                let r1y = r1_pos.y - r1_size.y / 2.0;
                let r1w = r1_size.x;
                let r1h = r1_size.y;
                let r2x = r2_pos.x - r2_size.x / 2.0;
                let r2y = r2_pos.y - r2_size.y / 2.0;
                let r2w = r2_size.x;
                let r2h = r2_size.y;

                // https://www.jeffreythompson.org/collision-detection/rect-rect.php
                if r1x + r1w >= r2x && r1x <= r2x + r2w && r1y + r1h >= r2y && r1y <= r2y + r2h {
                    if let Some(dmg_on_hit) = &mut other.dmg_on_hit {
                        if let Some(dmg_value) = dmg_on_hit.get_dmg_value_for(self) {
                            if let Some(mut_health) = &mut self.health {
                                mut_health.reduce_current(dmg_value);
                            }
                        }
                    }
                }
            }
        }
    }

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
        let mut location_shape: Option<Point> = None;
        let mut location_speed: Option<f32> = None;
        let mut health_current: Option<u32> = None;
        // let mut health = None;

        if self.is_visible_for(for_game_entity) {
            location_current = self
                .location
                .as_ref()
                .map(|location| vector2_to_point(location.get_current()));
            location_target = self
                .location
                .as_ref()
                .map(|location| vector2_to_point(location.get_target()));
            location_shape = self
                .location
                .as_ref()
                .map(|location| vector2_to_point(location.get_shape()));
            location_speed = self.location.as_ref().map(|location| location.speed);
            health_current = self.health.as_ref().map(|health| health.get_current());
        };

        Some(UdpMsgDown {
            _type: UdpMsgDownType::GAME_ENTITY_UPDATE.into(),
            game_entity_update: (Some(UdpMsgDownGameEntityUpdate {
                id: self.id,
                object_type: self.object_type.into(),
                location_current: location_current.into(),
                location_target: location_target.into(),
                location_shape: location_shape.into(),
                location_speed,
                health_current,
                is_self: for_game_entity.get_id() == self.id,
                ..Default::default()
            }))
            .into(),
            ..Default::default()
        })
    }
}
