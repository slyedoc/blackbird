use std::f32::consts::PI;

use crate::{
    assets::BuildingAssets, prefabs::{ships::Ship, solar_system::SolarSystem}, states::AppState, ui::*
};

use bevy::{
    color::palettes::tailwind,
    core_pipeline::{Skybox, bloom::Bloom, tonemapping::Tonemapping},
    pbr::{Atmosphere, AtmosphereSettings, CascadeShadowConfigBuilder, light_consts::lux},
    prelude::*,
    render::camera::Exposure,
    picking::backend::ray::RayMap,
};

pub struct HangerPlugin;

impl Plugin for HangerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Hanger), (setup, setup_ui));
        //.add_systems(Update, dynamic_scene);
    }
}


// const MAX_BOUNCES: usize = 64;
// const LASER_SPEED: f32 = 0.03;


// // Bounces a ray off of surfaces `MAX_BOUNCES` times.
// fn bounce_ray(mut ray: Ray3d, ray_cast: &mut MeshRayCast, gizmos: &mut Gizmos, color: Color) {
//     let mut intersections = Vec::with_capacity(MAX_BOUNCES + 1);
//     intersections.push((ray.origin, Color::srgb(30.0, 0.0, 0.0)));

//     for i in 0..MAX_BOUNCES {
//         // Cast the ray and get the first hit
//         let Some((_, hit)) = ray_cast
//             .cast_ray(ray, &MeshRayCastSettings::default())
//             .first()
//         else {
//             break;
//         };

//         // Draw the point of intersection and add it to the list
//         let brightness = 1.0 + 10.0 * (1.0 - i as f32 / MAX_BOUNCES as f32);
//         intersections.push((hit.point, Color::BLACK.mix(&color, brightness)));
//         gizmos.sphere(hit.point, 0.005, Color::BLACK.mix(&color, brightness * 2.0));

//         // Reflect the ray off of the surface
//         ray.direction = Dir3::new(ray.direction.reflect(hit.normal)).unwrap();
//         ray.origin = hit.point + ray.direction * 1e-6;
//     }
//     gizmos.linestrip_gradient(intersections);
// }


#[derive(Component)]
struct Terrain;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    building_assets: Res<BuildingAssets>,
) {
    commands.spawn((
        Name::new("MainCamera"),
        StateScoped(AppState::Hanger),
        Camera3d::default(),
        // HDR is required for atmospheric scattering to be properly applied to the scene
        Camera {
            hdr: true,
            ..default()
        },
        Msaa::Off,
        Transform::from_xyz(0., 20.0, 0.0),
        // This is the component that enables atmospheric scattering for a camera
        Atmosphere::EARTH,
        // The scene is in units of 10km, so we need to scale up the
        // aerial view lut distance and set the scene scale accordingly.
        // Most usages of this feature will not need to adjust this.
        AtmosphereSettings {
            //aerial_view_lut_max_distance: 3.2e5,
            //scene_units_to_m: 1e+4,
            ..Default::default()
        },
        // Skybox {
        //     image: asset_server.load("textures/skybox/space.ktx2"),
        //     brightness: 200000.0,
        //     ..default()
        // },
        // The directional light illuminance  used in this scene
        // (the one recommended for use with this feature) is
        // quite bright, so raising the exposure compensation helps
        // bring the scene to a nicer brightness range.
        Exposure::SUNLIGHT,
        // Tonemapper chosen just because it looked good with the scene, any
        // tonemapper would be fine :)
        Tonemapping::AcesFitted,
        // Bloom gives the sun a much more natural look.
        Bloom::NATURAL,
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            // lux::RAW_SUNLIGHT is recommended for use with this feature, since
            // other values approximate sunlight *post-scattering* in various
            // conditions. RAW_SUNLIGHT in comparison is the illuminance of the
            // sun unfiltered by the atmosphere, so it is the proper input for
            // sunlight to be filtered by the atmosphere.
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(1000.0, 500., 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder::default().build(),
    ));

    // // gltf files
    // commands.spawn((
    //     Name::new("Test Node"),
    //     StateScoped(AppState::Hanger),
    //     SceneRoot(
    //         asset_server.load(GltfAssetLabel::Mesh(10).from_asset("models/kb3d_lunarbase.glb"))
    //     ),
    //     Transform::from_xyz(0.0, 5.0, 0.0)
    // ));

    commands.spawn((
        Terrain,
        StateScoped(AppState::Hanger),
        SceneRoot(
            building_assets.building_lg_a.clone(),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)            
    ));

    

    // commands.spawn((
    //     Terrain,
    //     StateScoped(AppState::Hanger),
    //     SceneRoot(
    //         building_assets.art_nouveau.clone(),
    //     ),
    //     Transform::from_xyz(-1000.0, 0.0, 0.0)            
    // ));



    // ground
    commands.spawn((
        Name::new("Ground"),
        StateScoped(AppState::Hanger),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100000.)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            //base_color_texture: Some(asset_server.load("textures/ground/ground.png")),
            base_color: tailwind::GRAY_100.into(),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, ui: Res<UiAssets>) {
    let font = ui.font.clone();

    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    QuickButton,
                    Name::new("Skip Button"),
                    children![(
                        QuickButtonInner,
                        ImageNode::new(asset_server.load("textures/icon/white/arrowRight.png")),
                    )],
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.send_event(FadeTo(AppState::Hanger));
                    },
                );
        });
}

fn dynamic_scene(mut suns: Query<&mut Transform, With<DirectionalLight>>, time: Res<Time>) {
    suns.iter_mut()
        .for_each(|mut tf| tf.rotate_x(-time.delta_secs() * PI / 10.0));
}
