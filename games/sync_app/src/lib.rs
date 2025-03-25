mod systems;

use systems::*;

pub const RENDER_HEIGHT: f32 = 600.;
pub const RENDER_WIDTH: f32 = 800.;
use bevy::{
    color::palettes::tailwind::*, core_pipeline::Skybox, input::common_conditions::input_pressed,
    prelude::*,
};
use bevy_rand::prelude::*;

pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        sly_common::SlyCommonPlugin {
            title: "Sync App".into(),
            ..default()
        },
        MeshPickingPlugin,
        EntropyPlugin::<WyRand>::default(),        
    ))
    .add_systems(Startup, (setup_scene,))
    .add_systems(Update, (change_name.run_if(input_pressed(KeyCode::Enter)),));

    app
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let ground_matl = materials.add(Color::from(GRAY_700));

    // Spawn the shapes. The meshes will be pickable by default.
    // for (i, shape) in shapes.into_iter().enumerate() {
    //     commands
    //         .spawn((
    //             Mesh3d(shape),
    //             MeshMaterial3d(white_matl.clone()),
    //             Transform::from_xyz(
    //                 -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
    //                 2.0,
    //                 Z_EXTENT / 2.,
    //             )
    //             .with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //             Shape,
    //         ))
    //         .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
    //         .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
    //         .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
    //         .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()))
    //         .observe(rotate_on_drag);
    // }

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(ground_matl.clone()),
        PickingBehavior::IGNORE, // Disable picking for the ground plane.
        Name::new("Ground"),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 900.0,
            ..default()
        },
        Skybox {
            image: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            brightness: 500.0,
            rotation: Quat::IDENTITY,
        },
    ));
}
