mod camera;
pub use camera::*;

mod spell;
pub use spell::*;

mod ui;
pub use ui::*;

use avian3d::{math::Quaternion, prelude::*};
use bevy::{
    color::palettes::tailwind,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_hanabi::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            HanabiPlugin,
            ui::plugin,
            spell::plugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_grounded,
                movement,
                apply_movement_damping,
                debug_movement,
            )
                .chain(),
        )
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Jump,
    #[actionlike(Axis)]
    Zoom,
    #[actionlike(DualAxis)]
    Cursor,
    CursorGrab,
    ToggleCursorGrab,
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    let input_map = InputMap::default()
        .with_dual_axis(PlayerAction::Move, VirtualDPad::wasd())
        .with(PlayerAction::Jump, KeyCode::Space)
        .with_axis(PlayerAction::Zoom, MouseScrollAxis::Y)
        .with_dual_axis(PlayerAction::Cursor, MouseMove::default())
        .with(PlayerAction::CursorGrab, MouseButton::Right)
        .with(PlayerAction::CursorGrab, MouseButton::Left);

    let player_collider = Collider::capsule(0.4, 1.0);
    let player_seplls = PlayerSpells {
        spells: vec![Spell::FrostBolt, Spell::ArcaneExplosion, Spell::Blizzard],
    };
    let mut caster_shape = player_collider.clone();
    caster_shape.set_scale(Vec3::ONE * 0.99, 10);

    // Player
    // camera
    let player_camera = cmd
        .spawn((
            Camera3d::default(),
            Tonemapping::None,
            Bloom {
                intensity: 0.2,
                ..default()
            },
            PlayerCameraController::new(0.5, 0.0, 0.0),
            Transform::from_xyz(0.0, 2.0, 8.0),
        ))
        .id();

    cmd
        .spawn((
            Player,
            PlayerCamera(player_camera),
            player_seplls.clone(),
            Transform::from_xyz(0.0, 1.5, 0.0),
            InputManagerBundle::with_map(input_map),
            // physics
            RigidBody::Dynamic,
            Collider::capsule(0.4, 1.0),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            GravityScale(2.0),
            ShapeCaster::new(caster_shape, Vec3::ZERO, Quaternion::default(), Dir3::NEG_Y)
                .with_max_distance(0.2),
            LockedAxes::ROTATION_LOCKED,
        ))
        .insert((
            // movement
            MovementAcceleration(30.0),
            MovementDampingFactor(0.92),
            JumpImpulse(7.0),
            MaxSlopeAngle((30.0 as f32).to_radians()),
            // visual
            Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        ))
        .with_children(|parent| {
            // eyes
            parent.spawn((
                Transform::from_xyz(0.0, 0.5, -0.5),
                Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(0.2)))),
                MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            ));
        });

    // Static physics object with a collision shape
    cmd.spawn((
        RigidBody::Static,
        Collider::cylinder(4.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(4.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    // cube
    cmd.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.0)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.8, 0.7, 0.6))),
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    cmd.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));

    // ground
    cmd.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.1, 0.7, 0.1))),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
    ));

    // Light
    cmd.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // ui
    cmd.spawn(Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(20.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    })
    .with_children(|parent| {
        for s in player_seplls.spells.iter() {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    s.clone(),
                ))
                .with_child((
                    Text::new(format!("{:?}", s)),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        }
    });
}

/// Returns an observer that updates the entity's material to the one specified.
fn on_spell_click(
    player: Entity,
) -> impl Fn(
    Trigger<Pointer<Click>>,
    (
        Query<&Spell>,
        Query<&Transform, With<Player>>,
        &mut Commands,
        SpellEffects,
    ),
) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, (mut button_query, mut player_query, cmd, spell_effects)| {
        if let Ok(spell) = button_query.get_mut(trigger.target) {
            //material.0 = new_material.clone();
            dbg!(player, spell);

            let player_trans = player_query.get_mut(player).unwrap();
            let effect = &spell_effects.hashmap[spell];
            cmd.spawn((
                Name::new("firework"),
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect.clone()),
                    transform: Transform::from_translation(player_trans.translation),
                    ..Default::default()
                },
            ));
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Clone)]
struct PlayerSpells {
    spells: Vec<Spell>,
}

#[derive(Component)]
struct PlayerCamera(Entity);

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(f32);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(f32);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(f32);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(f32);

// Query for the `ActionState` component in your game logic systems!
fn movement(
    time: Res<Time>,
    mut controllers: Query<
        (
            &ActionState<PlayerAction>,
            &MovementAcceleration,
            &JumpImpulse,
            &mut LinearVelocity,
            &PlayerCamera,
            &Transform,
            Has<Grounded>,
        ),
        With<Player>,
    >,
    mut camera_query: Query<(&mut Transform, &mut PlayerCameraController), Without<Player>>,
    mut gizmos: Gizmos,
) {
    let delta_time = time.delta_secs();

    for (
        action_state,
        movement_acceleration,
        jump_impulse,
        mut linear_velocity,
        PlayerCamera(player_camera),
        player_trans,
        is_grounded,
    ) in &mut controllers
    {
        let movement = action_state.clamped_axis_pair(&PlayerAction::Move);
        if movement != Vec2::ZERO {
            linear_velocity.x += movement.x * movement_acceleration.0 * delta_time;
            linear_velocity.z -= movement.y * movement_acceleration.0 * delta_time;
        }

        if action_state.pressed(&PlayerAction::Jump) && is_grounded {
            linear_velocity.y = jump_impulse.0;
        }

        // Rotate around the player's position
        let player_translation = player_trans.translation;

        gizmos.line(
            player_translation,
            player_translation + -Vec3::Z,
            tailwind::RED_300,
        );

        // Update the child camera
        if let Ok((mut transform, mut cam)) = camera_query.get_mut(*player_camera) {
            cam.add_zoom(action_state.clamped_value(&PlayerAction::Zoom) * delta_time * 5.0);
            // Handle cursor rotation when grabbed
            if action_state.pressed(&PlayerAction::CursorGrab) {
                let cursor = action_state.axis_pair(&PlayerAction::Cursor);
                let rotation_speed = 0.5; // Adjust for sensitivity
                cam.add_rotation(
                    -cursor.x * rotation_speed * delta_time,
                    -cursor.y * rotation_speed * delta_time,
                );
                dbg!(cam.yaw, cam.pitch);
            }

            // Calculate the offset vector from the player to the camera

            let offset = Vec3::new(0.0, 2.0, 8.0) * cam.zoom;
            let yaw_rotation = Quat::from_rotation_y(cam.yaw);
            let pitch_rotation = Quat::from_rotation_x(cam.pitch);

            let combined_rotation = yaw_rotation * pitch_rotation;

            // Rotate the offset vector around the player's position
            let rotated_offset = combined_rotation * offset.normalize() * offset.length();

            // Update the camera's position and ensure it rotates around the player
            transform.translation = player_translation + rotated_offset;

            gizmos.line(
                player_translation,
                transform.translation,
                tailwind::GREEN_500,
            );

            // Ensure the camera is always looking at the player
            transform.look_at(player_translation, Vec3::Y);
        }
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>), With<Player>>,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vec3::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Slows down movement in the XZ plane.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

fn debug_movement(query: Query<&Transform, With<Player>>, mut gizmos: Gizmos) {
    for trans in &query {
        gizmos.line(
            trans.translation,
            trans.translation + -Vec3::Z,
            tailwind::RED_300,
        );
    }
}
