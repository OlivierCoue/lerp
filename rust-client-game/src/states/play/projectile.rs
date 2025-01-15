use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, PreSpawnedPlayerObject};
use rust_common_game::{
    projectile::{Projectile, ProjectileBundle},
    shared::PIXEL_METER,
};

use crate::IsoZ;

use super::{apply_render_mode, PlaySceneTag, RenderConfig};

#[derive(Bundle)]
pub struct ProjecileDisplayBundle {
    pub play_scene_tag: PlaySceneTag,
    pub transform: Transform,
    pub transform_interpolation: TransformInterpolation,
    pub sprite: Sprite,
    pub iso_z: IsoZ,
}

pub fn handle_new_projectile(
    render_config: Res<RenderConfig>,
    asset_server: Res<AssetServer>,
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
        let mut translation = apply_render_mode(&render_config, position).extend(1.);
        translation.y += 1. * PIXEL_METER;

        commands.entity(entity).insert(ProjecileDisplayBundle {
            play_scene_tag: PlaySceneTag,
            transform: Transform::from_translation(translation),
            transform_interpolation: TransformInterpolation,
            sprite: Sprite::from_image(asset_server.load("assets/projectile-16x8.png")),
            iso_z: IsoZ(1.),
        });

        commands
            .entity(entity)
            .insert_if_new(ProjectileBundle::from_protocol());
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
