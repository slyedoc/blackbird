#![allow(warnings)]
#![allow(unused_imports)]
use std::f32::consts::PI;

use bevy::{color::palettes::tailwind::*, log::LogPlugin, picking::pointer::PointerInteraction, prelude::*, window::WindowResolution};
use sly_common::prelude::*;

mod board;
mod state;
mod ui;

pub mod prelude {
    pub use bevy::color::palettes::tailwind::*;
    pub use leafwing_input_manager::prelude::*;

    use super::*;
    #[allow(unused_imports)]
    pub use {board::*, state::*, ui::*};
}

use crate::prelude::*;

const GAME_NAME: &str = "Tic Tac Toe";

pub fn new(
) -> App {
    let mut app = App::new();
    app.add_plugins((
        SlyDefaultPlugins {
            title: GAME_NAME.to_string(),
            position: (0, 0),
            size: (800, 600),
            ..default()
        },
        MeshPickingPlugin,
        StatePlugin,
        UiPlugin,
        BoardPlugin,
    ));
    //.insert_resource(WinitSettings::desktop_app())
    app         
}

pub fn exit(mut commands: Commands) {
    commands.send_event(AppExit::Success);
}