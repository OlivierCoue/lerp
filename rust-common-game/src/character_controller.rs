use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

// Basic CharacterController taken from https://github.com/Jondolf/avian/blob/main/crates/avian2d/examples/kinematic_character_2d/plugin.rs
// With a few update to addpat it for a top down game, the example was for a platformer with only x axes movement / jump / gravity...
// Also adapted it to handle CharacterController x CharacterController collisions

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // Run collision handling after collision detection.
            //
            // NOTE: The collision implementation here is very basic and a bit buggy.
            //       A collide-and-slide algorithm would likely work better.
            PostProcessCollisions,
            kinematic_controller_collisions,
        );
    }
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Res<Collisions>,
    bodies: Query<&RigidBody>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut character_controllers: Query<
        (&mut Position, &Rotation, &mut LinearVelocity),
        (With<RigidBody>, With<CharacterController>),
    >,
    time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.

        if let Ok((position, rotation, linear_velocity)) =
            character_controllers.get_mut(collider_parent1.get())
        {
            let is_first = true;
            let character_rb = *bodies.get(collider_parent1.get()).unwrap();
            let is_other_dynamic = bodies
                .get(collider_parent2.get())
                .is_ok_and(|rb| rb.is_dynamic());
            handle_collision(
                character_rb,
                contacts,
                is_first,
                is_other_dynamic,
                position,
                rotation,
                linear_velocity,
                time.delta_seconds(),
            );
        };

        if let Ok((position, rotation, linear_velocity)) =
            character_controllers.get_mut(collider_parent2.get())
        {
            let is_first = false;
            let character_rb = *bodies.get(collider_parent2.get()).unwrap();
            let is_other_dynamic = bodies
                .get(collider_parent1.get())
                .is_ok_and(|rb| rb.is_dynamic());
            handle_collision(
                character_rb,
                contacts,
                is_first,
                is_other_dynamic,
                position,
                rotation,
                linear_velocity,
                time.delta_seconds(),
            );
        };
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_collision(
    character_rb: RigidBody,
    contacts: &Contacts,
    is_first: bool,
    is_other_dynamic: bool,
    mut position: Mut<'_, Position>,
    rotation: &Rotation,
    mut linear_velocity: Mut<'_, LinearVelocity>,
    delta_seconds: f32,
) {
    // This system only handles collision response for kinematic character controllers.
    if !character_rb.is_kinematic() {
        return;
    }

    // Iterate through contact manifolds and their contacts.
    // Each contact in a single manifold shares the same contact normal.
    for manifold in contacts.manifolds.iter() {
        let normal = if is_first {
            -manifold.global_normal1(rotation)
        } else {
            -manifold.global_normal2(rotation)
        };

        let mut deepest_penetration: Scalar = Scalar::MIN;

        // Solve each penetrating contact in the manifold.
        for contact in manifold.contacts.iter() {
            if contact.penetration > 0.0 {
                position.0 += normal * contact.penetration;
            }
            deepest_penetration = deepest_penetration.max(contact.penetration);
        }

        // For now, this system only handles velocity corrections for collisions against static geometry.
        if is_other_dynamic {
            continue;
        }

        if deepest_penetration > 0.0 {
            // The character is intersecting an unclimbable object, like a wall.
            // We want the character to slide along the surface, similarly to
            // a collide-and-slide algorithm.

            // Don't apply an impulse if the character is moving away from the surface or is not moving.
            if linear_velocity.dot(normal) >= 0.0 {
                continue;
            }

            // Slide along the surface, rejecting the velocity along the contact normal.
            let impulse = linear_velocity.reject_from_normalized(normal);
            linear_velocity.0 = impulse;
        } else {
            // The character is not yet intersecting the other object,
            // but the narrow phase detected a speculative collision.
            //
            // We need to push back the part of the velocity
            // that would cause penetration within the next frame.

            let normal_speed = linear_velocity.dot(normal);

            // Don't apply an impulse if the character is moving away from the surface or is not moving.
            if normal_speed >= 0.0 {
                continue;
            }

            // Compute the impulse to apply.
            let impulse_magnitude = normal_speed - (deepest_penetration / delta_seconds);
            let impulse = impulse_magnitude * normal;

            // Avoid
            linear_velocity.0 -= impulse;
        }
    }
}
