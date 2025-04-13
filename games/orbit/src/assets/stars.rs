use bevy::{asset::{io::Reader, AssetLoader, LoadContext}, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use thiserror::Error;

#[derive(Default)]
pub struct StarAssetLoader;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum StarAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    /// A [Json](ron) Error
    #[error("Could not parse Json: {0}")]
    JsonError(#[from] serde_json::error::Error),
}

impl AssetLoader for StarAssetLoader {
    type Asset = Stars;
    type Error = StarAssetLoaderError;
    type Settings = ();

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = from_slice(&bytes)?;        
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &[".json"]
    }
}


/// custom asset loader for height files, this avoids the need to know the
/// bounds of any noise function we use
#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct Stars(pub Vec<Star>);

impl From<&DbStars> for Stars {
    fn from(db_stars: &DbStars) -> Self {
        let mut stars = Vec::new();
        for db_star in &db_stars.0 {
            let star = Star {
                name: db_star.name.clone(),
                position: Vec3::new(
                    db_star.x.unwrap_or(0.0),
                    db_star.y.unwrap_or(0.0),
                    db_star.z.unwrap_or(0.0),
                ),
                distance: db_star.distance.unwrap_or(0.0),
                color: db_star.color.as_ref().map_or(Color::WHITE, |c| {
                    Color::LinearRgba(LinearRgba::rgb( c.r, c.g, c.b))
                }),
                luminosity: db_star.luminosity.unwrap_or(0.0),
                index: db_star.i,
            };
            stars.push(star);
        }
        Self(stars)
    }
}

/// custom asset loader for height files, this avoids the need to know the
/// bounds of any noise function we use
#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct Star {
    pub name: String,
    // in relative position galiactic center
    pub position: Vec3,
    /// in light years from earth
    pub distance: f32,
    pub color: Color,
    pub luminosity: f32,
    /// db index
    pub index: u32,
}

/// custom asset loader for BSC5P format
#[derive(Debug, Serialize, Deserialize)]
pub struct DbStars(pub Vec<DbStar>);

#[derive(Debug, Serialize, Deserialize)]
pub struct DbColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbStar {
    // {
    //     "i": 15,
    //     "n": "HD358",
    //     "x": 14.46131606636936,
    //     "y": -0.9510478507315397,
    //     "z": 25.974665968524928,

    //     "p": 29.744199881023203,

    //     "N": 104.28367062634601,
    //     "K": {
    //         "r": 0.355,
    //         "g": 0.499,
    //         "b": 1
    //     }
    // },
    // "x": null,
    // "y": null,
    // "z": null,
    // "p": null,
    // "N": null,
    // 1 parsec ≈ 3.26 light-years.
    pub i: u32,
    #[serde(rename = "n")]
    pub name: String,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    /// Distance in parsecs, ignoring uncertainty.
    #[serde(rename = "p")]
    pub distance: Option<f32>,
    /// Naively calculated luminosity.
    #[serde(rename = "N")]
    pub luminosity: Option<f32>,
    #[serde(rename = "K")]
    pub color: Option<DbColor>,
}
