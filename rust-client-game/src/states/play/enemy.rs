use std::str::FromStr;

use crate::{states::play::*, utils::ZLayer};
use animation::AnimationConfig;
use avian2d::prelude::*;
use bevy::{prelude::*, sprite::Anchor};
use lightyear::prelude::PreSpawnedPlayerObject;
use rust_common_game::{death::Dead, enemy::*};

use super::{animation::AtlasConfigInput, direction::DirectionCount};

#[derive(Component)]
pub struct EnemyAliveRender;

pub fn on_enemy_alive(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_query: Query<
        (Entity, &Position),
        (
            Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>,
            With<Enemy>,
            With<EnemyAlive>,
            Without<EnemyAliveRender>,
            Without<Dead>,
        ),
    >,
) {
    for (entity, position) in player_query.iter_mut() {
        let animation_config = AnimationConfig::build(
            &asset_server,
            &mut texture_atlas_layouts,
            AtlasConfigInput {
                repeated: true,
                frame_count: 14,
                atlas_layout: TextureAtlasLayout::from_grid(
                    UVec2::new(123, 111),
                    14,
                    8,
                    None,
                    None,
                ),
                image_path: String::from_str("assets/atlas_enemy_walk_123x111.png").unwrap(),
            },
            AtlasConfigInput {
                repeated: true,
                frame_count: 14,
                atlas_layout: TextureAtlasLayout::from_grid(
                    UVec2::new(123, 111),
                    14,
                    8,
                    None,
                    None,
                ),
                image_path: String::from_str("assets/atlas_enemy_walk_123x111.png").unwrap(),
            },
            AtlasConfigInput {
                repeated: true,
                frame_count: 14,
                atlas_layout: TextureAtlasLayout::from_grid(
                    UVec2::new(123, 111),
                    14,
                    8,
                    None,
                    None,
                ),
                image_path: String::from_str("assets/atlas_enemy_walk_123x111.png").unwrap(),
            },
            AtlasConfigInput {
                repeated: false,
                frame_count: 21,
                atlas_layout: TextureAtlasLayout::from_grid(
                    UVec2::new(152, 117),
                    21,
                    8,
                    None,
                    None,
                ),
                image_path: String::from_str("assets/atlas_enemy_death_152x117.png").unwrap(),
            },
        );

        commands
            .entity(entity)
            .insert((
                PlaySceneTag,
                TransformInterpolation,
                Transform::from_translation(
                    cartesian_to_isometric_vec2(&position.0).extend(Z_DEFAULT),
                ),
                Visibility::default(),
                EnemyAliveRender,
                ZLayer::Default,
                DirectionCount(8),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite {
                        image: animation_config.atlas_idle.image_path.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: animation_config.atlas_idle.atlas_layout.clone(),
                            index: 0,
                        }),
                        anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                        ..default()
                    },
                    animation_config,
                    Transform::from_scale(Vec3::new(1., 1., 0.)),
                ));
            });
    }
}

pub fn on_enemy_death(
    mut dead_q: Query<
        &mut ZLayer,
        (
            Added<Dead>,
            Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>,
        ),
    >,
) {
    for mut z_layer in dead_q.iter_mut() {
        *z_layer = ZLayer::OnFloor;
    }
}
