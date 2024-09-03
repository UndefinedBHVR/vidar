use bevy::prelude::*;

use super::CharacterControllerSet;

pub fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, (track_entity).in_set(CharacterControllerSet::CameraSync))
        .add_systems(Startup, create_camera);
}

// Specifies that this is the primary camera and should be used for the main view
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RiggedCamera;

// Specifies the entity that we are attached to, as well as the offset from that entity
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct TrackedEntity(pub Vec3);

pub fn track_entity(
    mut query: Query<(&TrackedEntity, &mut Transform), Without<RiggedCamera>>,
    mut camera_query: Query<&mut Transform, With<RiggedCamera>>,
) {
    // There should only ever be one tracked entity and one rigged camera.
    if let Ok((tracked_entity, tracked_transform)) = query.get_single_mut() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = tracked_entity.0 + tracked_transform.translation;
        }
    }
}

pub fn create_camera(mut commands: Commands) {
    commands.spawn((
        RiggedCamera,
        Camera3dBundle {
            // Adjust our rotation so we're looking backwards on spawn
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0)),
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.384, 0.71, 0.949)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
