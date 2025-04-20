use std::{fs::File, io::Write};

use avian3d::parry::na;
use bevy::{
    pbr::experimental::meshlet::{
        MeshletMesh, MeshletMesh3d, MeshletPlugin,
        MESHLET_DEFAULT_VERTEX_POSITION_QUANTIZATION_FACTOR,
    },
    prelude::*,
    render::{primitives::Aabb, render_resource::AsBindGroup},
    state::commands,
    tasks::IoTaskPool,
    transform,
};

/// The initial scene file will be loaded below and not change when the scene is saved.
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

/// The new, updated scene data will be saved here so that you can see the changes.
const NEW_SCENE_FILE_PATH: &str = "scenes/new-scene.scn.ron";

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MeshletPlugin {
                cluster_buffer_slots: 8192,
            },
            MaterialPlugin::<MeshletDebugMaterial>::default(),
        ))
        .add_observer(on_build_prefab);
    }
}

#[derive(Event)]
pub struct BuildPrefab;

pub fn on_build_prefab(
    trigger: Trigger<BuildPrefab>,
    mut meshlet_meshes: ResMut<Assets<MeshletMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut debug_materials: ResMut<Assets<MeshletDebugMaterial>>,
    mut entities: Query<(
        &Transform,
        &GlobalTransform,
        Option<&Name>,
        Option<&Children>,
        Option<&Mesh3d>,
        Option<&MeshMaterial3d<StandardMaterial>>,
        Option<&Aabb>,
    )>,
    mut commands: Commands,
) {
    let e = trigger.target();

    let mut stack = Vec::new();
    let mut new_entity = commands.spawn((Transform::default(),)).id();
    let debug_material = debug_materials.add(MeshletDebugMaterial::default());

    stack.push((e, new_entity));

    while let Some((from, to)) = stack.pop() {
        let (transform, global_transform, name_opt, children_opt, mesh_opt, mat_opt, aabb_opt) =
            entities.get(from).unwrap();
        let mut trans = transform.clone();
        if from == e {
            trans.translation += vec3(0.0, 20., 0.0);
        }
        // update pos
        commands.entity(to).insert((trans, Visibility::default()));

        if let Some(name) = name_opt {
            // Create a new meshlet entity
            commands.entity(to).insert(name.clone());
        }

        // copy mesh
        if let Some(mesh_3d) = mesh_opt {
            let material = mat_opt.unwrap();
            // Clone the mesh and material
            let mut mesh = meshes.get_mut(mesh_3d).unwrap().clone();

            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_1);
            mesh.remove_attribute(Mesh::ATTRIBUTE_TANGENT);
            mesh.remove_attribute(Mesh::ATTRIBUTE_COLOR);

            let meshletmesh =
                MeshletMesh::from_mesh(&mesh, MESHLET_DEFAULT_VERTEX_POSITION_QUANTIZATION_FACTOR);

            match meshletmesh {
                Ok(meshlet) => {
                    let m = meshlet_meshes.add(meshlet);
                    // Create a new meshlet entity
                    commands.entity(to).insert((
                        MeshletMesh3d(m),
                        MeshMaterial3d(material.0.clone()),
                        //MeshMaterial3d(debug_material.clone()),
                    ));
                }
                Err(e) => {
                    error!("Failed to create meshlet: {:?}", e);
                    continue;
                }
            }

        }

        if let Some(aabb) = aabb_opt {
            // Create a new meshlet entity
            commands.entity(to).insert(aabb.clone());
        }

        if let Some(children) = children_opt {
            for child in children.iter() {
                let new_child = commands.spawn((ChildOf(to), Transform::default())).id();
                stack.push((child, new_child));
            }
        } else {
        }

        // copy material
        // if let Some(mat) = mat_opt {
        //     // Clone the material
        //     let material = materials.get(mat).unwrap().clone();

        //     // Create a new meshlet entity
        //     let new_material = commands.entity(new_e).insert(MeshMaterial3d(mat.0.clone()));
        // }
    }

    commands.queue(SavePrefab(new_entity));
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
struct MeshletDebugMaterial {
    _dummy: (),
}
impl Material for MeshletDebugMaterial {}

pub struct SavePrefab(pub Entity);

impl Command for SavePrefab {
    fn apply(self, world: &mut World) {
        info!("Saving prefab: {:?}", self.0);

        // find all children
        let mut entites = Vec::new();
        entites.push(self.0);
        
        let mut vecdeque = Vec::new();
        vecdeque.push(self.0);

        while let Some(e) = vecdeque.pop() {
            let children = world.get::<Children>(e);
            if let Some(children) = children {
                for child in children.iter() {
                    vecdeque.push(child);
                    entites.push(child);
                }
            }
        }
        
        let scene = DynamicSceneBuilder::from_world(&world)
            .extract_entities(entites.into_iter())
            .deny_component::<MeshMaterial3d<StandardMaterial>>()

            .build();

        // Scenes can be serialized like this:
        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();
        let serialized_scene = scene.serialize(&type_registry).unwrap();
        dbg!(&serialized_scene);

        let name = world
            .get::<Name>(self.0)
            .map(|name| name.as_str().to_string())
            .unwrap_or("Unnamed".to_string());

        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file
                File::create(format!("assets/scenes/{name}.scn.ron"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
    }
}
