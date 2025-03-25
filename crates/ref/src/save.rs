use bevy::prelude::*;

use crate::{AppState, PositionedImage, RefConfig, RefImage};

/// A timer resource used to save the game state periodically.
#[derive(Debug, Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct SaveTimer(pub Timer);

impl Default for SaveTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(60., TimerMode::Repeating))
    }
}

#[derive(Event, Reflect)]
pub struct SaveEvent;

// save ever so often
pub fn autosave(mut commands: Commands, mut save_timer: ResMut<SaveTimer>, time: Res<Time>) {
    if save_timer.tick(time.delta()).just_finished() {
        info!("Autosaving...");
        commands.send_event(SaveEvent);
    }
}

// save when 
pub fn save_on_exit(mut commands: Commands) {
    commands.send_event(SaveEvent);
}

// save the current state of the world
pub fn save(
    camera: Single<&Transform, With<Camera>>,
    query: Query<(&Transform, &RefImage, &Name)>,
    state: Res<State<AppState>>,
) {
    info!("Saving state: {:?}", state.get());

    let mut file = RefConfig {
        camera_position: camera.translation,
        images: Vec::new(),
    };

    for (trans, i, name) in query.iter() {
        file.images.push(PositionedImage {
            name: name.as_str().to_string(),
            path: i.path.clone(),
            position: trans.translation,
            scale: trans.scale.x,
        });
    }
    let image_count = file.images.len();

    use ron::ser::{to_string_pretty, PrettyConfig};
    let pretty = PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let s = to_string_pretty(&file, pretty).expect("Serialization failed");

    let root = std::env::var("BEVY_ASSET_ROOT").unwrap_or("".to_string());
    let file_path = std::path::Path::new(&root).join("assets/ref/config.ron");

    match std::fs::write(&file_path, s) {
        Ok(_) => {
            info!("Saved file: {:?} - {} images", &file_path, image_count);
        }
        Err(e) => {
            error_once!("Save failed: {:?}\n{:?}", &file_path, e);
        }
    };
}