use crate::states::play::*;
use animation::AnimationConfig;
use avian2d::collision::Collider;
use avian2d::prelude::*;
use bevy::prelude::*;
use rust_common_game::character_controller::*;
use rust_common_game::protocol::*;
use rust_common_game::shared::*;

// System create the player
#[allow(clippy::type_complexity)]
pub fn handle_new_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_query: Query<Entity, (Added<Interpolated>, With<Enemy>)>,
) {
    for entity in player_query.iter_mut() {
        println!("[handle_new_enemy] New Enemy");

        let animation_config = AnimationConfig::build(
            &asset_server,
            &mut texture_atlas_layouts,
            "assets/atlas_enemy_walk.png",
            "assets/atlas_enemy_idle.png",
        );

        commands
            .entity(entity)
            .insert((
                PlaySceneTag,
                RigidBody::Kinematic,
                CharacterController,
                Collider::circle(ENEMY_SIZE / 2.),
                LockedAxes::ALL_LOCKED,
                TransformInterpolation,
                Transform::from_xyz(0., 0., 1.),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite::from_atlas_image(
                        animation_config.texture_idle.clone(),
                        TextureAtlas {
                            layout: animation_config.atlas_idle.clone(),
                            index: 0,
                        },
                    ),
                    animation_config,
                    Transform::from_scale(Vec3::new(2., 2., 0.)),
                ));
            });
    }
}
