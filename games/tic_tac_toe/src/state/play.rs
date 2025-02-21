use crate::prelude::*;
use bevy::prelude::*;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Play), setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        StateScoped(AppState::Play),
    ));

    commands
        .spawn((
            Node {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..Default::default()
            },
            StateScoped(AppState::Play),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Text::new("Exit"),
                            TextFont {
                                font_size: 33.0,
                                ..Default::default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ))
                        .observe(
                            |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                                commands.set_state(AppState::Menu);
                            },
                        );
                });
        });


    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        TicTacToeBoard::default(),
        StateScoped(AppState::Play),
    ));
}
