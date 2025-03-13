#![allow(warnings)]
#![allow(unused_imports)]
use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::*, log::LogPlugin, picking::pointer::PointerInteraction, prelude::*,
    window::WindowResolution,
};
use sly_common::prelude::*;

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

pub struct TicTacToePlugin {
    #[cfg(target_arch = "wasm32")]
    pub canvas_id: String,
}

impl Default for TicTacToePlugin {
    fn default() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            canvas_id: "#bevy".to_string(),
        }
    }
}
pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                #[cfg(target_arch = "wasm32")]
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    fit_canvas_to_parent: true,
                    title: GAME_NAME.to_string(),
                    
                    canvas: Some("#bevy_canvas".to_string()),
                    ..default()
                }),
                ..default()
            }),
            
            //.disable::<LogPlugin>(),
        #[cfg(feature = "editor")]
        sly_editor::SlyEditorPlugin,
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
