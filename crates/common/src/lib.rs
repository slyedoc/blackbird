use bevy::{log::LogPlugin, prelude::* };

#[cfg(not(target_arch = "wasm32"))]
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use bevy_persistent::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::window::{WindowPosition, WindowResized, WindowResolution};
// #[cfg(target_arch = "wasm32")]
// use bus::prelude::*;

pub mod prelude {
    pub use crate::SlyDefaultPlugins;
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::WindowState;
}

pub struct SlyDefaultPlugins {
    pub title: String,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub canvas_id: String,

}

impl Default for SlyDefaultPlugins {
    fn default() -> Self {
        Self {
            title: "Sly".to_string(),
            position: (0, 0),
            size: (800, 600),
            canvas_id: "#bevy".to_string(),    
        }
    }
}

impl Plugin for SlyDefaultPlugins {
    fn build(&self, app: &mut App) {
        assert!(!self.title.is_empty(), "title must not be empty");
                
        // for desktop get Persistent Window Info from config file
        #[cfg(not(target_arch = "wasm32"))]
        let state_dir = dirs::state_dir()
            .map(|native_state_dir| native_state_dir.join(self.title.clone()))
            .unwrap_or(std::path::Path::new("local").join("state").join(self.title.clone()));

        #[cfg(not(target_arch = "wasm32"))]
        let window_state = Persistent::<WindowState>::builder()
            .name("window state")
            .format(StorageFormat::Ron)
            .path(state_dir.join("window-state.toml"))
            .default(WindowState {
                position: self.position,
                size: self.size,
            })
            .build()
            .expect("failed to initialize window state");

        // setup default plugins
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    #[cfg(target_arch = "wasm32")]
                    primary_window: Some(Window {
                        title: self.title.clone(),
                        canvas: Some(self.canvas_id.clone()),
                        ..default()
                    }),
                    #[cfg(not(target_arch = "wasm32"))]
                    primary_window: Some(Window {
                        title: self.title.clone(),
                        position: WindowPosition::At(IVec2::from(window_state.position)),
                        resolution: WindowResolution::new(
                            window_state.size.0 as f32,
                            window_state.size.1 as f32,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter:
                        "wgpu_hal=error,wgpu_core=error,bevy_render=error,bevy_persistent=error"
                            .into(),
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
            #[cfg(feature = "editor")]
            sly_editor::SlyEditorPlugin,
        ));

        #[cfg(not(target_arch = "wasm32"))]
        app.insert_resource(window_state)
            .add_systems(Update, on_window_moved)
            .add_systems(Update, on_window_resized);

    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Resource, Serialize, Deserialize)]
pub struct WindowState {
    pub position: (i32, i32),
    pub size: (u32, u32),
}


// TODO: this is never firing on wayland popos 24.04
#[cfg(not(target_arch = "wasm32"))]
fn on_window_moved(
    events: EventReader<WindowMoved>,
    windows: Query<&Window>,
    window_state: ResMut<Persistent<WindowState>>,
) {
    if !events.is_empty() {
        update_window_state(window_state, windows.single());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn on_window_resized(
    events: EventReader<WindowResized>,
    windows: Query<&Window>,
    window_state: ResMut<Persistent<WindowState>>,
) {
    if !events.is_empty() {
        update_window_state(window_state, windows.single());
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
