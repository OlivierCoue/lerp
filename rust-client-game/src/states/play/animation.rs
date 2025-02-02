use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::{death::Dead, skill::SkillInProgress};

use super::direction::Direction;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AnimationState {
    Walk,
    Idle,
    Attack,
    Dead,
}

#[derive(Component)]
pub struct AnimationConfig {
    pub timer: Timer,
    pub state: AnimationState,
    pub atlas_walk: AtlasConfig,
    pub atlas_idle: AtlasConfig,
    pub atlas_attack: AtlasConfig,
    pub atlas_death: AtlasConfig,
}

pub struct AtlasConfigInput {
    pub repeated: bool,
    pub frame_count: usize,
    pub atlas_layout: TextureAtlasLayout,
    pub image_path: String,
}

pub struct AtlasConfig {
    pub repeated: bool,
    pub frame_count: usize,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub image_path: Handle<Image>,
}

impl AnimationConfig {
    pub fn build(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
        atlas_img_path_walk: AtlasConfigInput,
        atlas_img_path_idle: AtlasConfigInput,
        atlas_img_path_attack: AtlasConfigInput,
        atlas_img_path_death: AtlasConfigInput,
    ) -> Self {
        let atlas_walk = AtlasConfig {
            repeated: atlas_img_path_walk.repeated,
            frame_count: atlas_img_path_walk.frame_count,
            atlas_layout: texture_atlas_layouts.add(atlas_img_path_walk.atlas_layout.clone()),
            image_path: asset_server.load(atlas_img_path_walk.image_path.as_str()),
        };

        let atlas_idle = AtlasConfig {
            repeated: atlas_img_path_idle.repeated,
            frame_count: atlas_img_path_idle.frame_count,
            atlas_layout: texture_atlas_layouts.add(atlas_img_path_idle.atlas_layout.clone()),
            image_path: asset_server.load(atlas_img_path_idle.image_path.as_str()),
        };

        let atlas_attack = AtlasConfig {
            repeated: atlas_img_path_attack.repeated,
            frame_count: atlas_img_path_attack.frame_count,
            atlas_layout: texture_atlas_layouts.add(atlas_img_path_attack.atlas_layout.clone()),
            image_path: asset_server.load(atlas_img_path_attack.image_path.as_str()),
        };

        let atlas_death = AtlasConfig {
            repeated: atlas_img_path_death.repeated,
            frame_count: atlas_img_path_death.frame_count,
            atlas_layout: texture_atlas_layouts.add(atlas_img_path_death.atlas_layout.clone()),
            image_path: asset_server.load(atlas_img_path_death.image_path.as_str()),
        };

        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            state: AnimationState::Idle,
            atlas_walk,
            atlas_idle,
            atlas_attack,
            atlas_death,
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    query_parent: Query<
        (
            Option<&LinearVelocity>,
            &Direction,
            Option<&SkillInProgress>,
            Has<Dead>,
            &Children,
        ),
        Without<AnimationConfig>,
    >,
    mut query_child: Query<(&mut Sprite, &mut AnimationConfig, &Parent), With<AnimationConfig>>,
) {
    for (mut sprite, mut animation_config, parent) in &mut query_child {
        animation_config.timer.tick(time.delta());

        let Ok((velocity, direction, skill_in_progress, is_dead, _)) =
            query_parent.get(parent.get())
        else {
            continue;
        };

        let is_walking = velocity.is_some_and(|v| v.length_squared() != 0.0);

        let (new_state, frame_count, repeated) = if is_dead {
            (
                AnimationState::Dead,
                animation_config.atlas_death.frame_count,
                animation_config.atlas_death.repeated,
            )
        } else if let Some(_skill_in_progress) = skill_in_progress {
            (
                AnimationState::Attack,
                animation_config.atlas_attack.frame_count,
                animation_config.atlas_attack.repeated,
            )
        } else if is_walking {
            (
                AnimationState::Walk,
                animation_config.atlas_walk.frame_count,
                animation_config.atlas_walk.repeated,
            )
        } else {
            (
                AnimationState::Idle,
                animation_config.atlas_idle.frame_count,
                animation_config.atlas_idle.repeated,
            )
        };

        let mut state_changed = false;
        if new_state != animation_config.state {
            animation_config.state = new_state;
            state_changed = true;
        }

        if state_changed {
            match new_state {
                AnimationState::Attack => {
                    sprite.image = animation_config.atlas_attack.image_path.clone();
                    let atlas = sprite.texture_atlas.as_mut().unwrap();
                    atlas.layout = animation_config.atlas_attack.atlas_layout.clone();
                    atlas.index = 0;
                }
                AnimationState::Walk => {
                    sprite.image = animation_config.atlas_walk.image_path.clone();
                    let atlas = sprite.texture_atlas.as_mut().unwrap();
                    atlas.layout = animation_config.atlas_walk.atlas_layout.clone();
                }
                AnimationState::Idle => {
                    sprite.image = animation_config.atlas_idle.image_path.clone();
                    let atlas = sprite.texture_atlas.as_mut().unwrap();
                    atlas.layout = animation_config.atlas_idle.atlas_layout.clone();
                }
                AnimationState::Dead => {
                    sprite.image = animation_config.atlas_death.image_path.clone();
                    let atlas = sprite.texture_atlas.as_mut().unwrap();
                    atlas.layout = animation_config.atlas_death.atlas_layout.clone();
                    atlas.index = 0;
                }
            }
        }

        if animation_config.timer.just_finished() || state_changed {
            let atlas = sprite.texture_atlas.as_mut().unwrap();

            let first_frame_index = direction.0 * frame_count;

            let is_last_frame = atlas.index % frame_count == frame_count - 1;

            if is_last_frame && !repeated {
                continue;
            }

            atlas.index = if is_last_frame {
                first_frame_index
            } else {
                first_frame_index + atlas.index % frame_count + 1
            };
        }
    }
}
