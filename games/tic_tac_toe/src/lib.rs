#![allow(warnings)]
#![allow(unused_imports)]
use std::f32::consts::PI;

use bevy::{color::palettes::tailwind::*, log::LogPlugin, picking::pointer::PointerInteraction, prelude::*, window::WindowResolution};
use sly_window_state::{prelude::*, WindowState};
use sly_editor::prelude::*;

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

pub fn run() {
    let window_state = WindowState::build(
        GAME_NAME,
        (0, 0),
        (10, 800)
    );

    let mut app = App::new();
    let position = WindowPosition::At(IVec2::from(window_state.position));
    let resolution = WindowResolution::new(
        window_state.size.0 as f32,
        window_state.size.1 as f32,
    );

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_NAME.to_owned(),
                position,
                resolution,
                ..Default::default()
            }),
            ..default()
        }).set(LogPlugin {
            filter: "wgpu_hal=error,wgpu_core=error,bevy_render=error,bevy_persistent=error".into(),
            level: bevy::log::Level::INFO,
            ..default()
        }),        
        MeshPickingPlugin,        
        SlyEditorPlugin,
        WindowStatePlugin,
        StatePlugin,
        UiPlugin,
        BoardPlugin,
    ))
    //.insert_resource(WinitSettings::desktop_app())
    .insert_resource(window_state)    
    .run();
}

pub fn exit(mut commands: Commands) {
    commands.send_event(AppExit::Success);
}