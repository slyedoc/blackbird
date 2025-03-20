mod sync_app;
use leptos_use::{use_element_size, UseElementSizeReturn};
pub use sync_app::*;

mod unidir_events;
pub use unidir_events::*;

use bevy::prelude::*;
use leptos::{html::Canvas, prelude::*, task::spawn_local};
use leptos_bevy_canvas::prelude::*;

use strum::{EnumIter, EnumProperty, IntoEnumIterator};

use crate::prelude::*;
use gloo_timers::future::*;

#[derive(Debug, Default, EnumIter, Clone, Copy, PartialEq, Eq, EnumProperty)]
pub enum Game {
    #[default]
    #[strum(props(img = "/img/mine.png", path = "mine"))]
    Mine,
    #[strum(props(img = "/img/breakout.png", path = "breakout"))]
    Breakout,
    #[strum(props(img = "/img/tic_tac_toe.png", path = "tic_tac_toe"))]
    TicTacToe,
    #[strum(props(img = "/img/cast_app.png", path = "cast-app"))]
    CastApp,
}

// #[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, EnumProperty)]
// pub enum UniqueGame {
//     #[strum(props(img = "/img/unidir_events.png", path = "unidir-events"))]
//     UnidirEvents,
//     #[strum(props(img = "/img/sync_app.png", path = "sync-app"))]
//     SyncApp,
// }

#[component]
pub fn Games() -> impl IntoView {
    view! {
      <div class="mx-auto flex w-full max-w-7xl items-start gap-x-8 px-4 py-10 sm:px-6 lg:px-8">
    <aside class="sticky top-8 hidden w-44 shrink-0 lg:block">
      "Left column area"
      <ul
      role="list"
      class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
    >
      {Game::iter()
        .map(|game| {
          view! {
            <li>
              <A
                href=game.get_str("path").unwrap()
                {..}
                class="block overflow-hidden rounded-lg bg-gray-100 aria-[current=page]:ring-2 aria-[current=page]:ring-indigo-500 aria-[current=page]:ring-offset-2 aria-[current=page]:ring-offset-gray-100 "
              >
                <span class="block text-sm font-medium text-gray-900">
                  {format!("{:?}", game)}
                </span>

                // <Show when={game == } fallback=move || view! { <p>"Loading..."</p> }>
                <span class="relative flex size-3">
                  <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-sky-400 opacity-75"></span>
                  <span class="relative inline-flex size-3 rounded-full bg-sky-500"></span>
                </span>
                // </Show>
                <div class="w-full h-full aspect-w-16 aspect-h-9">
                  <img src=game.get_str("img").unwrap() class="w-full h-full object-cover" />
                </div>
              </A>
            </li>
          }
        })
        .collect_view()}
    </ul>


    </aside>

    <main class="flex-1">
      "Main area"
      <Outlet />
    </main>

    <aside class="sticky top-8 hidden w-96 shrink-0 xl:block">
      "Right column area"
    </aside>
  </div>



    }
}

#[derive(Params, PartialEq)]
struct GameParams {
    id: Option<String>,
}

#[component]
pub fn GameProfile() -> impl IntoView {
    let params = use_params::<GameParams>();
    let cavas = NodeRef::<Canvas>::new();

    let UseElementSizeReturn { width, height } = use_element_size(cavas);

    let id = Signal::derive(move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.id.clone())
            .unwrap_or_default()
    });

    let game = Signal::derive(move || {
        let game_id = id.get();
        Game::iter()
            .filter_map(|g| {
                if let Some(p) = g.get_str("path") {
                    if p == game_id {
                        return Some(g);
                    }
                }
                None
            })
            .next()
            .unwrap_or_default()
    });

    // create new event channels
    let exit_tx_signal: RwSignal<Option<LeptosEventSender<StopSignal>>> = RwSignal::new(None);
      
    Effect::watch(
        move || game.get(),
        move |curr, prev, _| {
            //log::info!("Previous: {:?}, Current: {}", prev, curr);
            if prev.is_some() {                
                if let Some(tx) = exit_tx_signal.get_untracked() {
                    match tx.send(StopSignal) {
                        Err(_) => log::error!("StopSignal failed"),
                        _ => (),
                    };
                }                                
            }
            
            let g = curr.clone();
            spawn_local(async move {
                // TODO: find better, delaying here to allow the previous game to stop
                TimeoutFuture::new(100).await; // 100ms delay

                let (tx, bevy_rx) = event_l2b::<StopSignal>();
                exit_tx_signal.set(Some(tx));

                let app: Option<App> = match g {
                  #[cfg(all(feature = "breakout", feature = "hydrate"))]
                  Game::Breakout => Some(breakout::init_bevy_app()),
                  #[cfg(all(feature = "tic_tac_toe", feature = "hydrate"))]
                  Game::TicTacToe => Some(tic_tac_toe::init_bevy_app()),
                  #[cfg(all(feature = "cast_app", feature = "hydrate"))]
                  Game::CastApp => Some(cast_app::init_bevy_app()),
                  #[cfg(all(feature = "mine" , feature = "hydrate"))]
                  Game::Mine => Some(mine::init_bevy_app()),
                  #[allow(unreachable_patterns)]
                  game => {
                      log::error!("game feature '{:?}' wasn't enabled", game);
                      None
                  }
                };
      
                if let Some(mut app) = app {
                  app.import_event_from_leptos(bevy_rx)
                      .add_systems(Update, stop_bevy.run_if(on_event::<StopSignal>));
                      // delay so canvas exists
                      request_animation_frame(move || {
                          app.run();
                      });
                }
            });
        },
        true,
    );

    // stop any game on page exit
    Owner::on_cleanup(move || {
        if let Some(tx) = exit_tx_signal.get() {
            match tx.send(StopSignal) {
                Err(_) => log::info!("cleanup StopSignal failed"),
                _ => (),
            };
        }
        log::info!("TODO: Cleaning up BevyCanvas");
    });

    view! {
      <h2 class="text-center text-primary">{move || format!("{:?} {:?}", id.get(), game.get())}</h2>
      <div class="w-full">
        "Width: " {width} "Height: " {height}
        <canvas class="w-full bg-white dark:bg-black" node_ref=cavas id="bevy_canvas" />
      </div>
    }
}

#[component]
pub fn NoGame() -> impl IntoView {
    view! { <p>"Select a game."</p> }
}

#[derive(bevy::prelude::Event, Debug)]
pub struct StopSignal;

#[allow(dead_code)]
fn stop_bevy(mut app_exit: EventWriter<AppExit>) {
    log::info!("STOP BEVY");
    app_exit.send(AppExit::Success);
}
