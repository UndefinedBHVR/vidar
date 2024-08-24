mod assets;
mod audio;
mod character_controller;
mod demo_level;
#[cfg(feature = "dev")]
mod dev_tools;
mod screens;
mod theme;
use avian3d::PhysicsPlugins;
use bevy::{
    asset::AssetMetaCheck,
    audio::{
        AudioPlugin,
        Volume,
    },
    log::LogPlugin,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_gizmo_log::GizmoLogPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use blenvy::BlenvyPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Vidar".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume { volume: Volume::new(0.3) },
                    ..default()
                })
                .disable::<LogPlugin>(),
        );

        app.add_plugins(WorldInspectorPlugin::new());
        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins(GizmoLogPlugin::default());
        //app.add_plugins(PhysicsDebugPlugin::default());
        // Add other plugins.
        app.add_plugins((
            demo_level::plugin,
            //demo::plugin,
            screens::plugin,
            theme::plugin,
            assets::plugin,
            audio::plugin,
            BlenvyPlugin::default(),
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
        app.init_state::<GameState>();
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Playing,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            camera: Camera { order: 1, clear_color: ClearColorConfig::None, ..Default::default() },
            ..Default::default()
        },
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
        RenderLayers::layer(1),
    ));
}
