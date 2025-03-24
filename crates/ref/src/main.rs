mod actions;
pub use actions::*;
mod assets;
pub use assets::*;
mod camera;
pub use camera::*;

use bevy::{
    app::AppExit,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
};
use avian3d::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use iyes_progress::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, Reflect)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
}

#[derive(Component, Deref, DerefMut)]
struct RefImage(pub String);

// created when dragging starts, remove when done
#[derive(Debug, Resource)]
struct Dragging(pub Entity);

// created when scolling starts, remove when done
#[derive(Debug, Resource)]
struct Scaling(pub Entity);

// created when order starts, remove when done
#[derive(Debug, Resource)]
struct Order(pub Entity);

#[derive(Event)]
pub struct SaveEvent;

#[derive(Event)]
pub struct DeleteEvent(pub Entity);


const CONFIG_FILE: &str = ".ref.json";

fn main() {
    App::new()
        .add_plugins((
            sly_common::SlyCommonPlugin { title: "sly_ref" },
            MeshPickingPlugin,
            ProgressPlugin::<AppState>::new()
                .with_state_transition(AppState::Loading, AppState::Playing),
            PhysicsPlugins::default(), // using for collision detection
            RonAssetPlugin::<RefConfig>::new(&["ref.ron"]),
            InputManagerPlugin::<Action>::default(),
            #[cfg(feature = "dev")]
            dev::plugin,
        ))
        .init_state::<AppState>()
        .init_resource::<ActionState<Action>>()
        .insert_resource(Action::input_map())
        .init_resource::<SaveTimer>()
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Playing)
                .load_collection::<RefAssets>(),
        )
        .add_event::<SaveEvent>()
        .add_event::<DeleteEvent>()
        // not: trying something new, keep systems here for the most part since this should be small app
        // hoping to make it simpler to reason about
        .add_systems(Startup, setup)
        .add_systems(OnExit(AppState::Loading), load)
        .add_systems(
            Update,
            (
                file_drag_and_drop,
                camera::zoom_camera,
                camera::move_camera,
                set_save.run_if(on_event::<SaveEvent>),
                image_delete.run_if(on_event::<DeleteEvent>),
                autosave,
                (
                    image_drag.run_if(resource_exists::<Dragging>),
                    image_scale.run_if(resource_exists::<Scaling>),
                    image_order.run_if(resource_exists::<Order>),
                )
                    .chain(),
            )
                .run_if(in_state(AppState::Playing)),
        )
        // before we exit, check if we need to save
        .add_systems(PostUpdate, save_on_exit.run_if(on_event::<AppExit>))
        .run();
}

fn on_click_spawn_cube(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut num: Local<usize>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.25 + 0.55 * *num as f32, 0.0),
        ))
        // With the MeshPickingPlugin added, you can add pointer event observers to meshes:
        .observe(on_drag_rotate);
    *num += 1;
}

fn on_drag_rotate(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.target) {
        transform.rotate_y(drag.delta.x * 0.02);
        transform.rotate_x(drag.delta.y * 0.02);
    }
}

fn setup(
    mut commands: Commands,

    mut clear_color: ResMut<ClearColor>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    //mut meshes: ResMut<Assets<Mesh>>,
) {
    clear_color.0 = Color::srgb(0.1, 0.1, 0.1);

    commands
        .spawn((
            Text::new("Click Me to get a box\nDrag cubes to rotate"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(12.0),
                left: Val::Percent(12.0),
                ..default()
            },
        ))
        .observe(on_click_spawn_cube)
        .observe(
            |out: Trigger<Pointer<Out>>, mut texts: Query<&mut TextColor>| {
                let mut text_color = texts.get_mut(out.target).unwrap();
                text_color.0 = Color::WHITE;
            },
        )
        .observe(
            |over: Trigger<Pointer<Over>>, mut texts: Query<&mut TextColor>| {
                let mut color = texts.get_mut(over.target).unwrap();
                color.0 = bevy::color::palettes::tailwind::CYAN_400.into();
            },
        );

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::None,
        Bloom {
            intensity: 0.2,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("MainCamera"),
        PlaneCamera::default(),
    ));

    // commands.spawn((
    //     RayCaster::new(Vec3::ZERO, Direction3d::X).with_query_filter(SpatialQueryFilter::default()),
    //     PickCaster,
    //     Name::new("MousePicker"),
    // ));
}

fn load(
    mut commands: Commands,
    ref_assets: Res<RefAssets>,
    ref_files: Res<Assets<RefConfig>>,
    images: Res<Assets<Image>>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection, &PlaneCamera)>,
) {
    info!("Loaded: {} images", ref_assets.images.len());
    let config_file = ref_files.get(&ref_assets.config.clone()).unwrap();

    let Ok((mut camera_trans, mut camera_projection, _plane)) = camera.get_single_mut() else {
        warn_once!("No camera found for load!");
        return;
    };
    camera_projection.scale = config_file.camera_scale;
    camera_trans.translation = config_file.camera_position;

    for (name, config) in ref_assets.images.iter() {
        let image = images.get(config).unwrap();

        // search our loaded config file for this image
        let (pos, scale) = if let Some(c) = config_file.images.iter().find(|x| &x.image == name) {
            (c.position, c.scale)
        } else {
            (Vec3::ZERO, 1.0) // just put it in the middle for now
        };

        // get image size
        let size = Vec2::new(image.width() as f32, image.height() as f32);
        commands.spawn((
            Sprite {
                image: config.clone(),
                ..default()
            },
            Transform::from_translation(pos).with_scale(Vec3::splat(scale)),
            RigidBody::Static,
            Collider::cuboid(size.x, size.y, 0.05),
            Name::new(name.clone()),
            RefImage(name.clone()),
        ));
    }
}

fn image_drag(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<RefImage>>,
    mut camera: Query<(&OrthographicProjection, &PlaneCamera)>,
    actions: Res<ActionState<Action>>,
    time: Res<Time>,
    dragging: Res<Dragging>,
    mut save: EventWriter<SaveEvent>,
) {
    let Ok((camera_projection, plane)) = camera.get_single_mut() else {
        warn_once!("No camera found for zooming!");
        return;
    };

    if let Ok(mut transform) = query.get_mut(dragging.0) {
        let axis_pair = actions.axis_pair(&Action::MoveDrag);
        let change = Vec3::new(axis_pair.x, -axis_pair.y, 0.0)
            * (plane.drag_speed * camera_projection.scale)
            * time.delta_secs();
        transform.translation += change;
    } else {
        warn!("No transform found for entity: {:?}", dragging.0);
    }

    if actions.just_released(&Action::SelectMove) {
        save.send(SaveEvent);
        commands.remove_resource::<Dragging>();
    }
}

fn image_scale(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<RefImage>>,
    mut camera: Query<(&OrthographicProjection, &PlaneCamera)>,
    actions: Res<ActionState<Action>>,
    time: Res<Time>,
    scaling: Res<Scaling>,
    mut save: EventWriter<SaveEvent>,
) {
    let Ok((camera_projection, plane)) = camera.get_single_mut() else {
        warn_once!("No camera found for zooming!");
        return;
    };

    if let Ok(mut transform) = query.get_mut(scaling.0) {
        let axis_pair = actions.axis_pair(&Action::MoveDrag);
        let change =
            axis_pair.x * (plane.scale_speed * camera_projection.scale) * time.delta_secs();
        transform.scale += change;
        transform.scale = transform.scale.max(Vec3::splat(0.1));
    } else {
        warn!("No transform found for entity: {:?}", scaling.0);
    }

    if actions.just_released(&Action::SelectScale) {
        save.send(SaveEvent);
        commands.remove_resource::<Scaling>();
    }
}

fn image_order(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<RefImage>>,
    mut camera: Query<(&OrthographicProjection, &PlaneCamera)>,
    actions: Res<ActionState<Action>>,
    time: Res<Time>,
    order: Res<Order>,
    mut save: EventWriter<SaveEvent>,
) {
    let Ok((camera_projection, plane)) = camera.get_single_mut() else {
        warn_once!("No camera found for zooming!");
        return;
    };

    if let Ok(mut transform) = query.get_mut(order.0) {
        let axis_pair = actions.axis_pair(&Action::MoveDrag);
        let change =
            axis_pair.x * (plane.scale_speed * camera_projection.scale) * time.delta_secs();
        transform.translation.z += change;
        transform.scale = transform.scale.max(Vec3::splat(0.1));
    } else {
        warn!("No transform found for entity: {:?}", order.0);
    }

    if actions.just_released(&Action::SelectScale) {
        save.send(SaveEvent);
        commands.remove_resource::<Scaling>();
    }
}

#[derive(Debug, Resource)]
pub struct SaveTimer {
    pub timer: Timer,
    pub need_save: bool,
}

impl Default for SaveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            need_save: false,
        }
    }
}

fn set_save(mut save_timer: ResMut<SaveTimer>) {
    save_timer.need_save = true;
}

fn image_delete(
    mut commands: Commands,
    query: Query<&RefImage>,
    mut delete_events: EventReader<DeleteEvent>,
) {
    for DeleteEvent(e) in delete_events.read() {
        if let Ok(image) = query.get(*e) {
            commands.entity(*e).despawn_recursive();
            info!("Deleted image: {:?}", image.0);
            std::fs::remove_file(&image.0).unwrap();
        }
    }
}

fn autosave(
    mut save_timer: ResMut<SaveTimer>,
    cameras: Query<(&Transform, &OrthographicProjection, &PlaneCamera)>,
    query: Query<(&Transform, &RefImage)>,
    time: Res<Time>,
) {
    if save_timer.timer.tick(time.delta()).just_finished() {
        if save_timer.need_save {
            save(cameras, query);
        }
        save_timer.need_save = false;
        save_timer.timer.reset();
    }
}

fn save_on_exit(
    cameras: Query<(&Transform, &OrthographicProjection, &PlaneCamera)>,
    query: Query<(&Transform, &RefImage)>,
    save_timer: Res<SaveTimer>,
) {
    if save_timer.need_save {
        save(cameras, query);
    }
}

// This saves reads world state and saves it to a file
fn save(
    cameras: Query<(&Transform, &OrthographicProjection, &PlaneCamera), ()>,
    query: Query<(&Transform, &RefImage), ()>,
) {
    let Ok((camera_trans, camera_projection, _plane)) = cameras.get_single() else {
        warn_once!("No camera found for save!");
        return;
    };

    let mut file = RefConfig {
        camera_position: camera_trans.translation,
        camera_scale: camera_projection.scale,
        images: Vec::new(),
    };

    for (trans, path) in query.iter() {
        file.images.push(PositionedImage {
            image: path.0.clone(),
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
    match std::fs::write(CONFIG_FILE, s) {
        Ok(_) => {
            info!("Saved file: {:?} - {} images", CONFIG_FILE, image_count);
        }
        Err(e) => {
            error_once!("Save failed: {:?}\n{:?}", CONFIG_FILE, e);
        }
    };
}

fn file_drag_and_drop(mut events: EventReader<FileDragAndDrop>) {
    for event in events.read() {
        info!("{:?}", event);
        // TODO
    }
}
