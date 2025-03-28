#![allow(warnings)]

#[cfg(feature = "avian3d")]
use avian3d::{debug_render::PhysicsDebugPlugin, prelude::*};
use bevy::{
    color::palettes::{css, tailwind}, dev_tools::ui_debug_overlay::DebugUiPlugin, diagnostic::FrameTimeDiagnosticsPlugin, gizmos::light::LightGizmoPlugin, input::common_conditions::{input_just_pressed, input_toggle_active}, pbr::{
        irradiance_volume::IrradianceVolume,
        wireframe::{WireframeConfig, WireframePlugin},
    }, picking::pointer::PointerInteraction, prelude::*
};
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};

mod selected;
use fps::FpsPlugin;
pub use selected::*;

mod stepping;
pub use stepping::*;

pub mod prelude {
    pub use crate::{selected::*, stepping::*, *};
}

#[cfg(feature = "fps")]
mod fps;

pub struct SlyEditorPlugin;

impl Plugin for SlyEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WireframePlugin,
            #[cfg(feature = "avian3d")]
            PhysicsDebugPlugin::default(),
            DebugUiPlugin,
            //ResourceInspectorPlugin::<LevelSetup>::default(),
            //#[cfg(feature = "debug")]
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
            // ResourceInspectorPlugin::<AppStatus>::default()
            //     .run_if(input_toggle_active(true, KeyCode::F1)),
            // TODO: come back to stepping debugging
            SteppingPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate)
                .at(Val::Percent(35.0), Val::Percent(50.0)),
            EditorSelectedPlugin,
            #[cfg(feature = "fps")]
            FpsPlugin,
        ))
        .init_state::<EditorState>()
        .enable_state_scoped_entities::<EditorState>()
        .add_systems(Startup, setup)
        //.add_systems(Update, draw_mesh_intersections)
        .add_systems(
            Update,
            (
                toggle_editor.run_if(input_toggle_active(false, KeyCode::Backquote)),
                toggle_ui_overlay.run_if(input_just_pressed(KeyCode::F2)),
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
        );
        // .add_systems(
        //     Update,
        //     draw_irradiance_volume,
        // );
    }
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

// The system that will enable/disable the debug outlines around the nodes
fn toggle_ui_overlay(mut options: ResMut<bevy::dev_tools::ui_debug_overlay::UiDebugOptions>) {
    options.toggle();
}

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
