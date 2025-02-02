use crate::states::play::animation::AtlasConfigInput;
use crate::states::play::direction::DirectionCount;
use crate::states::play::*;
use crate::utils::ZLayer;
use animation::AnimationConfig;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::shared::replication::components::Controlled;
use rust_common_game::input::PlayerActions;
use rust_common_game::player::PlayerBundle;
use rust_common_game::protocol::*;
use rust_common_game::shared::*;
use rust_common_game::skill::*;
use rust_common_game::utils::isometric_to_cartesian;
use std::str::FromStr;

pub fn handle_new_client(
    mut client_query: Query<
        (Entity, &PlayerClient),
        (Added<Predicted>, With<PlayerClient>, With<Controlled>),
    >,
) {
    for (_, player_client) in client_query.iter_mut() {
        println!(
            "[handle_new_client] New client with id: {}",
            player_client.client_id
        );
    }
}

pub fn handle_new_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_query: Query<(Entity, Has<Controlled>), (Added<Predicted>, With<Player>)>,
    skill_db: Res<SkillDb>,
) {
    for (entity, controlled) in player_query.iter_mut() {
        println!("[handle_new_player] New Player");

        let animation_config = AnimationConfig::build(
            &asset_server,
            &mut texture_atlas_layouts,
            AtlasConfigInput {
                repeated: true,
                frame_count: 8,
                atlas_layout: TextureAtlasLayout::from_grid(UVec2::splat(256), 8, 16, None, None),
                image_path: String::from_str("assets/atlas_player_walk.png").unwrap(),
            },
            AtlasConfigInput {
                repeated: true,
                frame_count: 8,
                atlas_layout: TextureAtlasLayout::from_grid(UVec2::splat(256), 8, 16, None, None),
                image_path: String::from_str("assets/atlas_player_idle.png").unwrap(),
            },
            AtlasConfigInput {
                repeated: true,
                frame_count: 8,
                atlas_layout: TextureAtlasLayout::from_grid(UVec2::splat(256), 8, 16, None, None),
                image_path: String::from_str("assets/atlas_player_attack.png").unwrap(),
            },
            AtlasConfigInput {
                repeated: false,
                frame_count: 8,
                atlas_layout: TextureAtlasLayout::from_grid(UVec2::splat(256), 8, 16, None, None),
                image_path: String::from_str("assets/atlas_player_attack.png").unwrap(),
            },
        );

        let mut player_bundle = PlayerBundle::from_protocol();
        attach_all_skills(
            &mut commands,
            entity,
            &mut player_bundle.skills_available,
            &skill_db,
        );
        commands
            .entity(entity)
            .insert_if_new(player_bundle)
            .insert((
                PlaySceneTag,
                TransformInterpolation,
                Transform::from_xyz(0., 0., 1.),
                Visibility::default(),
                ZLayer::Default,
                DirectionCount(16),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite::from_atlas_image(
                        animation_config.atlas_idle.image_path.clone(),
                        TextureAtlas {
                            layout: animation_config.atlas_idle.atlas_layout.clone(),
                            index: 0,
                        },
                    ),
                    animation_config,
                    Transform::from_scale(Vec3::new(2., 2., 0.)),
                ));
            });

        if controlled {
            commands.entity(entity).insert(
                InputMap::new([
                    (PlayerActions::MoveUp, KeyCode::KeyW),
                    (PlayerActions::MoveDown, KeyCode::KeyS),
                    (PlayerActions::MoveLeft, KeyCode::KeyA),
                    (PlayerActions::MoveRight, KeyCode::KeyD),
                    (PlayerActions::SkillSlot3, KeyCode::KeyE),
                ])
                .with(PlayerActions::SkillSlot1, MouseButton::Left)
                .with(PlayerActions::SkillSlot2, MouseButton::Right),
            );
        }
    }
}

pub fn sync_cursor_poisition(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
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

    let actual_world_cursor_position = isometric_to_cartesian(
        world_cursor_position.x,
        world_cursor_position.y - 1. * PIXEL_METER,
    );

    let Ok(mut action_state) = action_state_query.get_single_mut() else {
        return;
    };

    action_state.set_axis_pair(&PlayerActions::Cursor, actual_world_cursor_position);
}
