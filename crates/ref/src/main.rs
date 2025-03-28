#![feature(trivial_bounds)]
mod actions;
use std::path::PathBuf;

pub use actions::*;
mod assets;
pub use assets::*;

mod comfy;
use bevy_health_bar3d::{plugin::HealthBarPlugin, prelude::*};
use bevy_tokio_tasks::TokioTasksPlugin;
pub use comfy::*;
mod save;
pub use save::*;
mod copy_paste;
pub use copy_paste::*;
mod select;
pub use select::*;
mod prefab;
pub use prefab::*;
mod ui;

pub use ui::*;
mod progress;
pub use progress::*;

use avian3d::prelude::*;
use bevy::{
    app::AppExit, color::palettes::tailwind, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, log::Level, math::vec3, prelude::*
};
use bevy_infinite_grid::InfiniteGridPlugin;
use bevy_mod_outline::OutlinePlugin;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;
//use rand::prelude::*;

//use bevy_eventwork::{ConnectionId, EventworkRuntime, Network, NetworkData, NetworkEvent};
//use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};
use sly_common::prelude::*;
//use url::Url;
//use uuid::Uuid;

fn main() {
    let file_path = config_file_path();
    let config = match std::fs::read(&file_path) {
        Ok(s) => ron::de::from_bytes::<RefConfig>(&s).unwrap_or_default(),
        Err(_) => {
            error!("Failed to load config file: {:?}", &file_path);
            RefConfig::default()
        }
    };

    let mut app = App::new();
    app.insert_resource(config)
        .add_plugins((
            sly_common::SlyCommonPlugin {
                title: "sly_ref",
                level: Level::INFO,
            },
            MeshPickingPlugin,
            PhysicsPlugins::default(), // using for collision detection
            InputManagerPlugin::<Action>::default(),
            //FilterQueryInspectorPlugin::<With<Selected>>::default(),
            InfiniteGridPlugin,
            TokioTasksPlugin::default(),
            HealthBarPlugin::<WorkflowProgress>::default(),
            EntropyPlugin::<WyRand>::default(),
        ))
        .insert_resource(
            ColorScheme::<WorkflowProgress>::new()
                .foreground_color(ForegroundColor::Static(tailwind::GRAY_200.into())),
        )
        .add_systems(
            Update,
            ui_select.run_if(|query: Query<Entity, With<Selected>>| !query.is_empty()),
        );

    if !app.is_plugin_added::<OutlinePlugin>() {
        app.add_plugins(OutlinePlugin);
    }

    app.init_resource::<ActionState<Action>>()
        .insert_resource(Action::input_map())
        // .insert_resource(AmbientLight {
        //     color: Color::WHITE,
        //     brightness: 0.0, // none
        // })
        .init_resource::<SaveTimer>()
        .add_event::<Save>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (
                update_progress,
                duplicate_selected.run_if(action_just_pressed(Action::Duplicate)),
                delete_selected.run_if(action_just_pressed(Action::Delete)),
                on_add_prefab,
                on_update_select,
                autosave,
                save::save.run_if(action_just_pressed(Action::Save)),
                paste.run_if(action_just_pressed(Action::Paste)),
                file_drop,
            ),
        )
        .add_systems(PostUpdate, save_on_exit.run_if(on_event::<AppExit>))
        .add_systems(Last, save.run_if(on_event::<Save>))
        .register_type::<Prefab>()
        .register_type::<RefConfig>()
        .register_type::<PrefabConfig>()
        .register_type::<SaveTimer>()
        .register_type::<Save>()
        .run();
}

fn setup(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<RefConfig>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //commands.spawn(InfiniteGridBundle::default());

    commands.spawn((
        Name::new("MainCamera"),
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.1,
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
        LookTransform {
            eye: config.camera_eye,
            target: config.camera_target,
            up: Vec3::Y,
        },
        UnrealCameraController::default(),
    ));

    commands.spawn((
        PointLight {   
            intensity: 20_000_000.,
            range: 500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 20.0, 4.0),
    ));

    // Spawn the light.
    // commands.spawn((
    //     DirectionalLight {
    //         illuminance: 15000.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
    //     CascadeShadowConfigBuilder {
    //         maximum_distance: 3.0,
    //         first_cascade_far_bound: 0.9,
    //         ..default()
    //     }
    //     .build(),
    // ));

    // ground
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.0)))),
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(tailwind::GRAY_800),
            metallic: 0.0,
            reflectance: 0.0,
            ..default()
        })),
    ));

    // background, used to deselect
    commands
        .spawn((
            Name::new("Backdrop"),
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(100.0)))),
            Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
        ))
        .observe(
            |_: Trigger<Pointer<Click>>,
             selected: Query<Entity, With<Selected>>,
             mut commands: Commands| {
                for e in selected.iter() {
                    commands.entity(e).remove::<Selected>();
                }
            },
        );

    // add prefabs
    for (i, p) in config.prefabs.iter().enumerate() {
        commands.spawn((
            Transform::from_translation(vec3(p.position.x, p.position.y, i as f32 * 0.1)), // offset z so no z fighting
            Name::new(p.prefab.name.clone()),
            p.prefab.clone(),
        ));
    }
}



fn duplicate_selected(
    selected: Query<Entity, (With<Selected>, With<Prefab>)>,
    mut commands: Commands,
) {
    for e in selected.iter() {
        commands.trigger_targets(Duplicate, e);
    }
}

fn delete_selected(
    selected: Query<Entity, (With<Selected>, With<Prefab>)>,
    mut commands: Commands,
) {
    for e in selected.iter() {
        commands.trigger_targets(Delete, e);
    }
}

fn config_file_path() -> PathBuf {
    let root = std::env::var("BEVY_ASSET_ROOT").unwrap_or("".to_string());
    std::path::Path::new(&root).join("assets/ref/config.ron")
}
