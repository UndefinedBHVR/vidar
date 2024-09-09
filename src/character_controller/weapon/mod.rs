use bevy::prelude::*;
mod components;
mod event;
mod input;
mod prefabs;
pub(super) fn plugin(app: &mut App) {
    // Temporarily appease clippy.
    app.add_plugins(input::plugin);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct WeaponContainer {
    // The currently active weapon slot. If None, no weapon is equipped.
    active_slot: Option<Entity>,
    // Just a list of weapon slots.
    slots: Vec<Entity>,
}
