//! # Character Controller Module
//!
//! This module provides functionality for implementing kinematic character controllers
//! in a 3D game environment. It includes systems and functions for collision detection
//! and sliding, allowing characters to move smoothly in complex environments.
//!
//! ## Key Components
//!
//! - `collide_and_slide_system`: A system that handles collision detection and sliding for all
//!   entities with a `KinematicCharacterController` component.
//! - `collide_and_slide`: A function that implements the core logic for collision detection and
//!   sliding, based on the Source engine's approach.
//! - `depenetrate`: A function that implements basic depenetration logic. This is ran after sliding
//!   to prevent the character from penetrating the surface.
//! ## Usage
//!
//! To use this module, add the `collide_and_slide_system` to your game's schedule
//! and ensure that entities intended to use character controller behavior have both
//! `KinematicCharacterController` and `RigidBody` components.
//!
//! ## Dependencies
//!
//! This module relies on the `avian3d` crate for physics operations and interactions.

use avian3d::{
    math::AdjustPrecision,
    prelude::*,
};
use bevy::prelude::*;

use super::KinematicCharacterController;

/// Handles collision detection and sliding for kinematic character controllers.
///
/// # Arguments
/// * `query` - Query for character controllers
/// * `spatial_query` - Spatial query system for collision detection
/// * `time` - Time resource for delta time calculations
pub fn collide_and_slide_system(
    mut query: Query<(&mut Transform, Entity, &mut KinematicCharacterController), With<RigidBody>>,
    mut spatial_query: SpatialQuery,
    time: Res<Time>,
) {
    for (mut transform, entity, mut controller) in &mut query {
        let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);

        collide_and_slide(&mut spatial_query, &filter, &mut controller, &mut transform, &time);

        depenetrate(&mut spatial_query, &filter, &controller.collider, &mut transform);
    }
}

/// Implements collision detection and sliding for a kinematic character controller.
///
/// # Arguments
/// * `spatial_query` - Spatial query system for collision detection
/// * `filter` - Filter to exclude specific entities from collision checks
/// * `controller` - Kinematic character controller to update
/// * `transform` - Transform of the character to update
/// * `time` - Time resource for delta time calculations
fn collide_and_slide(
    spatial_query: &mut spatial_query::SpatialQuery,
    filter: &spatial_query::SpatialQueryFilter,
    kinematic_controller: &mut KinematicCharacterController,
    transform: &mut Transform,
    time: &Res<Time>,
) {
    const EPSILON: f32 = 0.01; // Small padding value to prevent precision issues
    const MAX_BUMPS: u32 = 4; // Maximum number of collision iterations
    let delta_seconds = time.delta_seconds_f64().adjust_precision();
    let mut velocity = kinematic_controller.velocity * delta_seconds;
    let mut planes = Vec::new();

    for bump in 0..MAX_BUMPS {
        if velocity.length_squared() == 0.0 {
            break;
        }

        // Handle edge cases
        if velocity.is_nan() {
            velocity = Vec3::ZERO;
            break;
        }

        if !velocity.is_finite() {
            error!(
                "Failed to run `collide_and_slide`: velocity is not finite, but `{velocity:?}`. Escaped after {bump} bumps.",
            );
            velocity = Vec3::ZERO;
            break;
        }

        let (velocity_normalized, length) = Dir3::new_and_length(velocity).unwrap();
        let hit = spatial_query.cast_shape(
            &kinematic_controller.collider,
            transform.translation,
            transform.rotation,
            velocity_normalized,
            length,
            false,
            filter,
        );

        if let Some(hit) = hit {
            // Move to the safe distance minus padding
            let safe_movement = velocity * (hit.time_of_impact - EPSILON).max(0.0);
            transform.translation += safe_movement;

            // Update velocity
            velocity -= safe_movement;
            planes.push(hit.normal1);
            velocity = velocity.reject_from(hit.normal1);

            // Handle sliding along multiple planes
            if planes.len() > 1 {
                for (plane, next_plane) in
                    planes.iter().zip(planes.iter().cycle().skip(1)).take(planes.len())
                {
                    let crease = plane.cross(*next_plane);
                    velocity = velocity.project_onto(crease);
                }
            }
        } else {
            break;
        }
    }

    // Update the kinematic controller and transform
    kinematic_controller.velocity = velocity / delta_seconds;
    transform.translation += velocity;
}

/// Performs depenetration for a kinematic character controller.
///
/// # Arguments
/// * `spatial_query` - Spatial query system for collision detection
/// * `filter` - Filter to exclude specific entities from collision checks
/// * `collider` - Collider of the character
/// * `transform` - Transform of the character to update
fn depenetrate(
    spatial_query: &mut spatial_query::SpatialQuery,
    filter: &spatial_query::SpatialQueryFilter,
    collider: &Collider,
    transform: &mut Transform,
) {
    const EPSILON: f32 = 0.001;

    let hit = spatial_query.cast_shape(
        collider,
        transform.translation,
        transform.rotation,
        Dir3::NEG_Y,
        0.0,
        false,
        filter,
    );

    if let Some(hit) = hit {
        let push_out_distance = hit.time_of_impact + EPSILON;
        transform.translation += hit.normal1 * push_out_distance;
    }
}
