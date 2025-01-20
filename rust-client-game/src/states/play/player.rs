use crate::states::play::*;
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
            "assets/atlas_player_walk.png",
            "assets/atlas_player_idle.png",
            "assets/atlas_player_attack.png",
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
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite::from_atlas_image(
                        animation_config.atlas_texture_idle.clone(),
                        TextureAtlas {
                            layout: animation_config.atlas_layout.clone(),
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
        RenderMode::Iso => isometric_to_cartesian(
            world_cursor_position.x,
            world_cursor_position.y - 1. * PIXEL_METER,
        ),
        RenderMode::Cart => world_cursor_position,
    };

    let Ok(mut action_state) = action_state_query.get_single_mut() else {
        return;
    };

    action_state.set_axis_pair(&PlayerActions::Cursor, actual_world_cursor_position);
}
