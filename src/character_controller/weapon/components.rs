//! This module defines various components that can be used to represent
//! different aspects of weapons in a game. These components can be attached
//! to entities to give them weapon-like properties and behaviors.
//!
//! # Components
//!
//! - `Weapon`: Represents basic weapon properties like damage, range, and fire rate.
//! - `Falloff`: Defines damage falloff characteristics for weapons.
//! - `Viewmodel`: Specifies the visual representation of a weapon in first-person view.
//! - `RangedWeapon`: A marker component for ranged weapons.
//! - `ProjectileWeapon`: A marker component for weapons that fire projectiles.
//! - `WeaponID`: Provides a unique identifier for weapons.
//! - `WeaponModel`: Defines the 3D model and material for a weapon.
//! - `HasAmmo`: Represents ammunition-related properties for weapons.
//! - `FiresMultiple`: Indicates that a weapon fires multiple projectiles per shot.
//!
//! These components are designed to work with the Bevy game engine and support
//! reflection for runtime type information and debugging.

use bevy::prelude::*;

/// Represents basic properties of a weapon.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Weapon {
    /// The amount of damage dealt by the weapon.
    pub damage: f32,
    /// The maximum effective range of the weapon.
    pub range: f32,
    /// The time interval between shots (in seconds).
    pub fire_interval: f32,
    /// The time until the next shot can be fired (in seconds).
    pub next_fire: f32,
}

/// Defines damage falloff characteristics for a weapon.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Falloff {
    /// The distance at which damage falloff begins.
    pub start: f32,
    /// The distance over which the falloff occurs.
    pub duration: f32,
}

/// Specifies the visual representation of a weapon in first-person view.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Viewmodel {
    /// The 3D mesh used for the viewmodel.
    pub model: Handle<Mesh>,
    /// The material applied to the viewmodel mesh.
    pub material: Handle<StandardMaterial>,
}

/// A marker component for ranged weapons.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RangedWeapon;

/// A marker component for weapons that fire projectiles.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ProjectileWeapon;

/// Provides a unique identifier for weapons.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct WeaponID(pub String);

/// Defines the 3D model and material for a weapon.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct WeaponModel {
    /// The 3D mesh used for the weapon model.
    pub model: Handle<Mesh>,
    /// The material applied to the weapon model mesh.
    pub material: Handle<StandardMaterial>,
}

/// Represents ammunition-related properties for weapons.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct HasAmmo {
    /// The number of ammunition units consumed per shot.
    pub per_shot: i32,
    /// The current amount of ammunition in the clip.
    pub in_clip: i32,
    /// The maximum amount of ammunition that can be held in a clip.
    pub max_clip: i32,
    /// The maximum total amount of ammunition that can be carried.
    pub max: i32,
    /// The time required to reload the weapon (in seconds).
    pub reload_time: f32,
}

/// Indicates that a weapon fires multiple projectiles per shot.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct FiresMultiple {
    /// The number of projectiles fired per shot.
    pub count: i32,
}
