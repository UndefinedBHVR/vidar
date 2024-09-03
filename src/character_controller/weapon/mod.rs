//! Weapon module. Weapons are each defined as their own singleton entity, with a series of
//! components that define their properties. This makes having data driven weapons easier to
//! implement, as you can define a weapon in a data file and then load it in at runtime or even
//! modify it at runtime.
use bevy::prelude::*;
mod components;
mod input;
mod prefabs;
pub(super) fn plugin(app: &mut App) {
    // Temporarily appease clippy.
    let _ = app;
}
