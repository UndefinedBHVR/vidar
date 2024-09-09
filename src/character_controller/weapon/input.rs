use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{character_controller::CurrentPlayer, GameState};

use super::{
    super::input::PlayerActions,
    components::{
        Weapon,
        WeaponID,
    },
    event::WeaponFiredEvent,
    WeaponContainer,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, weapon_input.run_if(in_state(GameState::Playing)));
}

pub fn weapon_input(
    mut event_writer: EventWriter<WeaponFiredEvent>,
    query: Query<&ActionState<PlayerActions>>,
    weapon_query: Query<Entity, (With<Weapon>, With<WeaponID>)>,
    player_query: Query<(Entity, &WeaponContainer), With<CurrentPlayer>>,
) {
    let Ok((player_id, player_weapons)) = player_query.get_single() else {
        return;
    };
    let Some(weapon_id) = player_weapons.active_slot else {
        return;
    };
    if !weapon_query.contains(weapon_id) {
        info!("Player {player_id} has invalid weapon reference {weapon_id}.");
        return;
    }

    for action_state in query.iter() {
        if action_state.just_pressed(&PlayerActions::PrimaryAttack) {
            event_writer.send(WeaponFiredEvent { player_id, weapon_id });
        }
    }
}
