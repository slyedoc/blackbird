use std::f32::consts::PI;

use crate::{exit, prelude::*, GAME_NAME};
use bevy::{color::palettes::tailwind::GRAY_300, picking::pointer::PointerInteraction, prelude::*};
use leafwing_input_manager::common_conditions::action_just_pressed;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default())
            .init_resource::<ActionState<MenuAction>>()
            .insert_resource(MenuAction::input_map())
            .add_systems(OnEnter(AppState::Menu), (setup_scene, setup_ui))
            .add_systems(
                Update,
                (
                    exit.run_if(action_just_pressed(MenuAction::Exit)),
                )
                    .run_if(in_state(AppState::Menu)),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum MenuAction {
    Exit,
}

impl MenuAction {
    fn input_map() -> InputMap<MenuAction> {
        use MenuAction::*;
        let mut input_map = InputMap::default();
        input_map.insert(Exit, KeyCode::Escape);
        input_map
    }
}



fn setup_ui(mut commands: Commands) {
    // Instructions
    commands.spawn((
        Text::new("Hover over the shapes to pick them\nDrag to rotate"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        StateScoped(AppState::Menu),
    ));

    commands
        .spawn((
            Name::new("menu"),
            Node {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            PickingBehavior::IGNORE,
            StateScoped(AppState::Menu),
        ))
        .with_children(|p| {
            p.spawn((
                MenuPanel,
                Node {
                    width: Val::Px(400.),
                    padding: UiRect::all(Val::Px(10.0)),
                    // horizontally center child text
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(10.)),
                BorderColor(Color::from(GRAY_300)),
                Outline {
                    width: Val::Px(6.),
                    color: Color::WHITE,
                    ..default()
                },
            ))
            .with_children(|parent| {
                // title
                parent
                    .spawn((Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        // horizontally center child text
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(GAME_NAME),
                            TextFont {
                                font_size: 48.0,
                                ..Default::default()
                            },
                        ));
                    });

                // play
                parent.spawn(MenuButton).with_children(|parent| {
                    parent.spawn((MenuButtonText, Text::new("Play"))).observe(
                        |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                            commands.set_state(AppState::Play);
                        },
                    );
                });

                // exit
                parent.spawn(MenuButton).with_children(|parent| {
                    parent.spawn((MenuButtonText, Text::new("Exit"))).observe(
                        |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                            commands.send_event(AppExit::Success);
                        },
                    );
                });
            });
        });
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        IsDefaultUiCamera,
        UiBoxShadowSamples(6),
        StateScoped(AppState::Menu),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(GRAY_300))),
        StateScoped(AppState::Menu),
        PickingBehavior::IGNORE, // Disable picking for the ground plane.
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        StateScoped(AppState::Menu),
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));
}