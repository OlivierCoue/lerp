use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use rand_core::RngCore;
use rust_common_game::prelude::*;

pub(crate) fn generate_item_dropped_on_death(
    mut commands: Commands,
    dead_enemy_q: Query<&Position, (Added<Dead>, With<Enemy>)>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for position in dead_enemy_q.iter() {
        let roll = rng.next_u64() % 100;
        let rarity = match roll {
            0..1 => Some(ItemRarity::Unique),   // 1%
            1..5 => Some(ItemRarity::Rare),     // 5%
            5..15 => Some(ItemRarity::Magic),   // 10%
            15..30 => Some(ItemRarity::Common), // 15%
            _ => None,                          // 65%
        };

        let Some(ratity) = rarity else {
            continue;
        };

        commands.spawn((
            ItemDropped {
                position: position.0,
                ratity,
            },
            Replicate {
                target: ReplicationTarget {
                    target: NetworkTarget::All,
                },
                group: LOOT_REPLICATION_GROUP,
                ..default()
            },
        ));
    }
}
