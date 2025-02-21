use std::f32::consts::PI;

use crate::prelude::*;
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
                    draw_mesh_intersections,
                    rotate,
                    exit.run_if(action_just_pressed(MenuAction::Exit)),
                ).run_if(in_state(AppState::Menu)),
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

fn exit(mut commands: Commands) {
    commands.send_event(AppExit::Success);
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
            Node {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            StateScoped(AppState::Menu),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(200.),
                        // horizontally center child text
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((Button, Node::default(), BackgroundColor(NORMAL_BUTTON)))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Play"),
                                    TextFont {
                                        font_size: 33.0,
                                        ..Default::default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ))
                                .observe(
                                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                                        commands.set_state(AppState::Play);
                                    },
                                );
                        });

                    parent
                        .spawn((Button, Node::default(), BackgroundColor(NORMAL_BUTTON)))
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
                                        commands.send_event(AppExit::Success);
                                    },
                                );
                        });
                });
        });
}

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component)]
struct Shape;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Set up the materials.
    let white_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Tetrahedron::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Cone::default()),
        meshes.add(ConicalFrustum::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let extrusions = [
        meshes.add(Extrusion::new(Rectangle::default(), 1.)),
        meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
        meshes.add(Extrusion::new(Annulus::default(), 1.)),
        meshes.add(Extrusion::new(Circle::default(), 1.)),
        meshes.add(Extrusion::new(Ellipse::default(), 1.)),
        meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
        meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
    ];

    let num_shapes = shapes.len();

    // Spawn the shapes. The meshes will be pickable by default.
    for (i, shape) in shapes.into_iter().enumerate() {
        commands
            .spawn((
                Mesh3d(shape),
                MeshMaterial3d(white_matl.clone()),
                Transform::from_xyz(
                    -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
                    2.0,
                    Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                Shape,
                StateScoped(AppState::Menu),
            ))
            .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
            .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
            .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
            .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()))
            .observe(rotate_on_drag);
    }

    let num_extrusions = extrusions.len();

    for (i, shape) in extrusions.into_iter().enumerate() {
        commands
            .spawn((
                Mesh3d(shape),
                MeshMaterial3d(white_matl.clone()),
                Transform::from_xyz(
                    -EXTRUSION_X_EXTENT / 2.
                        + i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
                    2.0,
                    -Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                Shape,
                StateScoped(AppState::Menu),
            ))
            .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
            .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
            .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
            .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()))
            .observe(rotate_on_drag);
    }

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(ground_matl.clone()),
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

    // Camera
    commands.spawn((
        Camera3d::default(),
        StateScoped(AppState::Menu),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}

/// A system that rotates all shapes.
fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

/// An observer to rotate an entity when it is dragged
fn rotate_on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.entity()).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}
