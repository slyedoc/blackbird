use bevy::prelude::*;

use crate::ui::FadeTo;

use super::AppState;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(AppState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(AppState::Splash).and(resource_exists::<SplashTimer>)));
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((Camera2d::default(), StateScoped(AppState::Splash)));
    let icon = asset_server.load("branding/icon.png");
    // Display the logo
    commands.spawn((
        StateScoped(AppState::Splash),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        OnSplashScreen,
        children![(
            ImageNode::new(icon),
            Node {
                // This will set the logo to be 200px wide, and auto adjust its height
                width: Val::Px(200.0),
                ..default()
            },
        )],
    ));
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    if timer.tick(time.delta()).finished() {        
        commands.remove_resource::<SplashTimer>();
        commands.send_event(FadeTo(AppState::Menu));
    }
}
