use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Asset, TypePath)]
pub struct RefConfig {
    pub camera_position: Vec3,
    pub camera_scale: f32,
    pub images: Vec<PositionedImage>,
}

// used to load assets for now
#[derive(AssetCollection, Resource, Debug)]
pub struct RefAssets {
    /// Height maps
    #[asset(path = "ref", collection(typed, mapped))]
    pub images: HashMap<String, Handle<Image>>,

    #[asset(path = ".ref.json")]
    pub config: Handle<RefConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct PositionedImage {
    pub image: String,
    pub position: Vec3,
    pub scale: f32,
}

