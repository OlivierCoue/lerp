use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::skill::SkillInProgress;

use crate::common::cartesian_to_isometric_vec2;

use super::direction::Direction;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AnimationState {
    Walk,
    Idle,
    Attack,
}

#[derive(Component)]
pub struct AnimationConfig {
    pub timer: Timer,
    pub state: AnimationState,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub atlas_texture_walk: Handle<Image>,
    pub atlas_texture_idle: Handle<Image>,
    pub atlas_texture_attack: Handle<Image>,
}

impl AnimationConfig {
    pub fn build(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
        atlas_img_path_walk: &str,
        atlas_img_path_idle: &str,
        atlas_img_path_attack: &str,
    ) -> Self {
        let layout: TextureAtlasLayout =
            TextureAtlasLayout::from_grid(UVec2::splat(256), 8, 16, None, None);
        let atlas_layout = texture_atlas_layouts.add(layout.clone());

        let walk_texture: Handle<Image> = asset_server.load(atlas_img_path_walk);

        let idle_texture: Handle<Image> = asset_server.load(atlas_img_path_idle);

        let attack_texture: Handle<Image> = asset_server.load(atlas_img_path_attack);

        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            state: AnimationState::Idle,
            atlas_layout,

            atlas_texture_walk: walk_texture,

            atlas_texture_idle: idle_texture,

            atlas_texture_attack: attack_texture,
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    query_parent: Query<
        (
            &LinearVelocity,
            &Direction,
            Option<&SkillInProgress>,
            &Children,
        ),
        Without<AnimationConfig>,
    >,
    mut query_child: Query<(&mut Sprite, &mut AnimationConfig, &Parent), With<AnimationConfig>>,
) {
    for (mut sprite, mut animation_config, parent) in &mut query_child {
        animation_config.timer.tick(time.delta());

        let Ok((velocity, direction, skill_in_progress, _)) = query_parent.get(parent.get()) else {
            continue;
        };

        let renderered_velocity = cartesian_to_isometric_vec2(&velocity.0);
        let is_walking = renderered_velocity.length_squared() != 0.0;

        let new_state = if let Some(_skill_in_progress) = skill_in_progress {
            AnimationState::Attack
        } else if is_walking {
            AnimationState::Walk
        } else {
            AnimationState::Idle
        };

        let mut state_changed = false;
        if new_state != animation_config.state {
            animation_config.state = new_state;
            state_changed = true;
        }

        if state_changed {
            match new_state {
                AnimationState::Attack => {
                    sprite.image = animation_config.atlas_texture_attack.clone();
                    let atlas = sprite.texture_atlas.as_mut().unwrap();
                    atlas.index = 0;
                }
                AnimationState::Walk => {
                    sprite.image = animation_config.atlas_texture_walk.clone();
                }
                AnimationState::Idle => {
                    sprite.image = animation_config.atlas_texture_idle.clone();
                }
            }
        }

        if animation_config.timer.just_finished() || state_changed {
            let frame_per_anim = 8;

            let atlas = sprite.texture_atlas.as_mut().unwrap();

            let first_frame_index = direction.0 * frame_per_anim;

            atlas.index = if atlas.index % frame_per_anim == frame_per_anim - 1 {
                first_frame_index
            } else {
                first_frame_index + atlas.index % frame_per_anim + 1
            };
        }
    }
}
