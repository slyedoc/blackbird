use core::time::Duration;

use bevy::{color::palettes::tailwind, ecs::{component::HookContext, event, world::DeferredWorld}, prelude::*};
use bevy_tweening::{lens::*, *};

use crate::states::AppState;


const FADE_DURATION_SECS: f32 = 1.0;

pub struct FadePlugin;

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FadeTo>().add_systems(Update, (on_fade_to, loop_complete));
    }
}

#[derive(Event, Debug, Deref, DerefMut)]
pub struct FadeTo(pub AppState);

#[derive(Component)]
#[require(
    ZIndex = ZIndex(2),
    Node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    },
    BackgroundColor = BackgroundColor(Color::NONE),    
)]
#[component(on_add = on_add_fade)]
struct FadeOverlay {
    // clear when used
    target: Option<AppState>,     
}

fn on_add_fade(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    
  // Run around the window from corner to corner
  let dests = &[
    Color::NONE,
    Color::BLACK,
];
// Build a sequence from an iterator over a Tweenable (here, a
// Tracks<Transform>)
    let seq = Sequence::from_single(Tracks::new([
        Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            UiBackgroundColorLens {
                start: Color::NONE,
                end:  Color::BLACK,
            },
        )
        .with_repeat_count(RepeatCount::Finite(2))
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat)       
        // Get an event after each segment
        .with_completed_event(0)

    ]));


    world
        .commands()
        .entity(entity)
        .insert(Animator::new(seq));
}


impl Default for FadeOverlay {
    fn default() -> Self {
        FadeOverlay {
            // timer: Timer::from_seconds(FADE_DURATION_SECS, TimerMode::Once),
            // direction: FadeDirection::Out,
            target: Some(AppState::default()),
        }
    }
}


fn on_fade_to(    
    mut commands: Commands,
    mut added: EventReader<FadeTo>,
) {
    for added in added.read() {
        commands.spawn(FadeOverlay {
            target: Some(added.0.clone()),
            ..default()
        });
    }
}

fn loop_complete(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>, 
    
    mut fade: Query<&mut FadeOverlay>,
) {
  for ev in reader.read() {
    
    if let Ok(mut fade) = fade.get_mut(ev.entity) {         
        if let Some(target) = &mut fade.target.take() {
            commands.set_state(target.clone());             
        } else {
            commands.entity(ev.entity).despawn();
        }
    }
    
  }
}

// fn fade_system(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut events: EventReader<FadeTo>,
//     mut fade: Query<(Entity, &mut BackgroundColor, &mut FadeOverlay)>,
//     mut next_state: ResMut<NextState<AppState>>,
// ) {
//     let event = events.read().next();
//     let mut fade_query = fade.iter_mut().next();

//     let (e, mut bg, mut fade) = match (event, fade_query) {
//         (None, None) => return,
//         (None, Some(v)) => v,
//         (Some(to), None) => {
//             commands.spawn(FadeOverlay {
//                 target: to.0.clone(),
//                 ..default()
//             });
//             return;
//         }
//         (Some(to), Some((e, bg, mut fade))) => {
//             fade.target = to.0.clone();
//             fade.timer.reset();
//             fade.direction = FadeDirection::Out;
//             (e, bg, fade)
//         }
//     };

//     fade.timer.tick(time.delta());

//     let alpha = match fade.direction {
//         FadeDirection::In => 1.0 - fade.timer.fraction(),
//         FadeDirection::Out => fade.timer.fraction(),
//     }
//     .clamp(0.0, 1.0);

//     bg.0.set_alpha(alpha);

//     if fade.timer.just_finished() {
//         match fade.direction {
//             FadeDirection::Out => {
//                 bg.0.set_alpha(1.0);
//                 fade.timer.reset();
//                 fade.direction = FadeDirection::In;
//                 next_state.set(fade.target.clone());
//             }
//             FadeDirection::In => {
//                 commands.entity(e).despawn();
//             }
//         }
//     }
// }
