#[cfg(feature = "stars")]
mod stars;

use bevy::prelude::*;

pub struct SlyAssetsPlugin;

impl Plugin for SlyAssetsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "stars")]
        app.add_plugins(stars::StarAssetPlugin);
    }
}