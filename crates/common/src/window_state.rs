use bevy::{prelude::*, window::WindowResized};
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

pub struct WindowStatePlugin;

impl Plugin for WindowStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_window_moved)
            .add_systems(Update, on_window_resized);
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct WindowState {
    pub position: (i32, i32),
    pub size: (u32, u32),
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
