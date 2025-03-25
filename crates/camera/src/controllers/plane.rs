// use bevy::prelude::*;
// use serde::{Deserialize, Serialize};

// use crate::*;

// #[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
// #[reflect(Component, Serialize, Deserialize)]
// pub struct PlaneCamera {
//     pub zoom_rate: f32,
//     pub drag_speed: f32,
//     pub scale_speed: f32,
//     pub order_speed: f32,
//     pub bounds: Option<(Vec3, Vec3)>, // min, max
//     pub local: bool,
// }

// impl Default for PlaneCamera {
//     fn default() -> Self {
//         Self {
//             zoom_rate: 0.1,
//             drag_speed: 100.0,
//             scale_speed: 0.1,
//             order_speed: 0.1,
//             bounds: None,
//             local: true,
//         }
//     }
// }

// pub fn move_camera(
//     mut cameras: Query<(
//         &mut Transform,
//         &OrthographicProjection,
//         &PlaneCamera,
//     )>,
//     actions: Res<ActionState<Action>>,
//     time: Res<Time>,
//     mut save: EventWriter<SaveEvent>,
// ) {
//     let Ok((mut camera_trans, camera_proj, plane)) = cameras.get_single_mut() else {
//         return;
//     };

//     // If any button in a virtual direction pad is pressed, then the action state is
//     // "pressed"
//     if actions.pressed(&Action::Move) {
//         // Virtual direction pads are one of the types which return a DualAxis. The
//         // values will be represented as `-1.0`, `0.0`, or `1.0` depending on
//         // the combination of buttons pressed.
//         let axis_pair = actions.axis_pair(&Action::Move);
//         let point = if plane.local {
//             camera_trans.forward() * axis_pair.y + camera_trans.right() * axis_pair.x
//         } else {
//             -Vec3::Z * axis_pair.y + Vec3::X * axis_pair.x
//         };
//         let change = point * plane.drag_speed * camera_proj.scale * time.delta_secs();
//         camera_trans.translation += change;
//         if let Some(bounds) = plane.bounds {
//             camera_trans.translation = camera_trans.translation.clamp(bounds.0, bounds.1);
//         }
//         save.send(SaveEvent);
//     }

//     if actions.pressed(&Action::SelectMoveCamera) && actions.pressed(&Action::MoveDrag) {
//         let axis_pair = actions.axis_pair(&Action::MoveDrag);
//         let change = Vec3::new(-axis_pair.x, axis_pair.y, 0.0)
//             * plane.drag_speed
//             * camera_proj.scale
//             * time.delta_secs();
//         camera_trans.translation += change;
//         if let Some(bounds) = plane.bounds {
//             camera_trans.translation = camera_trans.translation.clamp(bounds.0, bounds.1);
//         }
//         save.send(SaveEvent);
//     }
// }

// pub fn zoom_camera(
//     mut query: Query<(
//         &mut OrthographicProjection,
//         &PlaneCamera,
//     )>,
//     actions: Res<ActionState<Action>>,
//     mut save: EventWriter<SaveEvent>,
// ) {
//     let Ok((mut camera_projection, plane)) = query.get_single_mut() else {
//         warn_once!("No camera found for zooming!");
//         return;
//     };
//     // Here, we use the `action_value` method to extract the total net amount that
//     // the mouse wheel has travelled Up and right axis movements are always
//     // positive by default
//     let zoom_delta = actions.value(&Action::Zoom);

//     // We want to zoom in when we use mouse wheel up,
//     // so we increase the scale proportionally
//     // Note that the projection's scale should always be positive (or our images
//     // will flip)
//     if zoom_delta != 0.0 {
//         camera_projection.scale *= 1. - zoom_delta * plane.zoom_rate;
//         save.send(SaveEvent);
//     }
// }
