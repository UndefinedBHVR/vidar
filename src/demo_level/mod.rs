use crate::{
    character_controller::{
        self,
        spawn_test_character,
    },
    GameState,
};
use bevy::prelude::*;
use blenvy::*;

#[allow(dead_code)]
pub fn plugin(app: &mut App) {
    app.add_plugins(character_controller::plugin)
        .add_systems(OnEnter(GameState::Playing), (spawn_level, spawn_test_character).chain());
}

fn spawn_level(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/VidarDev.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}
