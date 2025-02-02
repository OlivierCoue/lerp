use std::str::FromStr;

use crate::{states::play::*, utils::ZLayer};
use animation::AnimationConfig;
use avian2d::prelude::*;
use bevy::{prelude::*, sprite::Anchor};
use lightyear::prelude::PreSpawnedPlayerObject;
use rust_common_game::prelude::*;

use super::{
    animation::AtlasConfigInput,
    direction::{Direction, DirectionCount},
};

#[derive(Component)]
pub struct CharacterRender;

// Body
#[derive(Component)]
pub struct CharacterBodyRenderRef(pub Entity);

#[derive(Component)]
pub struct CharacterBodyRender;

#[derive(Bundle)]
pub struct CharacterBodyRenderBundle {
    pub marker: CharacterBodyRender,
    pub sprite: Sprite,
    pub animation_config: AnimationConfig,
}

// Coprs
#[derive(Component)]
pub struct CharacterCorpsRenderRef(pub Entity);

#[derive(Component)]
pub struct CharacterCropsRender;

#[derive(Bundle)]
pub struct CharacterCorpsRenderBundle {
    pub marker: CharacterCropsRender,
    pub sprite: Sprite,
}

pub fn on_enemy(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Position),
        (
            Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>,
            With<Enemy>,
            Without<CharacterRender>,
        ),
    >,
) {
    for (entity, position) in player_query.iter_mut() {
        commands.entity(entity).insert((
            CharacterRender,
            PlaySceneTag,
            TransformInterpolation,
            Transform::from_translation(cartesian_to_isometric_vec2(&position.0).extend(Z_DEFAULT)),
            Visibility::default(),
            ZLayer::Default,
            DirectionCount(8),
        ));
    }
}

fn get_enemy_animation_config(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> AnimationConfig {
    AnimationConfig::build(
        asset_server,
        texture_atlas_layouts,
        AtlasConfigInput {
            repeated: true,
            frame_count: 14,
            atlas_layout: TextureAtlasLayout::from_grid(UVec2::new(123, 111), 14, 8, None, None),
            image_path: String::from_str("assets/atlas_enemy_walk_123x111.png").unwrap(),
        },
        AtlasConfigInput {
            repeated: true,
            frame_count: 14,
            atlas_layout: TextureAtlasLayout::from_grid(UVec2::new(123, 111), 14, 8, None, None),
            image_path: String::from_str("assets/atlas_enemy_walk_123x111.png").unwrap(),
        },
        AtlasConfigInput {
            repeated: true,
            frame_count: 14,
            atlas_layout: TextureAtlasLayout::from_grid(UVec2::new(123, 111), 14, 8, None, None),
            image_path: String::from_str("assets/atlas_enemy_walk_123x111.png").unwrap(),
        },
        AtlasConfigInput {
            repeated: false,
            frame_count: 21,
            atlas_layout: TextureAtlasLayout::from_grid(UVec2::new(152, 117), 21, 8, None, None),
            image_path: String::from_str("assets/atlas_enemy_death_152x117.png").unwrap(),
        },
    )
}

pub fn update_enemy_render_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<
        (
            Entity,
            &mut ZLayer,
            Option<&CharacterBodyRenderRef>,
            Option<&CharacterCorpsRenderRef>,
            Option<&Direction>,
            Option<&Alive>,
            Option<&Dead>,
        ),
        (
            With<CharacterRender>,
            Or<(With<Predicted>, With<PreSpawnedPlayerObject>)>,
        ),
    >,
) {
    for (entity, mut z_layer, body_render_ref, corps_render_ref, direction, is_alive, is_dead) in
        query.iter_mut()
    {
        // Determine the new render state
        if is_alive.is_some() {
            if let Some(corps_render_ref) = corps_render_ref {
                commands.entity(entity).remove::<CharacterCorpsRenderRef>();
                commands.entity(corps_render_ref.0).despawn_recursive();
            }

            if body_render_ref.is_some() {
                continue;
            }

            // If enemy is alive, add animated sprite
            let animation_config =
                get_enemy_animation_config(&asset_server, &mut texture_atlas_layouts);

            let character_body_render = commands
                .spawn(CharacterBodyRenderBundle {
                    marker: CharacterBodyRender,
                    sprite: Sprite {
                        image: animation_config.atlas_idle.image_path.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: animation_config.atlas_idle.atlas_layout.clone(),
                            index: 0,
                        }),
                        anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                        ..default()
                    },
                    animation_config,
                })
                .id();

            commands
                .entity(entity)
                .insert(CharacterBodyRenderRef(character_body_render))
                .add_child(character_body_render);
            *z_layer = ZLayer::Default;
        } else if is_dead.is_some() {
            if let Some(body_render_ref) = body_render_ref {
                commands.entity(entity).remove::<CharacterBodyRenderRef>();
                commands.entity(body_render_ref.0).despawn_recursive();
            }

            if corps_render_ref.is_some() {
                continue;
            }

            // If enemy is dead, replace with static corpse sprite
            let index = direction.map_or(0, |d| d.0);
            let character_corps_render = commands
                .spawn(CharacterCorpsRenderBundle {
                    marker: CharacterCropsRender,
                    sprite: Sprite {
                        image: asset_server.load("assets/atlas_enemy_corps_157x117.png"),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                                UVec2::new(152, 117),
                                1,
                                8,
                                None,
                                None,
                            )),

                            index,
                        }),
                        anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                        ..default()
                    },
                })
                .id();

            commands
                .entity(entity)
                .insert(CharacterCorpsRenderRef(character_corps_render))
                .add_child(character_corps_render);
            *z_layer = ZLayer::OnFloor;
        }
    }
}
