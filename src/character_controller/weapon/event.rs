use bevy::prelude::*;
#[derive(Event, Debug)]
pub struct WeaponFiredEvent {
    pub player_id: Entity,
    pub weapon_id: Entity,
}
