#![allow(warnings)]
#![allow(unused_imports)]
use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::*, log::LogPlugin, picking::pointer::PointerInteraction, prelude::*,
    window::WindowResolution,
};

pub const RENDER_HEIGHT: f32 = 600.;
pub const RENDER_WIDTH: f32 = 800.;

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

pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        sly_common::SlyCommonPlugin {
            title: GAME_NAME,
            ..default()
        },
        MeshPickingPlugin,
        StatePlugin,
        UiPlugin,
        BoardPlugin,
    ));
    app
}

pub fn exit(mut commands: Commands) {
    commands.send_event(AppExit::Success);
}
