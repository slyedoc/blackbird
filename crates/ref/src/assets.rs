use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// only used for init load, not updated currently
#[derive(Default, Resource, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct RefConfig {
    pub camera_position: Vec3,
    pub images: Vec<PositionedImage>,
}

#[derive(Serialize, Deserialize, Reflect)]
pub struct PositionedImage {
    pub name: String,
    pub path: String,
    pub position: Vec3,
    pub scale: f32,
}

