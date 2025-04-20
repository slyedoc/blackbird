use crate::{
    prefabs::{ships::Ship, solar_system::SolarSystem}, states::AppState, ui::*
};

use bevy::{core_pipeline::{bloom::Bloom, tonemapping::Tonemapping, Skybox}, prelude::*};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Intro), (setup, setup_ui));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("MainCamera"),
        StateScoped(AppState::Intro),
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        // Skybox {
        //     image: asset_server.load("textures/skybox/space.png"),
        //     brightness: 1000.0,
        //     ..default()
        // },
        Bloom {
            intensity: 0.3, // the default is 0.3,
            ..default()
        },
        // Skybox {
        //     brightness: 5000.0,
        //     image: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //     ..default()
        // },
        // EnvironmentMapLight {
        //     diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
        //     specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //     intensity: 2500.0,
        //     ..default()
        // },
        // movement
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Name::new("Player"),
        StateScoped(AppState::Intro),
        SolarSystem::default(),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
    
}


fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, ui: Res<UiAssets>) {
    let font = ui.font.clone();


    commands.spawn((
        Name::new("Skip Panel"),
        StateScoped(AppState::Intro),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(5.),               
            right: Val::Percent(5.),            
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },        
    )).with_children(|parent| {
        parent.spawn((
            QuickButton,
            Name::new("Skip Button"),
            children![
                (
                    QuickButtonInner,
                    ImageNode::new(asset_server.load("textures/icon/white/arrowRight.png")),
                )
            ]
        )).observe(|_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.send_event(FadeTo(AppState::Menu));            
        });
    });
}