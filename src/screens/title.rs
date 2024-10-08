//! The title screen that appears when the game starts.

use bevy::{
    prelude::*,
    render::view::RenderLayers,
};

use super::Screen;
use crate::{
    theme::prelude::*,
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), show_title_screen);
}

fn show_title_screen(mut commands: Commands) {
    commands.ui_root().insert((StateScoped(Screen::Title), RenderLayers::layer(1))).with_children(
        |children| {
            children.button("Play").observe(enter_playing);
            children.button("Credits").observe(enter_credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").observe(exit_app);
        },
    );
}

fn enter_playing(
    _trigger: Trigger<OnPress>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    next_screen.set(Screen::Playing);
    game_state.set(GameState::Playing);
}

fn enter_credits(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
