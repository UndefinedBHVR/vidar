//! A kinematic character controller framework inspired by the [bevy-tnua](https://github.com/idanarye/bevy-tnua/tree/main) project
//! While also taking inspiration and ideas from the [Avian Physics](https://discord.com/channels/691052431525675048/1124043933886976171) channel in the official Bevy Discord server.\
//!
//! Please note that all components within this module are prefixed with `KCC` to make it clear that
//! they are part of the Kinematic Character Controller framework.
use avian3d::prelude::*;
use bevy::prelude::*;

mod move_and_slide;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (
            move_and_slide::collide_and_slide_system,
            update_kinematic_character_controller,
            update_kinematic_floor,
            update_kinematic_grounding,
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
            ground_direction: Vec3::Y,
            floor_collider: Collider::capsule(0.4, 0.8),
            floor_distance: 0.0,
            max_floor_distance: 0.1,
        }
    }
}

/// A component that when added to the controller enables snapping to the floor.
/// This component requires the [`KCCFloorDetection`] and the [`KCCGrounded`] components to be
/// present on the same entity.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct KCCFloorSnap;

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
    mut query: Query<(&mut KCCFloorDetection, &Transform, Option<&mut KCCGrounded>)>,
    spatial_query: SpatialQuery,
) {
    for (mut floor_detection, transform, grounded) in query.iter_mut() {
        floor_detection.prev_floor_normal = floor_detection.floor_normal;
        if let Some(mut grounded) = grounded {
            grounded.prev_grounded = grounded.grounded;
        }

        let Some(cast) = spatial_query.cast_shape(
            &floor_detection.floor_collider,
            transform.translation,
            Quat::IDENTITY,
            Dir3::new_unchecked(floor_detection.ground_direction.normalize()),
            floor_detection.max_floor_distance,
            true,
            SpatialQueryFilter::default(),
        ) else {
            // Nothing was hit, move on.
            continue;
        };

        floor_detection.floor_normal = cast.normal1;
        floor_detection.floor_distance = cast.time_of_impact;
    }
}

pub fn update_kinematic_grounding(mut query: Query<(&mut KCCGrounded, &KCCFloorDetection)>) {
    for (mut grounded, floor_detection) in query.iter_mut() {
        grounded.prev_grounded = grounded.grounded;
        grounded.grounded = floor_detection.floor_normal != Vec3::ZERO;
    }
}
