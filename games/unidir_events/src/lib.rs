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
use bevy::prelude::*;

pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        sly_common::SlyCommonPlugin {
            title: "Unidirectional Events".into(),
        },
        MeshPickingPlugin,
        // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
    ))
    .init_resource::<CurrentText>()
    .init_resource::<SelectedGlyph>()
    .add_systems(Startup, (setup_scene,))
    .add_systems(Update, update_text);

    app
}
