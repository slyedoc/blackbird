mod tic_tac_toe;
pub use tic_tac_toe::TicTacToe;

mod unidir_events;
pub use unidir_events::UnidirEvents;

mod sync_app;
pub use sync_app::SyncApp;

use std::fmt::Display;

use leptos::prelude::*;
use leptos_router::components::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::*;

#[derive(Debug, Default, EnumIter, Clone, Copy)]
pub enum Game {
    #[default]
    Breakout,
    TicTacToe,
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
        };
        write!(f, "{}", s)
    }
}

impl Game {
    pub fn path(self) -> &'static str {
        match self {
            Game::Breakout => "/breakout",
            Game::TicTacToe => "/tictactoe",
            Game::UnidirEvents => "/unidir_events",
            Game::SyncApp => "/sync_app",
        }
    }

    pub fn image(self) -> &'static str {
        match self {
            Game::Breakout => "/img/breakout.png",
            Game::TicTacToe => "/img/tic_tac_toe.png",
            Game::TicTacToe => "/img/breakout.png",
            Game::UnidirEvents => "/img/breakout.png",
            Game::SyncApp => "/img/breakout.png",
        }
    }
}

#[component]
pub fn Games() -> impl IntoView {
    view! {
      <h1>"Games"</h1>
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

// <li class="col-span-1 divide-y divide-gray-200 rounded-lg bg-white shadow-sm">
//   <div class="flex w-full items-center justify-between space-x-6 p-6">
//     <div class="flex-1 truncate">
//       <div class="flex items-center space-x-3">
//         <h3 class="truncate text-sm font-medium text-gray-900">{name}</h3>
//         <A href=game.path()>"Play"</A>
//       // <span class="inline-flex shrink-0 items-center rounded-full bg-green-50 px-1.5 py-0.5 text-xs font-medium text-green-700 ring-1 ring-green-600/20 ring-inset">Admin</span>
//       </div>
//     // <p class="mt-1 truncate text-sm text-gray-500">Regional Paradigm Technician</p>
//     </div>
//     <img
//       class="size-10 shrink-0 rounded-full bg-gray-300"
//       src=game.image()
//       alt=""
//     />
//   </div>
// </li>
