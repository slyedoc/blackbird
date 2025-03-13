mod components;
pub mod events;
mod resources;
mod setup;
mod systems;

use resources::*;
use setup::setup_scene;
use systems::*;
pub const RENDER_HEIGHT: f32 = 600.;
pub const RENDER_WIDTH: f32 = 800.;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    fit_canvas_to_parent: true,
                    canvas: Some("#bevy_canvas".into()),
                    resolution: WindowResolution::new(RENDER_WIDTH, RENDER_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
        MeshPickingPlugin,
        // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
    ))
    .init_resource::<CurrentText>()
    .init_resource::<SelectedGlyph>()

    .add_systems(Startup, (setup_scene,))
    .add_systems(Update, update_text);

    app
}
