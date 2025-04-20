use bevy::prelude::*;
use bevy_mod_mipmap_generator::generate_mipmaps;
use stars::*;
pub mod stars;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<stars::Stars>()
        .init_resource::<BuildingAssets>()
        .init_asset_loader::<stars::StarAssetLoader>()
        .init_resource::<stars::StarAssets>()        ;
        // TODO: use with save
        //.add_systems(Update, generate_mipmaps::<StandardMaterial>);
    }
}

#[derive(Resource)]
pub struct BuildingAssets {
    pub building_lg_a: Handle<Scene>,    
    //pub art_nouveau: Handle<Scene>,    
}

impl FromWorld for BuildingAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        //let lunarbase = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/kb3d_lunarbase.glb"));
        let building_lg_a = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/KB3D_LNB_BldgLG_A_grp/KB3D_LNB_BldgLG_A_grp.gltf"));
        //let art_nouveau = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/kb3d_art_nouveau.glb"));
        BuildingAssets { building_lg_a }
    }
}

