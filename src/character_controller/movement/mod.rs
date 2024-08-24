use avian3d::{
    math::{
        Scalar,
        Vector,
    },
    prelude::{
        LinearVelocity,
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
    CharacterControllerSet,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (update_grounded, movement_input, gravity_system)
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
        &mut LinearVelocity,
        Has<Grounded>,
        &mut Transform,
    )>,
    mut camera_query: Query<
        &mut Transform,
        (With<RiggedCamera>, Without<ActionState<PlayerActions>>),
    >,
    time: Res<Time>,
) {
    let Ok((action_state, mut lv, grounded, mut player_transform)) = query.get_single_mut() else {
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
        lv.0.x = direction.x;
        lv.0.z = direction.z;
    }

    if action_state.pressed(&PlayerActions::Jump) && grounded {
        lv.0.y = 5.0;
    }

    let mouse_sensitivity = Vec2::new(0.12, 0.10);
    let mut camera_movement = action_state.axis_pair(&PlayerActions::Camera) * time.delta_seconds();
    camera_movement.y = -camera_movement.y * mouse_sensitivity.y;
    camera_movement.x = camera_movement.x * mouse_sensitivity.x;
    let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    pitch = (pitch - camera_movement.y).clamp(-1.54, 1.54);
    yaw += camera_movement.x;

    camera_transform.rotation =
        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw);
}

pub fn gravity_system(
    mut query: Query<(&mut LinearVelocity, &Gravity, Has<Grounded>)>,
    time: Res<Time>,
) {
    for (mut lv, gravity, grounded) in query.iter_mut() {
        lv.0 += gravity.0 * time.delta_seconds();
        if grounded && gravity.0.dot(lv.0) > 0.0 {
            lv.0.y = 0.0;
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
