use crate::{define_on_controller_enabled_changed, CameraController, LookAngles, LookTransform, Smoother};

use bevy::{
    prelude::*, input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},        reflect::Reflect
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct OrbitCameraPlugin {
    pub override_input_system: bool,
}

impl OrbitCameraPlugin {
    pub fn new(override_input_system: bool) -> Self {
        Self {
            override_input_system,
        }
    }
}

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            //.add_systems(PreUpdate, on_controller_enabled_changed)
            .add_systems(Update, (on_controller_enabled_changed, control_system).chain())
            .add_event::<ControlEvent>();

        if !self.override_input_system {
            app.add_systems(Update, default_input_map);
        }
    }
}


/// A 3rd person camera that orbits around the target.
#[derive(Clone, Component, Copy, Debug, Reflect, Deserialize, Serialize)]
#[reflect(Component, Default, Debug)]
#[require(LookTransform, Transform, CameraController)]
pub struct OrbitCameraController {    
    pub mouse_rotate_sensitivity: Vec2,
    pub mouse_translate_sensitivity: Vec2,
    pub mouse_wheel_zoom_sensitivity: f32,
    pub pixels_per_line: f32,
    pub smoothing_weight: f32,
}

impl Default for OrbitCameraController {
    fn default() -> Self {
        Self {
            mouse_rotate_sensitivity: Vec2::splat(0.08),
            mouse_translate_sensitivity: Vec2::splat(0.1),
            mouse_wheel_zoom_sensitivity: 0.2,
            smoothing_weight: 0.8,            
            pixels_per_line: 53.0,
        }
    }
}

#[derive(Event)]
enum ControlEvent {
    Orbit(Vec2),
    TranslateTarget(Vec2),
    Zoom(f32),
}

define_on_controller_enabled_changed!(OrbitCameraController);

fn default_input_map(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    controllers: Query<(&OrbitCameraController, &CameraController)>,
) {
    // Can only control one camera at a time.
    let controller = if let Some((o, _controller)) = controllers.iter().find(|(_, c)| c.enabled) {
        o
    } else {
        return;
    };
    let OrbitCameraController {
        mouse_rotate_sensitivity,
        mouse_translate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        cursor_delta += event.delta;
    }

    if keyboard.pressed(KeyCode::ControlLeft) {
        events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        events.send(ControlEvent::TranslateTarget(
            mouse_translate_sensitivity * cursor_delta,
        ));
    }

    let mut scalar = 1.0;
    for event in mouse_wheel_reader.read() {
        // scale the event magnitude per pixel or per line
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / pixels_per_line,
        };
        scalar *= 1.0 - scroll_amount * mouse_wheel_zoom_sensitivity;
    }
    events.send(ControlEvent::Zoom(scalar));
}

fn control_system(
    time: Res<Time>,
    mut events: EventReader<ControlEvent>,
    mut cameras: Query<(&OrbitCameraController, &mut LookTransform, &Transform, &CameraController)>,
) {
    // Can only control one camera at a time.
    let (mut transform, scene_transform) = if let Some((_, transform, scene_transform, _)) = cameras.iter_mut().find(|c| c.3.enabled) {
            (transform, scene_transform)
        } else {
            return;
        };

    let mut look_angles = LookAngles::from_vector(-transform.look_direction().unwrap());
    let mut radius_scalar = 1.0;
    let radius = transform.radius();

    let dt = time.delta_secs();
    for event in events.read() {
        match event {
            ControlEvent::Orbit(delta) => {
                look_angles.add_yaw(dt * -delta.x);
                look_angles.add_pitch(dt * delta.y);
            }
            ControlEvent::TranslateTarget(delta) => {
                let right_dir = scene_transform.rotation * -Vec3::X;
                let up_dir = scene_transform.rotation * Vec3::Y;
                transform.target += dt * delta.x * right_dir + dt * delta.y * up_dir;
            }
            ControlEvent::Zoom(scalar) => {
                radius_scalar *= scalar;
            }
        }
    }

    look_angles.assert_not_looking_up();

    let new_radius = (radius_scalar * radius).min(1000000.0).max(0.001);
    transform.eye = transform.target + new_radius * look_angles.unit_vector();
}
