mod menu;
mod intro;

use bevy::prelude::*;
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_plugins((
            menu::MenuPlugin,
            intro::IntroPlugin,
        ));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Intro,
    Hanger,
}