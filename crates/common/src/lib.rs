mod water;
pub use water::*;

use bevy::prelude::*;

pub struct SlyCommonPlugin {
    pub title: &'static str,
}

impl Plugin for SlyCommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            #[cfg(not(target_arch = "wasm32"))]
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: self.title.to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                }),
            #[cfg(target_arch = "wasm32")]
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: self.title.to_string(),
                        fit_canvas_to_parent: false,
                        focused: false,
                        canvas: Some("#bevy_canvas".to_string()),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    file_path: "/assets".to_string(),
                    processed_file_path: "/imported_assets/Default".to_string(),
                    ..default()
                })
                .disable::<bevy::log::LogPlugin>(),
            #[cfg(feature = "editor")]
            sly_editor::SlyEditorPlugin,
        ));
        // https://github.com/bevyengine/bevy/issues/12126
        // #[cfg(target_arch = "wasm32")]
        // app.insert_resource(WinitSettings {
        //     focused_mode: UpdateMode::Continuous,
        //     unfocused_mode: UpdateMode::Continuous,
        // });
    }
}

// DefaultPlugins
//     .set(WindowPlugin {
//         primary_window: Some(Window {
//             title: title.into(),
//             #[cfg(target_arch = "wasm32")]
//             canvas: Some("#bevy_canvas".to_string()),
//             ..default()
//         }),
//         ..default()
//     })
//     .set(AssetPlugin {
//         mode: AssetMode::Processed,
//         #[cfg(target_arch = "wasm32")]
//         meta_check: bevy::asset::AssetMetaCheck::Never,
//         ..default()
//     })
//     .disable::<LogPlugin>()

// #[cfg(not(target_arch = "wasm32"))]
// mod window_state;
// #[cfg(not(target_arch = "wasm32"))]
// use window_state::*;

// use bevy::{log::LogPlugin, prelude::*};

// #[cfg(not(target_arch = "wasm32"))]
// use bevy_persistent::prelude::*;

// #[cfg(not(target_arch = "wasm32"))]
// use bevy::window::{WindowPosition, WindowResolution};
// // #[cfg(target_arch = "wasm32")]
// // use bus::prelude::*;

// pub mod prelude {
//     #[cfg(not(target_arch = "wasm32"))]
//     pub use crate::window_state::WindowState;
//     pub use crate::SlyDefaultPlugins;
// }

// pub struct SlyDefaultPlugins {
//     pub title: String,
//     pub position: (i32, i32),
//     pub size: (u32, u32),
//     pub canvas_id: String,
// }

// impl Default for SlyDefaultPlugins {
//     fn default() -> Self {
//         Self {
//             title: "Sly".to_string(),
//             position: (0, 0),
//             size: (800, 600),
//             canvas_id: "#bevy_canvas".to_string(),
//         }
//     }
// }

// impl Plugin for SlyDefaultPlugins {
//     fn build(&self, app: &mut App) {
//         assert!(!self.title.is_empty(), "title must not be empty");

//         // for desktop get Persistent Window Info from config file
//         #[cfg(not(target_arch = "wasm32"))]
//         let state_dir = dirs::state_dir()
//             .map(|native_state_dir| native_state_dir.join(self.title.clone()))
//             .unwrap_or(
//                 std::path::Path::new("local")
//                     .join("state")
//                     .join(self.title.clone()),
//             );

//         #[cfg(not(target_arch = "wasm32"))]
//         let window_state = Persistent::<WindowState>::builder()
//             .name("window state")
//             .format(StorageFormat::Ron)
//             .path(state_dir.join("window-state.toml"))
//             .default(WindowState {
//                 position: self.position,
//                 size: self.size,
//             })
//             .build()
//             .expect("failed to initialize window state");
//         #[cfg(not(target_arch = "wasm32"))]
//         app.insert_resource(window_state)
//             .add_plugins(WindowStatePlugin);

//         // setup default plugins
//         app.add_plugins((
//             DefaultPlugins
//                 .set(AssetPlugin {
//                     #[cfg(target_arch = "wasm32")]
//                     meta_check: bevy::asset::AssetMetaCheck::Never,
//                     ..default()
//                 })
//                 .set(WindowPlugin {
//                     #[cfg(target_arch = "wasm32")]
//                     primary_window: Some(Window {
//                         focused: false,
//                         fit_canvas_to_parent: true,
//                         //title: self.title.clone(),
//                         canvas: Some(self.canvas_id.clone()),
//                         ..default()
//                     }),
//                     #[cfg(not(target_arch = "wasm32"))]
//                     primary_window: Some(Window {
//                         title: self.title.clone(),
//                         position: WindowPosition::At(IVec2::from(self.position)),
//                         resolution: WindowResolution::new(self.size.0 as f32, self.size.1 as f32),
//                         ..default()
//                     }),
//                     ..default()
//                 })
//                 // .set(LogPlugin {
//                 //     filter:
//                 //         "wgpu_hal=error,wgpu_core=error,bevy_render=error,bevy_persistent=error"
//                 //             .into(),
//                 //     level: bevy::log::Level::INFO,
//                 //     ..default()
//                 // })
//                 .disable::<LogPlugin>(),
//             #[cfg(feature = "editor")]
//             sly_editor::SlyEditorPlugin,
//         ));
//     }
// }
