#![allow(warnings)]
use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{color::palettes::tailwind, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, prelude::*};
use bevy_enhanced_input::EnhancedInputPlugin;

pub mod states;
pub mod ui;
pub mod scene;
pub mod skybox;
pub mod actions;


pub fn init_bevy_app() -> App {
    let mut app = App::new();

    app.add_plugins((
        sly_common::SlyCommonPlugin {
            title: "Orbit",
            ..default()
        },
        PhysicsPlugins::default(),
        states::StatePlugin,
        ui::UiPlugin,
        scene::sol::SolPlugin,
        EnhancedInputPlugin,
        actions::AppActionPlugin,        
        //skybox::SkyboxPlugin,
    ));    

    app
}


