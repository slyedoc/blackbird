use bevy::prelude::*;
use stars::*;
pub mod stars;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<stars::Stars>()
        .init_asset_loader::<stars::StarAssetLoader>()
        .init_resource::<StarAssets>()
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load);
    }
}



#[derive(Resource, Default)]
struct StarAssets {
    handle: Handle<Stars>,
    printed: bool,
}

fn setup(mut state: ResMut<StarAssets>, asset_server: Res<AssetServer>) {
    // Recommended way to load an asset
    state.handle = asset_server.load("stars/bsc5p_3d.json");    
}


fn print_on_load(
    mut state: ResMut<StarAssets>,
    custom_assets: Res<Assets<Stars>>,    
) {
    let custom_asset = custom_assets.get(&state.handle);    

    // Can't print results if the assets aren't ready
    if state.printed {
        return;
    }

    if custom_asset.is_none() {
        info!("Custom Asset Not Ready");
        return;
    }

    info!("Custom asset loaded: {:?}", custom_asset.unwrap());        

    // Once printed, we won't print again
    state.printed = true;
}
