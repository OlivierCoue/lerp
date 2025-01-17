use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicationTarget};
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Mana {
    pub max: f32,
    pub current: f32,
}
impl Mana {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }
}

pub fn mana_regeneration(
    time: Res<Time<Fixed>>,
    mut mana_q: Query<&mut Mana, (Or<(With<Predicted>, With<ReplicationTarget>)>,)>,
) {
    for mut mana in mana_q.iter_mut() {
        let regen_amount = 50. * time.delta_secs();

        mana.current = (mana.current + regen_amount).min(mana.max).max(0.);
    }
}
