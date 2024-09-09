use super::components::*;
use bevy::prelude::*;

#[derive(Bundle, Reflect, Debug)]
struct RangedPrototypeBundle {
    core: Weapon,
    falloff: Falloff,
    weapon_id: WeaponID,
    ranged_weapon: RangedWeapon,
    ammo: HasAmmo,
    model: WeaponModel,
    fires_multiple: FiresMultiple,
}

impl Default for RangedPrototypeBundle {
    fn default() -> Self {
        Self::assault_rifle()
    }
}

/// This is just some default types we can use for testing.
#[allow(unused)]
impl RangedPrototypeBundle {
    pub fn assault_rifle() -> Self {
        Self {
            core: Weapon { damage: 15.0, range: 150.0, fire_interval: 0.1, next_fire: 0.0 },
            falloff: Falloff { start: 75.0, duration: 25.0 },
            weapon_id: WeaponID("assault_rifle".to_string()),
            ranged_weapon: RangedWeapon,
            ammo: HasAmmo { per_shot: 1, in_clip: 30, max_clip: 30, max: 180, reload_time: 1.5 },
            model: WeaponModel { model: Handle::default(), material: Handle::default() },
            fires_multiple: FiresMultiple { count: 1 },
        }
    }

    pub fn shotgun() -> Self {
        Self {
            core: Weapon { damage: 8.0, range: 50.0, fire_interval: 0.8, next_fire: 0.0 },
            falloff: Falloff { start: 10.0, duration: 20.0 },
            weapon_id: WeaponID("shotgun".to_string()),
            ranged_weapon: RangedWeapon,
            ammo: HasAmmo { per_shot: 1, in_clip: 8, max_clip: 8, max: 48, reload_time: 1.5 },
            model: WeaponModel { model: Handle::default(), material: Handle::default() },
            fires_multiple: FiresMultiple { count: 8 },
        }
    }

    pub fn sniper_rifle() -> Self {
        Self {
            core: Weapon { damage: 100.0, range: 500.0, fire_interval: 1.5, next_fire: 0.0 },
            falloff: Falloff { start: 400.0, duration: 100.0 },
            weapon_id: WeaponID("sniper_rifle".to_string()),
            ranged_weapon: RangedWeapon,
            ammo: HasAmmo { per_shot: 1, in_clip: 5, max_clip: 5, max: 30, reload_time: 3.0 },
            model: WeaponModel { model: Handle::default(), material: Handle::default() },
            fires_multiple: FiresMultiple { count: 1 },
        }
    }
}

#[derive(Bundle)]
struct RocketLauncherBundle {
    weapon: Weapon,
    weapon_id: WeaponID,
    projectile_weapon: ProjectileWeapon,
    ammo: HasAmmo,
    model: WeaponModel,
    fires_multiple: FiresMultiple,
}

impl Default for RocketLauncherBundle {
    fn default() -> Self {
        Self {
            weapon: Weapon { damage: 150.0, range: 200.0, fire_interval: 2.0, next_fire: 0.0 },
            weapon_id: WeaponID("rocket_launcher".to_string()),
            projectile_weapon: ProjectileWeapon,
            ammo: HasAmmo { per_shot: 1, in_clip: 4, max_clip: 4, max: 16, reload_time: 2.0 },
            model: WeaponModel { model: Handle::default(), material: Handle::default() },
            fires_multiple: FiresMultiple { count: 1 },
        }
    }
}
