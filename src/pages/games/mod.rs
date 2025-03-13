#[cfg(feature = "unidir_events")]
mod unidir_events;
#[cfg(feature = "unidir_events")]
pub use unidir_events::UnidirEvents;

#[cfg(feature = "sync_app")]
mod sync_app;
#[cfg(feature = "sync_app")]
pub use sync_app::*;

use std::fmt::Display;

use leptos::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::*;

#[derive(Debug, Default, EnumIter, Clone, Copy)]
pub enum Game {
    #[default]
    Breakout,
    TicTacToe,
    CastApp,

    UnidirEvents,
    SyncApp,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Game::Breakout => "Breakout",
            Game::TicTacToe => "Tic Tac Toe",
            Game::UnidirEvents => "Unidir Events",
            Game::SyncApp => "Sync App",
            Game::CastApp => "Cast App",
        };
        write!(f, "{}", s)
    }
}

impl Game {
    pub fn init(self) -> Option<bevy::app::App> {
        match self {
            #[cfg(all(feature = "breakout", feature = "hydrate"))]
            Game::Breakout => Some(breakout::init_bevy_app()),
            #[cfg(all(feature = "tic_tac_toe", feature = "hydrate"))]
            Game::TicTacToe => Some(tic_tac_toe::init_bevy_app()),
            #[cfg(all(feature = "cast_app", feature = "hydrate"))]
            Game::CastApp => Some(cast_app::init_bevy_app()),
            _ => {
                log::error!("Game feature not include or game not supported");
                None
            }
        }
    }

    pub fn path(self) -> &'static str {
        match self {
            Game::Breakout => "/breakout",
            Game::TicTacToe => "/tictactoe",
            Game::UnidirEvents => "/unidir_events",
            Game::SyncApp => "/sync_app",
            Game::CastApp => "/cast_app",
        }
    }

    pub fn image(self) -> &'static str {
        match self {
            Game::Breakout => "/img/breakout.png",
            Game::TicTacToe => "/img/tic_tac_toe.png",
            Game::UnidirEvents => "/img/breakout.png",
            Game::SyncApp => "/img/breakout.png",
            Game::CastApp => "/img/cast_app.png",
        }
    }
}

#[component]
pub fn Games() -> impl IntoView {
    view! {
      <h2 class="mt-10 text-center text-2xl/9 font-bold tracking-tight">"Games"</h2>
      <ul
        role="list"
        class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
      >
        {Game::iter()
          .map(|game| {
            view! {
              <li class="relative">
                <div class="group overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                  <a href=game.path() class="block">
                    <span class="block text-sm font-medium text-gray-900">
                      {format!("{}", game)}
                    </span>
                    <img src=game.image() />
                  </a>
                </div>
              </li>
            }
          })
          .collect_view()}
      </ul>
    }
}

#[derive(Params, PartialEq)]
struct GameParams {
    id: Option<String>,
}

#[component]
pub fn GameProfile() -> impl IntoView {
    let params = use_params::<GameParams>();
    //let query = use_query::<ContactSearch>();

    let name = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default()
    };

    view! {
      <div>
        <p>{name()}</p>
      </div>
    }
}

#[component]
pub fn NoGame() -> impl IntoView {
    view! { "No Games" }
}

