use avian3d::{
    math::AdjustPrecision,
    prelude::*,
};
use bevy::prelude::*;

use super::KinematicCharacterController;

fn project_onto_plane(velocity: Vec3, plane: Vec3) -> Vec3 {
    velocity - plane * velocity.dot(plane)
}

/// This system is used to run the recursive_collide_and_slide function for our kinematic character
/// controllers.
pub fn collide_and_slide_system(
    mut character_controllers: Query<
        (&mut Transform, &Collider, Entity, &mut KinematicCharacterController),
        With<RigidBody>,
    >,
    mut spatial_query: SpatialQuery,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_seconds_f64().adjust_precision();

    // Iterate over all character controllers and run the recursive_collide_and_slide function.
    for (mut transform, collider, entity, mut character_controller) in &mut character_controllers {
        let velocity = character_controller.velocity * delta_seconds;

        // Filter out ourself from the spatial query.
        let mut filter = SpatialQueryFilter::default().with_excluded_entities([entity]);

        // This algorithm keeps a list of planes for the function in order to prevent a "crushing"
        // effect where tight corridors can cause the character to get stuck or otherwise
        // forced into the ground

        collide_and_slide(&mut spatial_query, &filter, &mut character_controller, &mut transform, &time);

        // Move us to the new position
        //transform.translation += translation;

        // Update the velocity
        //character_controller.velocity = translation / delta_seconds;
    }
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This function handles the collision response for kinematic character controllers.
/// Its based upon the collide-and-slide algorithm, which is a common approach for
/// handling collisions with kinematic bodies.
///
/// This specific implementation is based primarily on [Improved Collision detection and Response](https://www.peroxide.dk/papers/collision/collision.pdf).
/// by Kasper Fauerby.
fn recursive_collide_and_slide(
    spatial_query: &mut spatial_query::SpatialQuery,
    filter: &spatial_query::SpatialQueryFilter,
    collider: &Collider,
    transform: &mut Transform,
    max_depth: usize,
    velocity: Vec3,
    padding: f32,
    planes: &mut Vec<Vec3>,
) -> Vec3 {
    if max_depth == 0 || velocity.length_squared() < 0.00001 || !velocity.is_finite() {
        return Vec3::ZERO;
    }

    let (velocity_normalized, length) = Dir3::new_and_length(velocity).unwrap();

    let hit = match spatial_query.cast_shape(
        collider,
        transform.translation,
        transform.rotation,
        velocity_normalized,
        length,
        true,
        filter.clone(),
    ) {
        Some(result) => result,
        None => return velocity,
    };

    if (hit.time_of_impact - padding).abs() > 0.01 {
        planes.clear();
    }

    planes.push(hit.normal1);
    let distance = hit.time_of_impact;
    // Update our position to be at the hit point.
    transform.translation += velocity_normalized * distance;
    let surface_point = velocity * (hit.time_of_impact - padding).max(0.0);
    let mut projected_velocity = project_onto_plane(velocity, surface_point);

    // Plane solving logic
    if planes.len() > 1 {
        for (plane, next_plane) in
            planes.iter().zip(planes.iter().cycle().skip(1)).take(planes.len())
        {
            let crease = plane.cross(*next_plane);
            projected_velocity = crease * crease.dot(projected_velocity);
        }
    }
    if projected_velocity.dot(velocity) <= 0.0 {
        return Vec3::ZERO;
    }

    surface_point
        + recursive_collide_and_slide(
            spatial_query,
            filter,
            collider,
            transform,
            max_depth - 1,
            projected_velocity,
            padding,
            planes,
        )
}

/// Implementation of the source engine's CBaseEntity::PhysicsTryMove function.
fn collide_and_slide(
    spatial_query: &mut spatial_query::SpatialQuery,
    filter: &spatial_query::SpatialQueryFilter,
    kinematic_controller: &mut KinematicCharacterController,
    transform: &mut Transform,
    time: &Res<Time>,
) {
    const EPSILON: f32 = 0.01; // Small padding value
    let primal_velocity =
        kinematic_controller.velocity * time.delta_seconds_f64().adjust_precision();
    let mut velocity = primal_velocity;
    let max_bumps = 6;
    let mut planes = Vec::new();
    for _ in 0..max_bumps {
        if !velocity.is_finite() {
            velocity = Vec3::ZERO;
            break;
        }

        if velocity.length_squared() <= EPSILON * EPSILON {
            break;
        }

        // Do a shapecast forward to see if we hit anything.
        let (velocity_normalized, length) = Dir3::new_and_length(velocity).unwrap();
        let hit = match spatial_query.cast_shape(
            &kinematic_controller.collider,
            transform.translation,
            transform.rotation,
            velocity_normalized,
            length,
            false,
            filter.clone(),
        ) {
            Some(result) => result,
            None => {
                break;
            }
        };

        planes.push(hit.normal1);
        velocity = project_onto_plane(velocity, hit.normal1);
        if planes.len() > 1 && planes.len() < 4 {
            for (plane, next_plane) in
                planes.iter().zip(planes.iter().cycle().skip(1)).take(planes.len())
            {
                let crease = plane.cross(*next_plane);
                velocity = crease * velocity.dot(crease);
            }
        }
    }

    kinematic_controller.velocity = velocity / time.delta_seconds_f64().adjust_precision();
    transform.translation += velocity;
}
