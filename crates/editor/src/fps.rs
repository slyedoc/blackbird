use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Resource)]
pub struct FpsAssets {
    pub top: Val,
    pub right: Val,
}

impl Default for FpsAssets {
    fn default() -> Self {
        Self {
            top: Val::Px(10.0),
            right: Val::Px(5.0),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<FpsAssets>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_fps_text);
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands, ass: Res<FpsAssets>) {
    commands
        .spawn((
            FpsText,
            Text::new("FPS: "),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: ass.top,
                right: ass.right,
                ..default()
            },
        ))
        .with_children(|b| {
            b.spawn((
                TextSpan::new("0"),
                // TextFont {
                //     font_size: 20.0,
                //     ..default()
                // },
            ));
        });
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<Entity, With<FpsText>>,
    mut writer: Text2dWriter,
) {
    for e in &query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                *writer.text(e, 1) = format!("{value:.0}");
            }
        }
    }
}
