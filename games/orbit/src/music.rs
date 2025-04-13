use bevy::{audio::Volume, prelude::*};

pub struct MusicPlugin;
impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (pause, mute, volume));
 
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/music/1 - Astro Reverie (Loop).ogg")),
        MyMusic,
    ));

    // example instructions
    commands.spawn((
        Text::new("-/=: Volume Down/Up\nSpace: Toggle Playback\nM: Toggle Mute"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct MyMusic;

// fn update_speed(music_controller: Query<&AudioSink, With<MyMusic>>, time: Res<Time>) {
//     let Ok(sink) = music_controller.single() else {
//         return;
//     };

//     sink.set_speed((ops::sin(time.elapsed_secs() / 5.0) + 1.0).max(0.1));
// }

fn pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<MyMusic>>,
) {
    let Ok(sink) = music_controller.single() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        sink.toggle_playback();
    }
}

fn mute(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut music_controller: Query<&mut AudioSink, With<MyMusic>>,
) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::KeyM) {
        sink.toggle_mute();
    }
}

fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut music_controller: Query<&mut AudioSink, With<MyMusic>>,
) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Equal) {
        let current_volume = sink.volume();
        sink.set_volume(current_volume + Volume::Linear(0.1));
    } else if keyboard_input.just_pressed(KeyCode::Minus) {
        let current_volume = sink.volume();
        sink.set_volume(current_volume - Volume::Linear(0.1));
    }
}
