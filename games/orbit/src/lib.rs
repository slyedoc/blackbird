#![allow(warnings)]
use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{color::palettes::tailwind, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, prelude::*, render::camera::Viewport, window::WindowResized};
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_mod_mipmap_generator::{MipmapGeneratorDebugTextPlugin, MipmapGeneratorPlugin};
use bevy_tweening::TweeningPlugin;
use states::AppState;

pub mod states;
pub mod ui;
pub mod prefabs;
pub mod skybox;
pub mod actions;
pub mod assets;
pub mod music;
//pub mod solar_system;


pub fn init_bevy_app(init_state: AppState) -> App {
    let mut app = App::new();

    app

    .add_plugins((
        sly_common::SlyCommonPlugin {
            title: "Orbit",
            ..default()
        },
        //bevy_inspector_egui::quick::StateInspectorPlugin::<states::AppState>::default(),
        MipmapGeneratorPlugin,
        MipmapGeneratorDebugTextPlugin,
        PhysicsPlugins::default(),
        TweeningPlugin,

        states::StatePlugin,
        ui::UiPlugin,
        EnhancedInputPlugin,
        actions::AppActionPlugin,        
        assets::AssetPlugin,
        prefabs::PrefabPlugin,
        music::MusicPlugin,
    ))
    .add_systems(Update, set_camera_viewports)
    .insert_state(init_state)
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.0,
        ..default()
    });    

    app
}

#[derive(Component)]
struct CameraPosition {
    pos: UVec2,
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<(&CameraPosition, &mut Camera)>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let size = window.physical_size() / 2;

        for (camera_position, mut camera) in &mut query {
            camera.viewport = Some(Viewport {
                physical_position: camera_position.pos * size,
                physical_size: size,
                ..default()
            });
        }
    }
}

