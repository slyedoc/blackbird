#![feature(trivial_bounds)]
mod actions;
use std::path::PathBuf;

pub use actions::*;
mod assets;
pub use assets::*;

mod comfy;
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

use avian3d::prelude::*;
use bevy::{
    app::AppExit, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, log::Level, math::vec3, prelude::*
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_mod_outline::OutlinePlugin;

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
    app.insert_resource(config).add_plugins((
        sly_common::SlyCommonPlugin {
            title: "sly_ref",
            level: Level::INFO,
        },
        MeshPickingPlugin,
        PhysicsPlugins::default(), // using for collision detection
        InputManagerPlugin::<Action>::default(),
        //FilterQueryInspectorPlugin::<With<Selected>>::default(),
        InfiniteGridPlugin,
        TokioTasksPlugin::default()
    ))
    .add_systems(Update, ui_select.run_if(|query: Query<Entity, With<Selected>>| {
        !query.is_empty()
    }));

    if !app.is_plugin_added::<OutlinePlugin>() {
        app.add_plugins(OutlinePlugin);
    }

    app
        .init_resource::<ActionState<Action>>()
        .insert_resource(Action::input_map())
        .init_resource::<SaveTimer>()
        .add_event::<Save>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
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
) {
    commands.spawn(InfiniteGridBundle::default());

    commands.spawn((
        Name::new("MainCamera"),
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::None,
        Bloom {
            intensity: 0.2,
            ..default()
        },
        // movement
        LookTransform {
            eye: config.camera_eye,
            target: config.camera_target,
            up: Vec3::Y,
        },
        UnrealCameraController::default(),
    ));

    // background, used to deselect
    commands.spawn((
        Name::new("Backdrop"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(100.0)))),
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
    )).observe(|_: Trigger<Pointer<Click>>, selected: Query<Entity, With<Selected>>, mut commands: Commands| {
        for e in selected.iter() {
            commands.entity(e).remove::<Selected>();
        }
    });

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
