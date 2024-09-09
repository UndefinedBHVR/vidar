//! This module defines the core input structure of the character controller.
//! It is used to define the input actions and their associated default bindings.
//! The actual input response behavior is defined in the relevant module for the action.
//! Such as movement input is defined in the movement module, or weapon input in the weapon module.
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
pub fn plugin(app: &mut App) {
    let _ = app;
}

pub fn input_map() -> InputMap<PlayerActions> {
    let mut map = InputMap::new([(PlayerActions::PrimaryAttack, MouseButton::Left)]);
    map.insert_dual_axis(PlayerActions::Movement, KeyboardVirtualDPad::WASD);
    map.insert_dual_axis(PlayerActions::Camera, MouseMove::default());
    map.insert(PlayerActions::Jump, KeyCode::Space);
    map.insert(PlayerActions::Reload, KeyCode::KeyR);
    map
}

// Enum defining the player input actions.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerActions {
    // This is a dual axis movement, takes in WASD and Gamepad input
    Movement,
    Camera,
    Jump,
    PrimaryAttack,
    Reload,
}

impl Actionlike for PlayerActions {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerActions::Movement => InputControlKind::DualAxis,
            PlayerActions::Camera => InputControlKind::DualAxis,
            PlayerActions::Jump => InputControlKind::Button,
            PlayerActions::PrimaryAttack => InputControlKind::Button,
            PlayerActions::Reload => InputControlKind::Button,
        }
    }
}
