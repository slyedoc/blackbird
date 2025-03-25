mod actions;
use std::path::PathBuf;

pub use actions::*;
mod assets;
pub use assets::*;
mod components;
use bevy_mod_outline::OutlinePlugin;
pub use components::*;
mod save;
pub use save::*;
mod events;
pub use events::*;
mod copy_paste;
pub use copy_paste::*;
mod select;
pub use select::*;


use avian3d::prelude::*;
use bevy::{
    app::AppExit, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, input::common_conditions::input_just_pressed, log::Level, math::vec3, prelude::*
};
use sly_common::prelude::*;
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
}

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
            bevy_inspector_egui::quick::StateInspectorPlugin::<AppState>::default(),
        ));

        if !app.is_plugin_added::<OutlinePlugin>() {
            app.add_plugins(OutlinePlugin);
        }

        app.init_state::<AppState>()
        .init_resource::<ActionState<Action>>()
        .insert_resource(Action::input_map())
        .init_resource::<SaveTimer>()
        .add_event::<SaveEvent>()
        .add_event::<DeleteEvent>()

        .add_systems(Startup, setup)        
        .add_systems(OnEnter(AppState::Playing), load)
        .add_systems(Update, (      
            on_add_ref_image,
            on_update_select,
            autosave,      
            save::save.run_if(input_just_pressed(KeyCode::KeyS)),
            paste.run_if(action_just_pressed(Action::Paste)),
            file_drop,
            image_delete.run_if(on_event::<DeleteEvent>),
        ))
        .add_systems(PostUpdate, save_on_exit.run_if(on_event::<AppExit>))
        .add_systems(Last, save.run_if(on_event::<SaveEvent>))

        .register_type::<RefImage>()
        .register_type::<RefConfig>()
        .register_type::<PositionedImage>()
        .register_type::<SaveTimer>()
        .register_type::<DeleteEvent>()
        .register_type::<SaveEvent>()

        .run();
}

fn setup(
    mut commands: Commands,

    mut clear_color: ResMut<ClearColor>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    clear_color.0 = Color::srgb(0.1, 0.1, 0.1);

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
            eye: Vec3::new(0.0, 0.0, 10.0),
            target: Vec3::ZERO,
            up: Vec3::Y
            
        },
        FpsCameraController::default(),
    ));

    app_state.set(AppState::Playing);
}



fn load(
    mut commands: Commands,
    config: Res<RefConfig>,
    mut camera: Single<&mut Transform, With<Camera>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    camera.translation = config.camera_position;
    
    for (i, r) in config.images.iter().enumerate() {
        let image_handle: Handle<Image> = asset_server.load(&r.path);
        // search our loaded config file for this image
        // get image size
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1.0)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(image_handle.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })), 
            //BillboardTexture(image_handle.clone()),
            //BillboardMesh(meshes.add(Rectangle::from_size(Vec2::splat(2.0)))),
            Transform::from_translation( vec3(r.position.x, r.position.y, i as f32 * 0.1))
                .rotate_local_x(std::f32::consts::FRAC_PI_2),
            Name::new(r.name.clone()),
            RefImage {
                path: r.path.clone(),
            },
        ));
    }
}

fn image_delete(
    mut commands: Commands,
    query: Query<&RefImage>,
    mut delete_events: EventReader<DeleteEvent>,
) {
    for DeleteEvent(e) in delete_events.read() {
        if let Ok(image) = query.get(*e) {
            commands.entity(*e).despawn_recursive();
            info!("Deleted image: {:?}", image.path);
            std::fs::remove_file(&image.path).unwrap();
        }
    }
}



fn config_file_path() -> PathBuf {
    let root = std::env::var("BEVY_ASSET_ROOT").unwrap_or("".to_string());
    std::path::Path::new(&root).join("assets/ref/config.ron")
}

// TODO: doesnt work on wayland
fn file_drop(
    mut evr_dnd: EventReader<FileDragAndDrop>,    
) {
    for ev in evr_dnd.read() {
        // TODO: on wayland this event never fires        
        dbg!("File drop event never fire!!!!!!");
        match ev {
            FileDragAndDrop::DroppedFile { window, path_buf } => {
                info!("Dropped file: {:?} at {:?}", path_buf, window);
                //let texture_handle = asset_server.load(path_buf.to_str().unwrap().to_string());

                // commands.spawn(
                //     SpriteBundle {
                //         texture: texture_handle,
                //         transform: Transform::from_xyz(world_cursor.0.x, world_cursor.0.y, 0.0),
                //         ..default()
                //     });
            }
            FileDragAndDrop::HoveredFile {
                window: _,
                path_buf: _,
            } => {
                // On wayland this sometimes prints multiple times for one drop
                info!("Hovered file");
            }
            FileDragAndDrop::HoveredFileCanceled { window: _ } => {
                info!("File canceled!");
            }
        }
    }
}
