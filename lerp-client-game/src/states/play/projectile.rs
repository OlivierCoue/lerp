use avian2d::prelude::*;
use bevy::prelude::*;
use lerp_common_game::prelude::*;
use lightyear::prelude::{client::Predicted, PreSpawnedPlayerObject};

use crate::{
    common::{cartesian_to_isometric_vec2, AppState},
    utils::ZLayer,
    IsoZ,
};

use super::PlaySceneTag;

#[derive(Bundle)]
struct ProjecileDisplayBundle {
    pub play_scene_tag: PlaySceneTag,
    pub transform: Transform,
    pub transform_interpolation: TransformInterpolation,
    pub sprite: Sprite,
    pub iso_z: IsoZ,
    pub z_layer: ZLayer,
}

fn handle_new_projectile(
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
        let mut translation = cartesian_to_isometric_vec2(position).extend(1.);
        translation.y += 1. * PIXEL_METER;

        commands.entity(entity).insert(ProjecileDisplayBundle {
            play_scene_tag: PlaySceneTag,
            transform: Transform::from_translation(translation),
            transform_interpolation: TransformInterpolation,
            sprite: Sprite::from_image(asset_server.load("projectile-16x8.png")),
            iso_z: IsoZ(1.),
            z_layer: ZLayer::Default,
        });

        commands
            .entity(entity)
            .insert_if_new(ProjectileBundle::from_protocol());
    }
}

fn handle_removed_projectile(
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

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_new_projectile, handle_removed_projectile).run_if(in_state(AppState::Play)),
        );
    }
}
