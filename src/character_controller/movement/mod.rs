use avian3d::{
    math::{
        Scalar,
        Vector,
    },
    prelude::{
        RigidBody,
        Rotation,
        ShapeHits,
    },
};
use bevy::prelude::*;
use leafwing_input_manager::{
    plugin::InputManagerPlugin,
    prelude::ActionState,
};

use super::{
    camera_rig::RiggedCamera,
    input::PlayerActions,
    kinematic_controller::KinematicCharacterController,
    CharacterControllerSet,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (velocity_damping_system, update_grounded, movement_input, gravity_system)
            .chain()
            .in_set(CharacterControllerSet::Input),
    );
    app.add_plugins(InputManagerPlugin::<PlayerActions>::default());
}

// Marker component for whether or not we're currently grounded.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Gravity(Vec3);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec3::new(0.0, -9.81 * 2.0, 0.0))
    }
}

// System that listens for leafwing input events and updates the player's movement
pub fn movement_input(
    mut query: Query<(
        &ActionState<PlayerActions>,
        &mut KinematicCharacterController,
        Has<Grounded>,
        &mut Transform,
    )>,
    mut camera_query: Query<
        &mut Transform,
        (With<RiggedCamera>, Without<ActionState<PlayerActions>>),
    >,
    time: Res<Time>,
) {
    let Ok((action_state, mut kcc, grounded, mut player_transform)) = query.get_single_mut() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    let movement = action_state.clamped_axis_pair(&PlayerActions::Movement).xy();
    let direction = player_transform
        .rotation
        .mul_vec3(Vec3::new(movement.x, 0.0, -movement.y))
        .normalize_or_zero()
        * 2.0;

    if movement != Vec2::ZERO {
        kcc.velocity.x = direction.x;
        kcc.velocity.z = direction.z;
    }

    if action_state.pressed(&PlayerActions::Jump) && grounded {
        kcc.velocity.y = 5.0;
    }

    let mouse_sensitivity = Vec2::new(0.12, 0.10);
    let mut camera_movement = action_state.axis_pair(&PlayerActions::Camera) * time.delta_seconds();
    camera_movement.y = -camera_movement.y * mouse_sensitivity.y;
    camera_movement.x *= mouse_sensitivity.x;
    let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    pitch = (pitch - camera_movement.y).clamp(-1.54, 1.54);
    yaw += camera_movement.x;

    camera_transform.rotation =
        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw);
}

pub fn velocity_damping_system(
    mut query: Query<&mut KinematicCharacterController>,
    _time: Res<Time>,
) {
    for mut kcc in query.iter_mut() {
        kcc.velocity.x *= 0.9;
        kcc.velocity.z *= 0.9;
    }
}

pub fn gravity_system(
    mut query: Query<(&mut KinematicCharacterController, &Gravity, Has<Grounded>)>,
    time: Res<Time>,
) {
    for (mut kcc, gravity, grounded) in query.iter_mut() {
        kcc.velocity += gravity.0 * time.delta_seconds();
        if grounded && gravity.0.dot(kcc.velocity) > -0.01 {
            kcc.velocity.y = 0.0;
        }
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits, &Rotation), (With<Gravity>, With<RigidBody>)>,
) {
    let max_angle = (45.0 as Scalar).to_radians();
    for (entity, hits, rotation) in &mut query {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = Some(max_angle) {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}
