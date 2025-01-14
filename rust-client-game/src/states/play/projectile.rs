use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, PreSpawnedPlayerObject};
use rust_common_game::projectile::{EntityPhysics, Projectile, ProjectileBundle};

use super::{apply_render_mode, PlaySceneTag, RenderConfig};

#[derive(Bundle)]
pub struct ProjecileDisplayBundle {
    pub play_scene_tag: PlaySceneTag,
    pub transform: Transform,
    pub transform_interpolation: TransformInterpolation,
    pub sprite: Sprite,
}

pub fn handle_new_projectile(
    render_config: Res<RenderConfig>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut projectile_query: Query<
        (Entity, &Position, Has<EntityPhysics>),
        (
            Or<(Added<Predicted>, Added<PreSpawnedPlayerObject>)>,
            With<Projectile>,
        ),
    >,
) {
    for (entity, position, has_physics) in projectile_query.iter_mut() {
        let translation = apply_render_mode(&render_config, position).extend(1.);

        commands.entity(entity).insert(ProjecileDisplayBundle {
            play_scene_tag: PlaySceneTag,
            transform: Transform::from_translation(translation),
            transform_interpolation: TransformInterpolation,
            sprite: Sprite::from_image(asset_server.load("assets/projectile-16x8.png")),
        });

        if !has_physics {
            commands.entity(entity).insert(ProjectileBundle::physics());
        }
    }
}

pub fn handle_removed_projectile(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>>,
    mut projectile_query: RemovedComponents<Projectile>,
) {
    for entity in projectile_query.read() {
        if query.get(entity).is_ok() {
            commands
                .entity(entity)
                .remove::<(ProjectileBundle, ProjecileDisplayBundle)>();
            commands.entity(entity).despawn_descendants();
        }
    }
}
