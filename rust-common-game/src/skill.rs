use avian2d::prelude::Position;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{projectile::SpawnProjectileEvent, protocol::Player};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug, Copy, Reflect)]
pub enum Skill {
    BowAttack,
    SplitArrow,
}

#[derive(Event)]
pub struct SkillBowAttackEvent {
    pub initiator: Entity,
    pub target: Vec2,
}

pub fn on_skill_bow_attack(
    mut skill_bow_attack_event: EventReader<SkillBowAttackEvent>,
    mut spawn_projectile_events: EventWriter<SpawnProjectileEvent>,
    initiator_q: Query<(&Position, Option<&Player>)>,
) {
    for event in skill_bow_attack_event.read() {
        if let Ok((initiator_position, initiator_player)) = initiator_q.get(event.initiator) {
            let direction = (event.target - initiator_position.0).normalize();

            spawn_projectile_events.send(SpawnProjectileEvent {
                client_id: initiator_player.map(|p| p.0),
                from_position: initiator_position.0,
                direction,
            });
        }
    }
}

#[derive(Event)]
pub struct SkillSplitArrowEvent {
    pub initiator: Entity,
    pub target: Vec2,
}

pub fn on_skill_split_attack(
    mut skill_split_arrow_event: EventReader<SkillSplitArrowEvent>,
    mut spawn_projectile_events: EventWriter<SpawnProjectileEvent>,
    initiator_q: Query<(&Position, Option<&Player>)>,
) {
    for event in skill_split_arrow_event.read() {
        if let Ok((initiator_position, initiator_player)) = initiator_q.get(event.initiator) {
            let directions =
                generate_fan_projectile_directions(initiator_position.0, event.target, 3, 20.);

            for direction in directions {
                spawn_projectile_events.send(SpawnProjectileEvent {
                    client_id: initiator_player.map(|p| p.0),
                    from_position: initiator_position.0,
                    direction,
                });
            }
        }
    }
}

fn generate_fan_projectile_directions(
    from: Vec2,
    target: Vec2,
    count: u32,
    angle: f32,
) -> Vec<Vec2> {
    if count == 0 {
        return Vec::new();
    }

    // Calculate the straight direction (normalized)
    let direction = (target - from).normalize();

    // Convert the angle to radians
    let angle_rad = angle.to_radians();

    // Calculate the initial rotation angle (to center the fan)
    let total_angle = angle_rad * (count as f32 - 1.0);
    let start_angle = -total_angle / 2.0;

    // Helper to rotate a vector by a given angle
    let rotate = |v: Vec2, angle: f32| {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
    };

    // Generate directions
    (0..count)
        .map(|i| {
            let current_angle = start_angle + angle_rad * i as f32;
            rotate(direction, current_angle)
        })
        .collect()
}
