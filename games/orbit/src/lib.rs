#![allow(warnings)]
use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{color::palettes::tailwind, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, prelude::*};
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_tweening::TweeningPlugin;

pub mod states;
pub mod ui;
pub mod prefabs;
pub mod skybox;
pub mod actions;
pub mod assets;
pub mod music;
//pub mod solar_system;


pub fn init_bevy_app() -> App {
    let mut app = App::new();

    app.add_plugins((
        sly_common::SlyCommonPlugin {
            title: "Orbit",
            ..default()
        },
        bevy_inspector_egui::quick::StateInspectorPlugin::<states::AppState>::default(),
        PhysicsPlugins::default(),
        TweeningPlugin,

        states::StatePlugin,
        ui::UiPlugin,
        EnhancedInputPlugin,
        actions::AppActionPlugin,        
        //assets::AssetPlugin,
        prefabs::PrefabPlugin,
        music::MusicPlugin,
    ))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.0,
        ..default()
    });    

    app
}


