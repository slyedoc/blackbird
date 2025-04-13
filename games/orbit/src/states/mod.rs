mod intro;
mod menu;
mod splash;
mod hanger;

use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_plugins((
            splash::SplashPlugin,
            menu::MenuPlugin,
            intro::IntroPlugin,
            hanger::HangerPlugin,
        ));
    }
}

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum AppState {
    #[default]
    Splash,    
    Menu,    
    Intro,
    Hanger,
}

