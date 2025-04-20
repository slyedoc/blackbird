#![allow(warnings)]

#[cfg(feature = "avian3d")]
use avian3d::{debug_render::PhysicsDebugPlugin, prelude::*};
use bevy::{
    color::palettes::{css, tailwind},
    dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin},
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::hierarchy,
    gizmos::light::LightGizmoPlugin,
    input::common_conditions::{input_just_pressed, input_toggle_active},
    pbr::{
        irradiance_volume::IrradianceVolume,
        wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    },
    picking::{backend::ray::RayMap, pointer::PointerInteraction},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiContextPass, EguiPlugin},
    bevy_inspector::{
        hierarchy::{Hierarchy, SelectedEntities, SelectionMode},
        ui_for_all_assets, ui_for_entities, ui_for_resources,
    },
    egui,
    quick::{ResourceInspectorPlugin, WorldInspectorPlugin},
    DefaultInspectorConfigPlugin,
};
mod select;
use select::*;
mod prefab;
pub use prefab::*;

mod stepping;
pub use stepping::*;

pub mod prelude {
    pub use crate::{stepping::*, *};
}

#[cfg(feature = "fps")]
mod fps;

pub struct SlyEditorPlugin;

impl Plugin for SlyEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WireframePlugin::default(),
            DebugPickingPlugin::default(),
            #[cfg(feature = "avian3d")]
            PhysicsDebugPlugin::default(),
            //ResourceInspectorPlugin::<LevelSetup>::default(),
            //#[cfg(feature = "debug")]
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            DefaultInspectorConfigPlugin,
            //WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
            // ResourceInspectorPlugin::<AppStatus>::default()
            //     .run_if(input_toggle_active(true, KeyCode::F1)),
            // TODO: come back to stepping debugging
            // SteppingPlugin::default()
            //     .add_schedule(Update)
            //     .add_schedule(FixedUpdate)
            //     .at(Val::Percent(35.0), Val::Percent(50.0)),
            #[cfg(feature = "fps")]
            fps::FpsPlugin,
            select::SelectPlugin,
            prefab::PrefabPlugin,
        ))
        .init_resource::<Selected>()
        .init_state::<EditorState>()
        .enable_state_scoped_entities::<EditorState>()
        .add_systems(Startup, setup)
        //.add_systems(Update, draw_mesh_intersections)
        .add_systems(
            Update,
            (
                toggle_editor.run_if(input_just_pressed(KeyCode::F1)),
                //toggle_editor.run_if(input_toggle_active(false, KeyCode::Backquote)),
                toggle_physics.run_if(input_just_pressed(KeyCode::F2)),
                toggle_wireframe.run_if(input_just_pressed(KeyCode::F3)),
                toggle_aabb.run_if(input_just_pressed(KeyCode::F4)),
                toggle_directional_light_atmspheric_fog_influence
                    .run_if(input_just_pressed(KeyCode::F6)),
                toggle_atmspheric_fog.run_if(input_just_pressed(KeyCode::F7)),
                //toggle_navmesh.run_if(input_just_pressed(KeyCode::F3)),
                // toggle_picking.run_if(action_just_pressed(DebugAction::TogglePicking)),
                //toggle_voxel_visibility.run_if(input_just_pressed(KeyCode::F8)),
                //toggle_irradiance_volumes.run_if(input_just_pressed(KeyCode::F9)),
                //reload_scene.run_if(action_just_pressed(DebugAction::Reload)),
            ),
        )
        .add_systems(EguiContextPass, inspector_ui.run_if(in_editor))
        .add_systems(
            PreUpdate,
            (|mut mode: ResMut<DebugPickingMode>| {
                *mode = match *mode {
                    DebugPickingMode::Disabled => DebugPickingMode::Normal,
                    DebugPickingMode::Normal => DebugPickingMode::Noisy,
                    DebugPickingMode::Noisy => DebugPickingMode::Disabled,
                }
            })
            .distributive_run_if(input_just_pressed(KeyCode::F5)),
        );
        // .add_systems(
        //     Update,
        //     draw_irradiance_volume,
        // );
    }
}

fn inspector_ui(world: &mut World) {
    world.resource_scope(|world, mut selected_entities: Mut<Selected>| {
        let mut egui_context = world
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .single(world)
            .expect("EguiContext not found")
            .clone();

        egui::SidePanel::left("hierarchy")
            .default_width(200.0)
            .show(egui_context.get_mut(), |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    // bevy_inspector::ui_for_world(world, ui);
                    // ui.allocate_space(ui.available_size());

                    ui.heading("Hierarchy");
                    egui::CollapsingHeader::new("Entities")
                        .default_open(true)
                        .show(ui, |ui| {
                            // ui_for_entities(world, ui);
                            let type_registry = world.resource::<AppTypeRegistry>().clone();
                            let type_registry = type_registry.read();

                            let mut hierarchy = Hierarchy {
                                world,
                                type_registry: &type_registry,
                                selected: &mut selected_entities.0,
                                context_menu: None,
                                shortcircuit_entity: None,
                                extra_state: &mut (),
                            };
                            if hierarchy.show_with_default_filter::<()>(ui) {
                                // selected changed
                            };
                        });
                    egui::CollapsingHeader::new("Resources").show(ui, |ui| {
                        ui_for_resources(world, ui);
                    });
                    egui::CollapsingHeader::new("Assets").show(ui, |ui| {
                        ui_for_all_assets(world, ui);
                    });

                    ui.label("Press ` to toggle UI");
                    ui.allocate_space(ui.available_size());
                });
            });

        egui::SidePanel::right("inspector")
            .default_width(250.0)
            .show(egui_context.get_mut(), |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    if ui.button("Prefab").clicked() {
                        match selected_entities.0.as_slice() {
                            &[entity] => {
                                world.trigger_targets(BuildPrefab, entity);
                            }
                            entities => {
                                warn_once!("more than one selected, not creating prefab");
                            }
                        }
                    }

                    ui.heading("Inspector");

                    match selected_entities.0.as_slice() {
                        &[entity] => {
                            bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                        }
                        entities => {
                            bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                                world, entities, ui,
                            );
                        }
                    }

                    ui.allocate_space(ui.available_size());
                });
            });
    });
}

static GIZMO_COLOR: Color = Color::Srgba(css::YELLOW);

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum EditorState {
    Enabled,
    #[default]
    Disabled,
}

#[allow(dead_code)]
pub fn in_editor(state: Res<State<EditorState>>) -> bool {
    match state.get() {
        EditorState::Enabled => true,
        EditorState::Disabled => false,
    }
}

fn setup(mut config_store: ResMut<GizmoConfigStore>) {
    // disable PhysicsGizmos
    #[cfg(feature = "avian3d")]
    {
        let config = config_store.config_mut::<PhysicsGizmos>().0;
        config.enabled = false;
    }
}

fn toggle_aabb(mut config_store: ResMut<GizmoConfigStore>) {
    let (store, aabb) = config_store.config_mut::<AabbGizmoConfigGroup>();
    store.enabled = !store.enabled;
    aabb.draw_all = true;
}

#[cfg(feature = "avian3d")]
fn toggle_physics(mut config_store: ResMut<GizmoConfigStore>) {
    let (store, _physics) = config_store.config_mut::<PhysicsGizmos>();
    store.enabled = !store.enabled;
}

// fn toggle_navmesh(
//     query: Query<Entity, With<NavMeshSettings>>,
//     mut commands: Commands,
//     mut enabled: Local<bool>,
// ) {
//     *enabled = !*enabled;

//     for entity in query.iter() {
//         if *enabled {
//             commands
//                 .entity(entity)
//                 .insert(NavMeshDebug(tailwind::RED_400.into()));
//         } else {
//             commands.entity(entity).remove::<NavMeshDebug>();
//         }
//     }
// }

// fn toggle_picking(mut mode: ResMut<DebugPickingMode>) {
//     *mode = match *mode {
//         DebugPickingMode::Disabled => DebugPickingMode::Normal,
//         _ => DebugPickingMode::Disabled,
//     };
// }

fn toggle_editor(mut next_state: ResMut<NextState<EditorState>>, state: Res<State<EditorState>>) {
    match state.get() {
        EditorState::Enabled => {
            next_state.set(EditorState::Disabled);
        }
        EditorState::Disabled => {
            next_state.set(EditorState::Enabled);
        }
    }
}

fn toggle_wireframe(mut config: ResMut<WireframeConfig>) {
    config.global = !config.global;
}

// Handles a request from the user to toggle the voxel visibility on and off.
// fn toggle_voxel_visibility(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut voxel_cube_parent_query: Query<&mut Visibility, With<VoxelCubeParent>>,
// ) {

//     for mut visibility in voxel_cube_parent_query.iter_mut() {
//         match *visibility {
//             Visibility::Visible => *visibility = Visibility::Hidden,
//             Visibility::Hidden => *visibility = Visibility::Visible,
//             _ => (),
//         }
//     }
// }

// Turns on and off the irradiance volume as requested by the user.
// fn toggle_irradiance_volumes(
//     mut commands: Commands,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     light_probe_query: Query<Entity, With<LightProbe>>,
//     mut app_status: ResMut<AppStatus>,
//     assets: Res<ExampleAssets>,
//     mut ambient_light: ResMut<AmbientLight>,
// ) {
//     let Some(light_probe) = light_probe_query.iter().next() else {
//         return;
//     };

//     if app_status.irradiance_volume_present {
//         commands.entity(light_probe).remove::<IrradianceVolume>();
//         ambient_light.brightness = app_status.ambient_brightness * app_status.irradiance_volume_intensity;
//         app_status.irradiance_volume_present = false;
//     } else {
//         commands.entity(light_probe).insert(IrradianceVolume {
//             voxels: assets.irradiance_volume.clone(),
//             intensity: app_status.irradiance_volume_intensity,
//             ..default()
//         });
//         ambient_light.brightness = 0.0;
//         app_status.irradiance_volume_present = true;
//     }
// }

// // Draws a gizmo showing the bounds of the irradiance volume.
// fn draw_irradiance_volume(
//     mut gizmos: Gizmos,
//     irradiance_volume_query: Query<&GlobalTransform, With<IrradianceVolume>>,
//     app_status: Res<AppStatus>,
// ) {
//     if app_status.voxels_visible {
//         for transform in irradiance_volume_query.iter() {
//             gizmos.cuboid(*transform, GIZMO_COLOR);
//         }
//     }
// }

fn toggle_atmspheric_fog(mut fog: Single<&mut DistanceFog>) {
    let a = fog.color.alpha();
    fog.color.set_alpha(1.0 - a);
}

fn toggle_directional_light_atmspheric_fog_influence(mut fog: Single<&mut DistanceFog>) {
    let a = fog.directional_light_color.alpha();
    fog.directional_light_color.set_alpha(0.5 - a);
}

#[allow(dead_code)]
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, tailwind::RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, tailwind::PINK_100);
    }
}
