//! A kinematic character controller framework inspired by the [bevy-tnua](https://github.com/idanarye/bevy-tnua/tree/main) project
//! While also taking inspiration and ideas from the [Avian Physics](https://discord.com/channels/691052431525675048/1124043933886976171) channel in the official Bevy Discord server.\
//!
//! Please note that all components within this module are prefixed with `KCC` to make it clear that
//! they are part of the Kinematic Character Controller framework.

use avian3d::prelude::*;
use bevy::prelude::*;

mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (
            movement::gravity_system,
            movement::collide_and_slide_system,
            update_kinematic_character_controller,
            update_kinematic_floor,
            floor_snap,
        )
            .chain(),
    );
}

/// A component that represents the core logic of a kinematic character controller.
/// This component has a dedicated system that updates its internal state and calls the movement
/// basis.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct KinematicCharacterController {
    /// The velocity we had last tick.
    pub prev_velocity: Vec3,
    /// The velocity we have this tick.
    pub velocity: Vec3,
    /// The up vector of the character.
    pub up: Vec3,
    /// How many times the collider will "bounce" off of surfaces.
    pub bounces: u32,
    /// The collider that represents the shape of this character.
    #[reflect(ignore)]
    pub collider: Collider,
}

impl Default for KinematicCharacterController {
    fn default() -> Self {
        Self {
            prev_velocity: Vec3::ZERO,
            velocity: Vec3::ZERO,
            up: Vec3::Y,
            bounces: 4,
            collider: Collider::capsule(0.4, 0.8),
        }
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
/// A component that when added to the controller enables grounding management.
/// This component requires the [`KCCFloorDetection`] component to be present on the same entity.
pub struct KCCGrounded {
    /// Is this character currently grounded?
    pub grounded: bool,
    /// Was this character grounded last tick?
    pub prev_grounded: bool,
}

/// Component that represents the floor detection of a kinematic character controller.
/// This component has a dedicated system that runs a shapecast to detect the floor.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct KCCFloorDetection {
    /// [`Vec3`] representing the normal of the floor we were on last tick.
    /// [`Vec3::ZERO`] if we are not grounded.
    pub prev_floor_normal: Vec3,
    /// [`Vec3`] representing the normal of the floor we are currently standing on.
    /// [`Vec3::ZERO`] if we are not grounded.
    pub floor_normal: Vec3,
    /// Direction that gravity is pulling this character in
    pub ground_direction: Vec3,
    #[reflect(ignore)]
    pub floor_collider: Collider,
    /// The distance from the floor that this character is currently at.
    pub floor_distance: f32,
    /// How far from the floor this character can be before it is considered not grounded.
    pub max_floor_distance: f32,
}

impl Default for KCCFloorDetection {
    fn default() -> Self {
        Self {
            prev_floor_normal: Vec3::ZERO,
            floor_normal: Vec3::ZERO,
            ground_direction: Vec3::NEG_Y,
            floor_collider: Collider::capsule(0.4, 0.8),
            floor_distance: 0.0,
            max_floor_distance: 0.05,
        }
    }
}

/// A component that when added to the controller enables snapping to the floor.
/// This component requires the [`KCCFloorDetection`] and the [`KCCGrounded`] components to be
/// present on the same entity.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct KCCFloorSnap;

/// Component that handles gravity for a kinematic character controller
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct KCCGravity {
    /// The maximum velocity the character can reach when falling
    pub terminal_velocity: f32,
    /// The acceleration factor (9.81 on Earth)
    pub acceleration_factor: f32,
    /// Current velocity from gravity
    pub current_velocity: Vec3,
    /// Direction of gravity
    pub direction: Vec3,
}

impl Default for KCCGravity {
    fn default() -> Self {
        Self {
            terminal_velocity: 53.0, // ~terminal velocity for human
            acceleration_factor: 9.81 * 2.0,
            current_velocity: Vec3::ZERO,
            direction: Vec3::NEG_Y,
        }
    }
}

/// Component that controls how the character handles slopes
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct KCCSlope {
    /// Maximum angle in radians that the character can walk up
    pub max_slope_angle: f32,
    /// Friction coefficient applied when on slopes
    pub friction: f32,
}

impl Default for KCCSlope {
    fn default() -> Self {
        Self {
            max_slope_angle: 80.0_f32.to_radians(),
            friction: 0.8,
        }
    }
}

/// Function that updates the kinematic character controller's internal state. Currently, this only
/// updates the previous velocity.
pub fn update_kinematic_character_controller(
    mut query: Query<(&mut KinematicCharacterController, &mut LinearVelocity)>,
) {
    for (mut controller, _) in query.iter_mut() {
        controller.prev_velocity = controller.velocity;
        //linear_velocity.0 = controller.velocity;
    }
}

pub fn update_kinematic_floor(
    mut query: Query<(&mut KCCFloorDetection, &Transform, Option<&mut KCCGrounded>, Entity)>,
    spatial_query: SpatialQuery,
) {
    for (mut floor_detection, transform, mut grounded, entity) in query.iter_mut() {
        floor_detection.prev_floor_normal = floor_detection.floor_normal;
        if let Some(grounded) = grounded.as_mut() {
            grounded.prev_grounded = grounded.grounded;
        }

        let Some(cast) = spatial_query.cast_shape(
            &floor_detection.floor_collider,
            transform.translation,
            Quat::IDENTITY,
            Dir3::new_unchecked(floor_detection.ground_direction.normalize()),
            floor_detection.max_floor_distance,
            false,
            &SpatialQueryFilter::default().with_excluded_entities([entity]),
        ) else {
            // Nothing was hit, move on.
            continue;
        };

        floor_detection.floor_normal = cast.normal1;
        floor_detection.floor_distance = cast.time_of_impact;
        if let Some(grounded) = grounded.as_mut() {
            grounded.grounded = true;
        }
    }
}

pub fn floor_snap(
    mut query: Query<(
        &mut Transform,
        &KCCFloorDetection,
        &KCCGrounded,
        Option<&KCCFloorSnap>,
        &KinematicCharacterController,
    )>,
) {
    for (mut transform, floor_detection, grounded, _, controller) in query.iter_mut() {
        if (grounded.grounded || grounded.prev_grounded)
            && controller.velocity.y <= 0.0
            && floor_detection.floor_distance < 0.01
        {
            transform.translation.y -= floor_detection.floor_distance - 0.001;
        }
    }
}
