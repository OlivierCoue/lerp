use crate::states::play::*;
use avian2d::collision::Collider;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::shared::replication::components::Controlled;
use rust_common_game::character_controller::*;
use rust_common_game::protocol::*;
use rust_common_game::shared::*;

#[derive(Component)]
pub struct AnimationConfig {
    pub atlas_walk: Handle<TextureAtlasLayout>,
    pub texture_walk: Handle<Image>,
    pub atlas_idle: Handle<TextureAtlasLayout>,
    pub texture_idle: Handle<Image>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

// System create the player
#[allow(clippy::type_complexity)]
pub fn handle_new_player(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_query: Query<
        (Entity, Has<Controlled>, Has<Enemy>),
        (Or<(Added<Predicted>, Added<Interpolated>)>, With<Player>),
    >,
) {
    for (entity, is_controlled, is_enemy) in player_query.iter_mut() {
        println!(
            "[handle_new_player] New Player with id: {}",
            connection.id()
        );

        let layout = TextureAtlasLayout::from_grid(UVec2::splat(256), 8, 16, None, None);

        let player_walk_texture: Handle<Image> = asset_server.load("assets/atlas_player_walk.png");
        let player_walk_texture_atlas_layout = texture_atlas_layouts.add(layout.clone());

        let player_idle_texture: Handle<Image> = asset_server.load("assets/atlas_player_idle.png");
        let player_idle_texture_atlas_layout = texture_atlas_layouts.add(layout.clone());

        let animation_config = AnimationConfig {
            atlas_walk: player_walk_texture_atlas_layout.clone(),
            texture_walk: player_walk_texture.clone(),
            atlas_idle: player_idle_texture_atlas_layout,
            texture_idle: player_idle_texture,
        };

        let mut collider = if is_enemy {
            Collider::circle(ENTITY_SIZE / 2.0 / 2.)
        } else {
            Collider::circle(ENTITY_SIZE / 2.)
        };
        collider.set_scale(Vec2::new(1., 1.), 10);

        commands
            .entity(entity)
            .insert((
                PlaySceneTag,
                RigidBody::Kinematic,
                CharacterController,
                collider,
                LockedAxes::ROTATION_LOCKED,
                TransformInterpolation,
                Transform::from_xyz(0., 0., 1.),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite::from_atlas_image(
                        player_walk_texture,
                        TextureAtlas {
                            layout: player_walk_texture_atlas_layout,
                            index: 0,
                        },
                    ),
                    animation_config,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    Transform::from_scale(Vec3::new(2., 2., 0.)),
                ));
            });

        if is_controlled {
            commands
                .entity(entity)
                .insert((InputMap::new([(PlayerActions::Stop, KeyCode::KeyS)])
                    .with(PlayerActions::Move, MouseButton::Left),));
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn animate_sprite(
    time: Res<Time>,
    render_config: Res<RenderConfig>,
    query_parent: Query<(&LinearVelocity, &Children), Without<AnimationTimer>>,
    mut query_child: Query<
        (&mut AnimationTimer, &mut Sprite, &AnimationConfig, &Parent),
        With<AnimationTimer>,
    >,
) {
    for (mut timer, mut sprite, animation_config, parent) in &mut query_child {
        let Ok((velocity, _)) = query_parent.get(parent.get()) else {
            continue;
        };

        timer.tick(time.delta());

        if timer.just_finished() {
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

pub fn movement(
    time: Res<Time<Physics>>,
    mut query: Query<
        (&Position, &mut Targets, &mut LinearVelocity, &MovementSpeed),
        With<Predicted>,
    >,
) {
    for (position, targets, velocity, movement_speed) in &mut query {
        shared_movement_behaviour(&time, position, movement_speed, velocity, targets);
    }
}

#[allow(clippy::type_complexity)]
pub fn sync_cursor_poisition(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    render_config: Res<RenderConfig>,
    mut action_state_query: Query<
        &mut ActionState<PlayerActions>,
        (With<Player>, With<Predicted>, With<Controlled>),
    >,
) {
    let (camera, camera_transform) = camera_query.single();

    let Ok(winodw) = windows.get_single() else {
        return;
    };

    let Some(screen_cursor_position) = winodw.cursor_position() else {
        return;
    };

    let Ok(world_cursor_position) =
        camera.viewport_to_world_2d(camera_transform, screen_cursor_position)
    else {
        return;
    };

    let actual_world_cursor_position = match render_config.mode {
        RenderMode::Iso => isometric_to_cartesian(world_cursor_position.x, world_cursor_position.y),
        RenderMode::Cart => world_cursor_position,
    };

    let Ok(mut action_state) = action_state_query.get_single_mut() else {
        return;
    };

    let action = action_state.dual_axis_data_mut_or_default(&PlayerActions::Cursor);
    action.fixed_update_pair = actual_world_cursor_position;
}

#[allow(clippy::type_complexity)]
pub fn set_player_target(
    mut query: Query<
        (&ActionState<PlayerActions>, &mut Targets),
        (With<Player>, With<Predicted>, With<Controlled>),
    >,
) {
    for (action, mut targets) in query.iter_mut() {
        if action.pressed(&PlayerActions::Move) {
            let Some(cursor_position) = action.dual_axis_data(&PlayerActions::Cursor) else {
                println!("cursor_position not set skipping");
                return;
            };

            *targets = Targets(vec![Vec2::new(
                cursor_position.pair.x,
                cursor_position.pair.y,
            )])
        }
    }
}
