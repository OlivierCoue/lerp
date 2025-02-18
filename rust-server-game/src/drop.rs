use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use rust_common_game::prelude::*;

pub(crate) fn generate_loop_on_death(
    mut commands: Commands,
    dead_enemy_q: Query<&Position, (Added<Dead>, With<Enemy>)>,
) {
    let mut i = 0;
    for position in dead_enemy_q.iter() {
        commands.spawn((
            Loot {
                position: position.0,
            },
            Replicate {
                target: ReplicationTarget {
                    target: NetworkTarget::All,
                },
                group: LOOT_REPLICATION_GROUP,
                ..default()
            },
        ));
        i += 1;
    }
    if i > 0 {
        println!("{}", i);
    }
}
