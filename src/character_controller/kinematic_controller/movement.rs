//! # Character Controller Module
//!
//! This module provides a robust kinematic character controller implementation
//! for 3D environments, handling collision detection, sliding, and gravity.
//!
//! ## Features
//!
//! - Multi-pass collision detection and response
//! - Slope-aware movement
//! - Configurable gravity with terminal velocity
//! - Efficient depenetration system
//!
//! ## Systems
//!
//! - `collide_and_slide_system`: Main movement and collision response system
//! - `gravity_system`: Handles gravity application with terminal velocity

use avian3d::{math::AdjustPrecision, prelude::*};
use bevy::prelude::*;

use super::{KCCFloorDetection, KCCGravity, KCCSlope, KinematicCharacterController};

// Movement configuration constants
const MAX_BUMPS: u32 = 4;
const MIN_MOVEMENT: f32 = 0.0001;
const COLLISION_EPSILON: f32 = 0.01;
const DEPENETRATION_EPSILON: f32 = 0.01;

/// Result of a movement calculation iteration
#[derive(Debug)]
struct MovementResult {
    movement: Vec3,
    remaining_velocity: Vec3,
    hit_normal: Option<Vec3>,
}

/// Main system for handling character movement and collision response
///
/// Processes both horizontal movement and gravity in separate passes to ensure
/// proper collision response in all scenarios.
#[allow(clippy::too_many_arguments)]
pub fn collide_and_slide_system(
    mut query: Query<(
        &mut Transform,
        Entity,
        &mut KinematicCharacterController,
        Option<&KCCSlope>,
        Option<&KCCFloorDetection>,
        Option<&mut KCCGravity>,
    ), With<RigidBody>>,
    mut spatial_query: SpatialQuery,
    time: Res<Time>,
) {
    let delta = time.delta_seconds_f64().adjust_precision();

    for (mut transform, entity, mut controller, slope, floor_detection, gravity) in &mut query {
        let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);

        // Process horizontal movement
        let movement = process_movement(
            &mut spatial_query,
            &filter,
            &controller,
            &mut transform,
            controller.velocity * delta,
            slope,
            floor_detection,
            false,
        );

        controller.velocity = movement.remaining_velocity / delta;

        // Process gravity separately if enabled
        if let Some(mut gravity) = gravity {
            let movement = process_movement(
                &mut spatial_query,
                &filter,
                &controller,
                &mut transform,
                gravity.current_velocity * delta,
                slope,
                floor_detection,
                true,
            );

            // Update gravity velocity based on collision results
            if movement.hit_normal.is_some() {
                gravity.current_velocity = movement.remaining_velocity / delta;
            }
        }

        // Perform depenetration
        depenetrate(&mut spatial_query, &filter, &controller.collider, &mut transform);
    }
}

/// Core movement processing function that handles collision detection and response
///
/// Returns a `MovementResult` containing the actual movement performed and any
/// remaining velocity that couldn't be applied due to collisions.
fn process_movement(
    spatial_query: &mut spatial_query::SpatialQuery,
    filter: &spatial_query::SpatialQueryFilter,
    controller: &KinematicCharacterController,
    transform: &mut Transform,
    velocity: Vec3,
    slope: Option<&KCCSlope>,
    floor_detection: Option<&KCCFloorDetection>,
    is_gravity_pass: bool,
) -> MovementResult {
    if velocity.length_squared() < MIN_MOVEMENT {
        return MovementResult {
            movement: Vec3::ZERO,
            remaining_velocity: Vec3::ZERO,
            hit_normal: None,
        };
    }

    let mut total_movement = Vec3::ZERO;
    let mut current_velocity = velocity;
    let mut collision_planes = Vec::with_capacity(MAX_BUMPS as usize);
    let mut last_hit_normal = None;

    for _ in 0..MAX_BUMPS {
        if current_velocity.length_squared() < MIN_MOVEMENT {
            break;
        }

        let (velocity_dir, length) = match Dir3::new_and_length(current_velocity) {
            Ok(v) => v,
            Err(_) => break,
        };

        match spatial_query.cast_shape(
            &controller.collider,
            transform.translation,
            transform.rotation,
            velocity_dir,
            length,
            false,
            filter,
        ) {
            Some(hit) => {
                let safe_distance = (hit.time_of_impact - COLLISION_EPSILON).max(0.0);
                let safe_movement = current_velocity * safe_distance;

                transform.translation += safe_movement;
                total_movement += safe_movement;
                current_velocity -= safe_movement;
                last_hit_normal = Some(hit.normal1);

                if is_gravity_pass && should_stop_on_slope(slope, floor_detection, hit.normal1) {
                    break;
                }

                current_velocity = calculate_sliding_velocity(&mut collision_planes, hit.normal1, current_velocity);
            }
            None => {
                transform.translation += current_velocity;
                total_movement += current_velocity;
                current_velocity = Vec3::ZERO;
                break;
            }
        }
    }

    MovementResult {
        movement: total_movement,
        remaining_velocity: current_velocity,
        hit_normal: last_hit_normal,
    }
}

#[inline]
fn should_stop_on_slope(slope: Option<&KCCSlope>, floor_detection: Option<&KCCFloorDetection>, normal: Vec3) -> bool {
    match (slope, floor_detection) {
        (Some(slope), Some(_)) => normal.angle_between(Vec3::Y) < slope.max_slope_angle,
        _ => true,
    }
}

/// Calculates sliding velocity along collision planes
#[inline]
fn calculate_sliding_velocity(planes: &mut Vec<Vec3>, normal: Vec3, velocity: Vec3) -> Vec3 {
    planes.push(normal);
    let mut result = velocity.reject_from(normal);

    // Handle multiple collision planes
    if planes.len() > 1 {
        result = planes.windows(2)
            .fold(result, |acc, plane_pair| {
                acc.project_onto(plane_pair[0].cross(plane_pair[1]))
            });
    }

    result
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
        let push_out_distance = hit.time_of_impact + DEPENETRATION_EPSILON;
        transform.translation += hit.normal1 * push_out_distance;
    }
}

/// Optimized gravity system with terminal velocity handling
pub fn gravity_system(
    mut query: Query<(&KinematicCharacterController, &mut KCCGravity)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (_, mut gravity) in query.iter_mut() {
        let current_speed = gravity.current_velocity.length();
        if current_speed >= gravity.terminal_velocity {
            // Decelerate to terminal velocity
            gravity.current_velocity *= 0.99;
            continue;
        }

        let delta_velocity = gravity.direction * gravity.acceleration_factor * dt;
        let new_velocity = gravity.current_velocity + delta_velocity;

        gravity.current_velocity = if new_velocity.length() > gravity.terminal_velocity {
            new_velocity.normalize() * gravity.terminal_velocity
        } else {
            new_velocity
        };
    }
}
