use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::client::Interpolated;
use rust_common_game::{projectile::Projectile, shared::*};

use super::{apply_render_mode, PlaySceneTag, RenderConfig};

#[derive(Component)]
#[require(
    Projectile,
    PlaySceneTag,
    RigidBody,
    Collider,
    LockedAxes,
    Visibility,
    Transform,
    TransformInterpolation
)]
pub struct RenderProjectile;

#[allow(clippy::type_complexity)]
pub fn handle_new_projectile(
    render_config: Res<RenderConfig>,
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Position), (Added<Interpolated>, With<Projectile>)>,
) {
    for (entity, position) in projectile_query.iter_mut() {
        let translation = apply_render_mode(&render_config, position).extend(1.);
        commands.entity(entity).insert((
            RenderProjectile,
            RigidBody::Kinematic,
            Collider::circle(PROJECTILE_SIZE / 2.),
            LockedAxes::ROTATION_LOCKED,
            Transform::from_translation(translation),
        ));
    }
}
