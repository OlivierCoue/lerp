// #[derive(Component)]
// struct Projectile {
//     max_distance: f32,
//     distance_traveled: f32,
// }

// #[derive(Component)]
// struct Velocity(Vec3);

// fn move_projectiles(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &mut Transform, &mut Projectile, &Velocity)>,
// ) {
//     for (entity, mut transform, mut projectile, velocity) in query.iter_mut() {
//         let delta = velocity.0 * time.delta_seconds();
//         transform.translation += delta;

//         // Update distance traveled
//         projectile.distance_traveled += delta.length();

//         // Despawn if max distance is reached
//         if projectile.distance_traveled >= projectile.max_distance {
//             commands.entity(entity).despawn();
//         }
//     }
// }
