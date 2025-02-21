// Completely based on smooth_bevy_cameras
use bevy::prelude::*;

mod controllers;
pub use controllers::*;

mod look_angles;
mod look_transform;

pub use look_angles::*;
pub use look_transform::*;

pub struct SlyCameraPlugin;

impl Plugin for SlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LookTransformPlugin,
            // controllers
            FpsCameraPlugin::default(),
            OrbitCameraPlugin::default(),
            UnrealCameraPlugin::default(),
        ));
    }
}

