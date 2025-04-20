use avian3d::parry::query;
use bevy::{input::keyboard::{Key, KeyboardInput}, pbr::wireframe::Wireframe, picking::backend::ray::RayMap, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::{EguiContext, input::egui_wants_any_pointer_input}, bevy_inspector::hierarchy::{SelectedEntities, SelectionMode}};

use crate::{in_editor, EditorState};

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Selected>()
        .add_systems(
            Update,
            (
                raycast.run_if(in_editor.and( not(egui_wants_any_pointer_input))),
            ),
        ).add_systems(PostUpdate, (
            update_selection.run_if(in_editor),
        ))
        .add_systems(OnExit(EditorState::Enabled), remove_selectet);
    }
}

#[derive(Resource, Default)]
pub struct Selected(pub SelectedEntities);

fn raycast(
    mut ray_cast: MeshRayCast,

    time: Res<Time>,
    // The ray map stores rays cast by the cursor
    ray_map: Res<RayMap>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<Selected>,
    mut gizmos: Gizmos,
    mut egui_context: Single<&mut EguiContext, With<PrimaryWindow>>,
) {
    let (camera, camera_transform) = *camera_query;
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    let Some((e, hit)) = ray_cast
        .cast_ray(ray, &MeshRayCastSettings::default())
        .first()
    else {
        return;
    };

    let mut ctrl = false;
    let mut shift = false;
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        shift = true;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) {
        ctrl = true;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        let mode = SelectionMode::from_ctrl_shift(ctrl, shift);
        match mode {
            SelectionMode::Add => {
                selected.0.select(mode, *e, |_, _| std::iter::empty());
            }
            SelectionMode::Extend => {
                selected.0.select(mode, *e, |_, _| std::iter::empty());
            }
            SelectionMode::Replace => {
                selected.0.select_replace(*e);
            }
        }
    }

    gizmos.sphere(hit.point, 0.1, Color::WHITE);
}

#[derive(Component)]
pub struct SelectMarker;

fn update_selection(
    mut selected: ResMut<Selected>,
    mut gizmos: Gizmos,
    query: Query<Entity, With<SelectMarker>>, 
    mut commands: Commands,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
) {

    for e in query.iter() {
        if !selected.0.contains(e) {
            commands.entity(e)
                .remove::<SelectMarker>()
                .remove::<Wireframe>();  
            for child in children.iter_descendants(e) {
                commands.entity(child).remove::<Wireframe>();  
            }  
        }
        
    }
    
    for e in selected.0.iter() {
        commands.entity(e).insert((
            SelectMarker,
            Wireframe,
        ));                 
        // add wireframe to selected entity 
        for child in children.iter_descendants(e) {
            commands.entity(child).insert((                
                Wireframe,
            ));                 
        }
    } 
}


fn remove_selectet(query: Query<Entity, With<SelectMarker>>,
    children: Query<&Children>,
     mut commands: Commands
) {
    for e in query.iter() {
        commands.entity(e)
        .remove::<SelectMarker>()
        .remove::<Wireframe>();

        for child in children.iter_descendants(e) {
            commands.entity(child).remove::<Wireframe>();  
        }  
    }
    
}