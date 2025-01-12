use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use rust_common_game::{
    projectile::*,
    protocol::REPLICATION_GROUP,
    shared::{PIXEL_METER, PROJECTILE_BASE_MOVEMENT_SPEED, PROJECTILE_SIZE},
};

#[derive(Event)]
pub struct SpawnProjectileEvent {
    pub from_position: Vec2,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct ProjectileData {
    pub max_distance: f32,
    pub distance_traveled: f32,
}

#[derive(Component)]
pub struct PreviousPosition(pub Vec2);

pub fn on_spawn_projectile_event(
    mut commands: Commands,
    mut spawn_projectile_events: EventReader<SpawnProjectileEvent>,
) {
    for event in spawn_projectile_events.read() {
        let velocity = event.direction * PROJECTILE_BASE_MOVEMENT_SPEED;

        commands.spawn((
            Position::from_xy(event.from_position.x, event.from_position.y),
            Projectile,
            ProjectileData {
                max_distance: 10. * PIXEL_METER,
                distance_traveled: 0.,
            },
            RigidBody::Kinematic,
            Collider::circle(PROJECTILE_SIZE / 2.),
            LockedAxes::ROTATION_LOCKED,
            PreviousPosition(event.from_position),
            LinearVelocity(velocity),
            Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::None,
                    interpolation: NetworkTarget::All,
                },
                target: ReplicationTarget {
                    target: NetworkTarget::All,
                },
                controlled_by: ControlledBy {
                    target: NetworkTarget::None,
                    ..default()
                },
                group: REPLICATION_GROUP,
                ..default()
            },
        ));
    }
}

pub fn update_and_despawn_projectile(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut PreviousPosition,
            &mut Position,
            &mut ProjectileData,
        ),
        With<Projectile>,
    >,
) {
    for (entity, mut previous_position, current_position, mut projectile_data) in query.iter_mut() {
        let distance_traveled = previous_position.0.distance(current_position.0);
        projectile_data.distance_traveled += distance_traveled;

        if projectile_data.distance_traveled >= projectile_data.max_distance {
            commands.entity(entity).despawn_recursive();
        } else {
            previous_position.0 = current_position.0;
        }
    }
}
