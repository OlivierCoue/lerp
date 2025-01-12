use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, PreSpawnedPlayerObject};
use rust_common_game::projectile::Projectile;

use super::{apply_render_mode, PlaySceneTag, RenderConfig};

pub fn handle_new_projectile(
    render_config: Res<RenderConfig>,
    mut commands: Commands,
    mut projectile_query: Query<
        (Entity, &Position),
        (
            Or<(Added<Predicted>, Added<PreSpawnedPlayerObject>)>,
            With<Projectile>,
        ),
    >,
) {
    for (entity, position) in projectile_query.iter_mut() {
        let translation = apply_render_mode(&render_config, position).extend(1.);
        commands.entity(entity).insert((
            PlaySceneTag,
            Transform::from_translation(translation),
            TransformInterpolation,
            Visibility::default(),
        ));
    }
}
