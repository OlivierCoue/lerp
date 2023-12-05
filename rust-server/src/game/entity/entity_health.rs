use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SGameEntityHealth {
    pub min: u32,
    pub max: u32,
    pub current: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct GameEntityHealthParams {
    pub min: u32,
    pub max: u32,
    pub opt_current: Option<u32>,
    pub delete_if_bellow_min: bool,
}

pub struct GameEntityHealth {
    min: u32,
    max: u32,
    current: u32,
    delete_if_bellow_min: bool,
    revision: u32,
}
impl GameEntityHealth {
    pub fn new(params: GameEntityHealthParams) -> GameEntityHealth {
        let GameEntityHealthParams {
            min,
            max,
            opt_current,
            delete_if_bellow_min,
        } = params;

        let current = match opt_current {
            Some(current) => current,
            None => max,
        };

        GameEntityHealth {
            max,
            min,
            current,
            delete_if_bellow_min,
            revision: 0,
        }
    }

    pub fn get_revision(&self) -> u32 {
        self.revision
    }

    pub fn get_current(&self) -> u32 {
        self.current
    }

    pub fn is_empty(&self) -> bool {
        self.current <= self.min
    }

    pub fn should_be_delete(&self) -> bool {
        self.delete_if_bellow_min && self.is_empty()
    }

    pub fn reduce_current(&mut self, dmg_value: u32) {
        if dmg_value < self.current {
            self.current -= dmg_value;
        } else {
            self.current = 0
        }
        self.revision += 1;
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> SGameEntityHealth {
        SGameEntityHealth {
            min: self.min,
            max: self.max,
            current: self.current,
        }
    }
}
