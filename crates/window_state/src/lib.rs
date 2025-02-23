use std::path::Path;

use bevy::{prelude::*, window::WindowResized};
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use crate::{WindowState, WindowStatePlugin};
}

pub struct WindowStatePlugin;

impl Plugin for WindowStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_window_moved)
            .add_systems(Update, on_window_resized);
            //.add_systems(Update, move_all_windows_on_arrow_keys);
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct WindowState {
    pub position: (i32, i32),
    pub size: (u32, u32),
}

impl WindowState {
    pub fn build(name: &str, position: (i32, i32), size: (u32, u32)) -> Persistent<WindowState> {
        assert!(!name.is_empty(), "name must not be empty");

        let state_dir = dirs::state_dir()
            .map(|native_state_dir| native_state_dir.join(name))
            .unwrap_or(Path::new("local").join("state").join(name));

        Persistent::<WindowState>::builder()
            .name("window state")
            .format(StorageFormat::Ron)
            .path(state_dir.join("window-state.toml"))
            .default(WindowState { position, size })
            .build()
            .expect("failed to initialize window state")
    }
}

// TODO: this is never firing on wayland popos 24.04
fn on_window_moved(
    events: EventReader<WindowMoved>,
    windows: Query<&Window>,
    window_state: ResMut<Persistent<WindowState>>,
) {
    if !events.is_empty() {
        update_window_state(window_state, windows.single());
    }
}

fn on_window_resized(
    events: EventReader<WindowResized>,
    windows: Query<&Window>,
    window_state: ResMut<Persistent<WindowState>>,
) {
    if !events.is_empty() {
        update_window_state(window_state, windows.single());
    }
}

fn update_window_state(mut window_state: ResMut<Persistent<WindowState>>, window: &Window) {
    let position = match &window.position {
        WindowPosition::At(position) => (position.x, position.y),
        _ => unreachable!(),
    };
    let size = (
        window.resolution.physical_width(),
        window.resolution.physical_height(),
    );

    if window_state.position != position || window_state.size != size {
        // info!(
        //     "updating window state: position: {:?}, size: {:?}",
        //     position, size
        // );
        window_state.set(WindowState { position, size }).ok();
    }
}

// fn move_all_windows_on_arrow_keys(
//     mut keyboard_input: EventReader<KeyboardInput>,
//     mut window_query: Query<(Entity, &mut Window)>,
// ) {
//     for event in keyboard_input.read() {
//         let mut move_x = 0;
//         let mut move_y = 0;
//         match event.key_code {
//             KeyCode::ArrowLeft => move_x = -10,
//             KeyCode::ArrowRight => move_x = 10,
//             KeyCode::ArrowUp => move_y = -10,
//             KeyCode::ArrowDown => move_y = 10,
//             _ => {}
//         }

//         if move_x != 0 || move_y != 0 {
//             for (_, mut window) in window_query.iter_mut() {
//                 if let WindowPosition::At(position) = window.position {
//                     info!(
//                         "moving window from {:?} by ({}, {})",
//                         position, move_x, move_y
//                     );
//                     window.position = WindowPosition::At(position + IVec2::new(move_x, move_y));
//                 }
//             }
//         }
//     }
// }
