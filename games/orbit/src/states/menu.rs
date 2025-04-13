use crate::music::MyMusic;
use crate::prefabs::solar_system::SolarSystem;
use crate::skybox::Cubemap;
use crate::states::AppState;
use crate::ui::*;
use bevy::audio::Volume;
use bevy::core_pipeline::Skybox;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::ecs::system::command;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            //.enable_state_scoped_entities::<MenuState>()
            .add_systems(OnEnter(AppState::Menu), setup)
            .add_systems(OnEnter(MenuState::Main), setup_main)
            .add_systems(OnEnter(MenuState::Settings), setup_settings)
            .add_systems(
                Update,
                update_settings.run_if(in_state(MenuState::Settings)),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::Menu)]
#[states(scoped_entities)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let skybox_handle = asset_server.load("textures/skybox/space.ktx2");

    commands.spawn((
        Name::new("MainCamera"),
        StateScoped(AppState::Menu),
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Skybox {
            image: asset_server.load("textures/skybox/space.ktx2"),
            brightness: 10000.0,
            ..default()
        },
        Bloom {
            intensity: 0.1, // the default is 0.3,
            ..default()
        },
        // Skybox {
        //     brightness: 5000.0,
        //     image: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //     ..default()
        // },
        // EnvironmentMapLight {
        //     diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
        //     specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //     intensity: 2500.0,
        //     ..default()
        // },
        // movement
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));

    // commands.insert_resource(Cubemap {
    //     is_loaded: false,
    //     image_handle: skybox_handle,
    //     activated: true,
    // });

    commands.spawn((
        StateScoped(AppState::Menu),
        SolarSystem::default(),
        Name::new("Sol"),
        Transform::default(),
    ));
}

fn setup_main(mut commands: Commands, asset_server: Res<AssetServer>, ui: Res<UiAssets>) {
    let font = ui.font.clone();

    commands
        .spawn((
            Name::new("Menu Panel"),
            StateScoped(MenuState::Main),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(20.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderRadius::all(Val::Px(5.)),
            BackgroundColor(PANEL_BACKGROUND),
            BorderColor(PANEL_BORDER),
            // Outline {
            //     width: Val::Px(2.),
            //     color: Color::WHITE,
            //     ..default()
            // },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(30.0)),
                    ..default()
                },
                children![(Name::new("Title"), Text::new("Orbit"), H1,)],
            ));

            parent
                .spawn((
                    Name::new("New Game Button"),
                    MenuButton,
                    children!((
                        MenuButtonInner,
                        //ImageNode::new(asset_server.load("textures/icon/white/plus.png")),
                        Text("New Game".to_string()),
                    )),
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>,
                     mut commands: Commands| {
                        commands.send_event(FadeTo(AppState::Intro));
                        
                    },
                );

            parent
                .spawn((
                    Name::new("Resume Button"),
                    MenuButton,
                    children!((
                        MenuButtonInner,
                        Text::new("*Resume*"),
                        //ImageNode::new(asset_server.load("textures/icon/white/checkmark.png")),
                    )),
                ))
                .observe(|_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        //commands.send_event(Save);
                    },
                );

            parent
                .spawn((
                    Name::new("Settings Button"),
                    MenuButton,
                    children!((
                        MenuButtonInner,
                        Text::new("Settings"),
                        //ImageNode::new(asset_server.load("textures/icon/white/checkmark.png")),
                    )),
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.set_state(MenuState::Settings);
                    },
                );

            #[cfg(not(target_arch = "wasm32"))]
            parent
                .spawn((
                    Name::new("Exit Button"),
                    MenuButton,
                    children!((
                        MenuButtonInner,
                        Text::new("Exit"),
                        //ImageNode::new(asset_server.load("textures/icon/white/exitRight.png")),
                    )),
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.send_event(AppExit::Success);
                    },
                );
        });

    commands.spawn((
        Name::new("Version Text"),
        //StateScoped(MenuState::Main),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(5.),
            bottom: Val::Px(5.),
            ..default()
        },
        children!((
            Text("Version: 0.1.0".to_string()),
            //ImageNode::new(asset_server.load("textures/icon/white/exitRight.png")),
        )),
    ));
}

fn setup_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui: Res<UiAssets>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let font = ui.font.clone();

    let settings_panel = commands
        .spawn((
            Name::new("Settings"),
            StateScoped(MenuState::Settings),
            Node {
                //position_type: PositionType::Absolute,
                left: Val::Percent(10.),
                width: Val::Percent(80.),
                height: Val::Percent(80.),

                padding: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderRadius::all(Val::Px(5.)),
            BackgroundColor(PANEL_BACKGROUND),
            BorderColor(PANEL_BORDER),
            // Outline {
            //     width: Val::Px(2.),
            //     color: Color::WHITE,
            //     ..default()
            // },
        ))
        .id();

    commands.spawn((
        ChildOf {
            parent: settings_panel,
        },
        Node {
            padding: UiRect::all(Val::Px(30.0)),
            ..default()
        },
        children![(Name::new("Title"), Text::new("Settings"), H1,)],
    ));

    let row = commands
        .spawn((
            ChildOf {
                parent: settings_panel,
            },
            Node {
                flex_direction: FlexDirection::Row,
                ..default()
            },
            children![(
                Text::new("Volume"),
                //ImageNode::new(asset_server.load("textures/icon/white/plus.png")),
            ),],
        ))
        .id();

    commands
        .spawn((ChildOf { parent: row }, Slider { value: 0.5 }))
        .observe(
            |trigger: Trigger<SliderChanged>,
             mut commands: Commands,
             mut music_controller: Single<&mut AudioSink, With<MyMusic>>| {
                music_controller.set_volume(Volume::Linear(trigger.event().value));
            },
        );

    commands
        .spawn((
            ChildOf {
                parent: settings_panel,
            },
            Name::new("Exit Button"),
            MenuButton,
            children!((
                MenuButtonInner,
                Text::new("Exit"),
                //ImageNode::new(asset_server.load("textures/icon/white/exitRight.png")),
            )),
        ))
        .observe(
            |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                commands.set_state(MenuState::Main);
            },
        );

    commands.spawn((
        Name::new("Version Text"),
        StateScoped(MenuState::Settings),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(5.),
            bottom: Val::Px(5.),
            ..default()
        },
        children!((
            Text("Version: 0.1.0".to_string()),
            //ImageNode::new(asset_server.load("textures/icon/white/exitRight.png")),
        )),
    ));
}

fn update_settings(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::Escape) {
        commands.set_state(MenuState::Main);
    }
}
