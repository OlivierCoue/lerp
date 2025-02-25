use bevy::prelude::*;
use lerp_map_gen::*;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(MapGenPlugin)
        .add_systems(Update, access_map)
        .run();
}
