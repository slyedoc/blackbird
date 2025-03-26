
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::Path;

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_mod_outline::{OutlineMode, OutlineVolume};
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use strum::EnumIter;

use crate::{comfy, Action, Selected};

#[derive(Component, Debug, Default, Clone, Reflect, Serialize, Deserialize, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
#[require(OutlineVolume(|| OutlineVolume {
    visible: false,
    colour: Color::srgb(1.0, 1.0, 1.0),
    width: 5.0,
}))]
#[require(OutlineMode(|| OutlineMode::FloodFlat))]
pub struct Prefab {
    pub name: String,
    pub workflow: Workflow,
} 

#[derive(Component, EnumIter, PartialEq, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub enum Workflow {
    StaticImage {
        image: Option<String>,
    },
    TextToImage {
        seed: u32,
        seed_random: bool,
        prompt: String,
        image: Option<String>,
    },
    TextToModel {
        seed: u32,
        seed_random: bool,
        prompt: String,
        image: Option<String>,
        model: Option<String>,
    },
}

impl Default for Workflow {
    fn default() -> Self {
        Workflow::StaticImage { image: None }
    }
}


impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Workflow::StaticImage { .. } => write!(f, "Static Image"),
            Workflow::TextToImage { .. } => write!(f, "Text To Image"),
            Workflow::TextToModel { .. } => write!(f, "Text To Model"),
        }
    }
}

#[derive(Component)]
pub struct ComfyImageTask(pub Task<HashMap<String, Vec<Vec<u8>>>>);


// impl ReloadImageTask {
//     fn new(asset_processsor: &AssetProcessor) -> Self {
//         let thread_pool = AsyncComputeTaskPool::get();
//         let proc = asset_processsor.clone();
//         let task = thread_pool.spawn(async move {
//             // wait before we start checking so file watcher has time to detect the change
//             task::sleep(Duration::from_secs_f32(0.1)).await;
//             // check the status
//             let status = proc.get_state().await;
//             status
//         });
//         Self(task)
//     }

//     fn retry(&mut self, asset_processsor: &AssetProcessor) {
//         let thread_pool = AsyncComputeTaskPool::get();
//         let proc = asset_processsor.clone();
//         let task = thread_pool.spawn(async move {
//             let status = proc.get_state().await;
//             status
//         });
//         self.0 = task;
//     }
// }

pub fn on_add_prefab(
    mut commands: Commands,
    added_query: Query<(Entity, &Prefab), Added<Prefab>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, prefab) in added_query.iter() {
        commands
            .entity(entity)
            .observe(on_drag)
            .observe(on_select)
            .observe(on_duplicate)
            .observe(on_delete)
            .observe(on_rename)
            .observe(on_generate);

        let (image, model) = match &prefab.workflow {
            Workflow::TextToImage { image, .. } => (image.clone(), None),
            Workflow::TextToModel { image, model, .. } => (image.clone(), model.clone()),
            Workflow::StaticImage { image } => (image.clone(), None),
        };

        let image_handle: Handle<Image> = match image {
            Some(img) => asset_server.load(img),
            None => Handle::<Image>::default(),
        };

        commands.entity(entity).insert((
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1.0)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(image_handle.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
        ));
    }
}

fn on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.target) {
        transform.translation.x += drag.delta.x * 0.02;
        transform.translation.y -= drag.delta.y * 0.02;
    }
}

// this could add a event instead of updateing
fn on_select(
    target: Trigger<Pointer<Click>>,
    mut commands: Commands,
    actions: Res<ActionState<Action>>,
    mut query: Query<(Entity, Option<&Selected>), With<Prefab>>,
) {
    for (e, selected) in query.iter_mut() {
        if e == target.target {
            if selected.is_none() {
                commands.entity(e).insert(Selected);
            }
        } else {
            if !actions.pressed(&Action::SelectAll) {
                if selected.is_some() {
                    commands.entity(e).remove::<Selected>();
                }
            }
        }
    }
}

#[derive(Event)]
pub struct Duplicate;

fn on_duplicate(
    trigger: Trigger<Duplicate>,
    mut commands: Commands,
    query: Query<(&Prefab, &Transform)>,
) {
    let entity = trigger.entity();

    let (prefab, trans) = query.get(entity).unwrap();

    // find new name
    let names = query
        .iter()
        .map(|(p, _)| p.name.clone())
        .collect::<Vec<_>>();

    let new_name = create_unique_name(&prefab.name, names);

    let mut new_prefab = prefab.clone();
    new_prefab.name = new_name.clone();

    match &mut new_prefab.workflow {
        Workflow::TextToImage { .. } => {
            info!("TextToImage: {:?}", new_prefab);
        }
        Workflow::TextToModel { .. } => {
            info!("TextToModel: {:?}", new_prefab);
        }
        Workflow::StaticImage { image } => {
            if let Some(img) = image {
                *img = update_image(&img, &new_prefab.name, true);
            }
        }
    }

    commands.spawn((
        Transform::from_translation(trans.translation + Vec3::new(2.0, 0., 0.1)), // offset z so no z fighting
        Name::new(new_name.clone()),
        new_prefab,
    ));
}

/// can be used to copy or rename the image, delete meta file
fn update_image(img: &String, name: &String, copy: bool) -> String {
    let asset_path = Path::new("./assets/");

    let image_path = Path::new(&img);
    let file_ext = image_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("png");
    let new_image_path = Path::new("ref").join(format!("{}.{}", name, file_ext));

    let src = asset_path.join(&image_path);
    let dst = asset_path.join(&new_image_path);
    if copy {
        dbg!("Copying image from {:?} to {:?}", &src, &dst);
        std::fs::copy(&src, &dst).unwrap_or_default();
    } else {
        dbg!("Renaming image from {:?} to {:?}", &src, &dst);
        std::fs::rename(&src, &dst).unwrap_or_default();
    }
    // delete meta file
    // let meta_path = format!("{}.meta", &src.to_str().unwrap());
    // dbg!("Deleting meta file {:?}", &meta_path);
    // std::fs::remove_file(meta_path).unwrap_or_default();
    let path = new_image_path.to_str().unwrap().to_string();
    path
}

fn remove_numeric_suffix(name: String) -> String {
    if let Some(pos) = name.rfind('_') {
        // Check if the characters after the underscore are all digits.
        if name[pos + 1..].chars().all(|c| c.is_ascii_digit()) {
            return name[..pos].to_string();
        }
    }
    name.to_string()
}

#[derive(Event)]
pub struct Delete;

fn on_delete(trigger: Trigger<Delete>, mut commands: Commands, query: Query<&Prefab>) {
    let entity = trigger.entity();

    let prefab = query.get(entity).unwrap();

    match &prefab.workflow {
        Workflow::TextToImage { .. } => {
            dbg!("TODO: TextToImage {:?}", prefab);
        }
        Workflow::TextToModel { .. } => {
            dbg!("TODO: TextToModel: {:?}", prefab);
        }
        Workflow::StaticImage { image } => {
            if let Some(img) = image {
                let path = std::path::Path::new("assets").join(img);
                if path.exists() {
                    std::fs::remove_file(path).unwrap_or_default();
                }
            }
        }
    }

    commands.entity(entity).despawn_recursive();
}

#[derive(Event)]
pub struct Rename(pub String);

pub fn on_rename(
    trigger: Trigger<Rename>,
    mut query: Query<&mut Prefab>,
) {
    let entity = trigger.entity();
    let mut new_name = trigger.0.clone();

    let names = query
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<_>>();

    new_name = create_unique_name(&new_name, names);

    let mut prefab = query.get_mut(entity).unwrap();
    prefab.name = new_name.clone();

    match &mut prefab.workflow {
        Workflow::TextToImage { image, .. } => {
            if let Some(img) = image {
                let image_path = update_image(&img, &new_name, false);
                *img = image_path.clone();
            }
        }
        Workflow::TextToModel { image, .. } => {
            if let Some(img) = image {
                let image_path = update_image(&img, &new_name, false);
                *img = image_path.clone();
            }
        }
        Workflow::StaticImage { image } => {
            if let Some(img) = image {
                let image_path = update_image(&img, &new_name, false);
                *img = image_path.clone();
            }
        }
    }
    // rename image

    //prefab.name = new_name.clone();
}

fn create_unique_name(new_name: &String, names: Vec<String>) -> String {
    let mut new_name = new_name.clone();
    if names.contains(&new_name) {
        // Rename
        let short_name = remove_numeric_suffix(new_name.clone());
        let mut name = short_name.clone();
        let mut i = 1;
        while names.contains(&name) {
            i += 1;
            name = format!("{}_{}", short_name, i);
        }
        new_name = name.clone();
    }
    new_name
}

// #[derive(Component)]
// pub struct ReloadImageTask(pub Task<ProcessorState>);

// impl ReloadImageTask {
//     fn new(asset_processsor: &AssetProcessor) -> Self {
//         let thread_pool = AsyncComputeTaskPool::get();
//         let proc = asset_processsor.clone();
//         let task = thread_pool.spawn(async move {
//             // wait before we start checking so file watcher has time to detect the change
//             task::sleep(Duration::from_secs_f32(0.1)).await;
//             // check the status
//             let status = proc.get_state().await;
//             status
//         });
//         Self(task)
//     }

//     fn retry(&mut self, asset_processsor: &AssetProcessor) {
//         let thread_pool = AsyncComputeTaskPool::get();
//         let proc = asset_processsor.clone();
//         let task = thread_pool.spawn(async move {
//             let status = proc.get_state().await;
//             status
//         });
//         self.0 = task;
//     }
// }

// pub fn check_reload_image(
//     mut commands: Commands,
//     asset_processsor: Res<AssetProcessor>,
//     mut tasks: Query<(Entity, &Prefab, &mut MeshMaterial3d<StandardMaterial>, &mut ReloadImageTask)>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     // check for existing tasks
//     for (e, prefab, mat, mut task) in tasks.iter_mut() {
//         dbg!("Checking task {:?}", e);
//         // check the status, if complete, remove the task
//         if let Some(status) = block_on(future::poll_once(&mut task.0)) {
//             dbg!( status == ProcessorState::Finished );
//             if status == ProcessorState::Finished {
//                 commands.entity(e).remove::<ReloadImageTask>();
//                 // reload the image
//                 dbg!("Reloading image if any");
//                 if let Some(image) = &prefab.image {
//                     dbg!("Reloading image {:?}", image);
//                     let asset_path = Path::new("./assets/");
//                     if !std::fs::exists( asset_path.join(image) ).unwrap() {
//                         error!("image not found");
//                     } else {
//                         info!("image found");
//                     }
//                     let image_handle: Handle<Image> = asset_server.load(image);
//                     let mat = materials.get_mut(&mat.0).unwrap();
//                     mat.base_color_texture = Some(image_handle.clone());
//                 }
//             }
//             else {
//                 dbg!("retry task");
//                 task.retry(&asset_processsor);
//             }
//         } else {
//             // task is still running
//             dbg!("Task still running");
//         }
//     }
// }

#[derive(Event)]
pub struct Generate;

pub fn on_generate(trigger: Trigger<Generate>, mut query: Query<&mut Prefab>, runtime: ResMut<TokioTasksRuntime>) {

    let mut prefab = query.get_mut(trigger.entity()).unwrap();
    let e = trigger.entity();
    let name = prefab.name.clone();
    match &mut prefab.workflow {
        Workflow::StaticImage { .. } => {}
        Workflow::TextToImage { image, seed, .. } => {
            // Create a reqwest client.



            // // Set the seed for our KSampler node.
            // if let Some(seed_value) = prompt.pointer_mut("/3/inputs/seed") {
            //     *seed_value = Value::Number(serde_json::Number::from(5));
            // }
            let image_path = match image {
                Some(img) => img.clone(),
                None => {
                    Path::new("ref").join(format!("{}.png", name)).to_str().unwrap().to_string()                    
                }
            };
            let seed = seed.clone();
            runtime.spawn_background_task(async move |mut ctx: TaskContext| {                       
                // JSON prompt text.
                let prompt_text = include_str!("workflows/ref_image_gen.json");

                // Parse the prompt JSON.
                let mut prompt: Value = serde_json::from_str(prompt_text).unwrap();
                            // Set the text prompt for our positive CLIPTextEncode.
                if let Some(text_value) = prompt.pointer_mut("/9/inputs/seed") {
                     *text_value = json!(seed);
                } else {
                    panic!("Failed to set seed");
                }


                // Connect to the websocket.
                let (client, client_id, mut ws) = comfy::connect_comfy().await.unwrap();

                // Wait for execution to complete and download the images.
                let images = comfy::get_images(&mut ws, &client, &prompt, &client_id).await.unwrap();

                ws.close(None).await.unwrap();   
                assert!(images.len() == 1, "No images generated");        

                for (_node_id, images_vec) in images.iter() {
                    for (_i, image_data) in images_vec.iter().enumerate() {
                        let file_path = Path::new("assets").join(&image_path);
                        tokio::fs::write(&file_path, image_data).await.unwrap();
                        info!("Saved image to {:?}", file_path);
                    }
                }

                ctx.run_on_main_thread(move |ctx| {                    
                    if let Some(mut p) = ctx.world.get_mut::<Prefab>(e) {
                        match &mut p.workflow {
                            Workflow::TextToImage { image, .. } => {
                                *image = Some(image_path.clone());                                    
                            }
                            _ => {}
                        }
                    }
                }).await;
                dbg!("generated images: {}", images.len());
            });


            
        }
        Workflow::TextToModel {
            ..
        } => {},
    }        
}
