use avian2d::prelude::*;
use bevy::prelude::*;

use super::{apply_render_mode, RenderConfig};

#[derive(Component)]
pub struct AnimationConfig {
    pub timer: Timer,
    pub atlas_walk: Handle<TextureAtlasLayout>,
    pub texture_walk: Handle<Image>,
    pub atlas_idle: Handle<TextureAtlasLayout>,
    pub texture_idle: Handle<Image>,
}

impl AnimationConfig {
    pub fn build(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
        atlas_img_path_walk: &str,
        atlas_img_path_idle: &str,
    ) -> Self {
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(256), 8, 16, None, None);

        let player_walk_texture: Handle<Image> = asset_server.load(atlas_img_path_walk);
        let player_walk_texture_atlas_layout = texture_atlas_layouts.add(layout.clone());

        let player_idle_texture: Handle<Image> = asset_server.load(atlas_img_path_idle);
        let player_idle_texture_atlas_layout = texture_atlas_layouts.add(layout.clone());

        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            atlas_walk: player_walk_texture_atlas_layout.clone(),
            texture_walk: player_walk_texture.clone(),
            atlas_idle: player_idle_texture_atlas_layout,
            texture_idle: player_idle_texture,
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn animate_sprite(
    time: Res<Time>,
    render_config: Res<RenderConfig>,
    query_parent: Query<(&LinearVelocity, &Children), Without<AnimationConfig>>,
    mut query_child: Query<(&mut Sprite, &mut AnimationConfig, &Parent), With<AnimationConfig>>,
) {
    for (mut sprite, mut animation_config, parent) in &mut query_child {
        let Ok((velocity, _)) = query_parent.get(parent.get()) else {
            continue;
        };

        animation_config.timer.tick(time.delta());

        if animation_config.timer.just_finished() {
            let mut direction_index = None;
            let renderered_velocity = apply_render_mode(&render_config, &velocity.0);

            if renderered_velocity.length_squared() != 0.0 {
                // Calculate the angle in radians and normalize to [0, 2π]
                let angle = renderered_velocity.y.atan2(renderered_velocity.x); // atan2(y, x) gives the angle relative to the X-axis
                let mut angle_deg = angle.to_degrees(); // Convert to degrees
                angle_deg = (angle_deg + 180.0).rem_euclid(360.0); // Normalize negative angles to [0, 360]

                let adjusted_angle = 360. - ((angle_deg + 270.) % 360.0);

                // Map the adjusted angle to one of 16 directions
                let sector_size = 360.0 / 16.0; // Each direction covers 22.5 degrees
                direction_index = Some(
                    ((adjusted_angle + (sector_size / 2.0)) / sector_size).floor() as usize % 16,
                );
            }

            let frame_per_anim = 8;

            if let Some(direction_index) = direction_index {
                if sprite.image != animation_config.texture_walk {
                    sprite.image = animation_config.texture_walk.clone();
                }

                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.layout != animation_config.atlas_walk {
                        atlas.layout = animation_config.atlas_walk.clone();
                        atlas.index = 0;
                    }

                    let first_frame_index = direction_index * frame_per_anim;
                    atlas.index = if atlas.index % frame_per_anim == frame_per_anim - 1 {
                        first_frame_index
                    } else {
                        first_frame_index + atlas.index % frame_per_anim + 1
                    };
                }
            } else {
                if sprite.image != animation_config.texture_idle {
                    sprite.image = animation_config.texture_idle.clone();
                }

                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.layout != animation_config.atlas_idle {
                        atlas.layout = animation_config.atlas_idle.clone();
                    }

                    if atlas.index % frame_per_anim != 0 {
                        atlas.index = atlas.index - atlas.index % frame_per_anim
                    } else {
                        atlas.index += 1
                    }
                }
            }
        }
    }
}
