use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Prefab;

/// only used for init load, not updated currently
#[derive(Resource, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct RefConfig {
    pub camera_eye: Vec3,
    pub camera_target: Vec3,
    pub camera_up: Vec3,
    pub prefabs: Vec<PrefabConfig>,
}

impl Default for RefConfig {
    fn default() -> Self {
        Self {
            camera_eye: Vec3::new(0.0, 3.0, 10.0),
            camera_target: Vec3::new(0.0, 0.0, 0.0),
            camera_up: Vec3::Y,
            prefabs: Vec::new(),
        }
    }
}
#[derive(Serialize, Deserialize, Reflect)]
pub struct PrefabConfig {
    pub position: Vec3,
    pub scale: f32,
    pub prefab: Prefab,
}
