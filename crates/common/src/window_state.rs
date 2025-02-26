use bevy::prelude::*;

pub struct WindowStatePlugin;

impl Plugin for WindowStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.system());
    }
}