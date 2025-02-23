use std::fmt::Display;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_meta::*;
use strum::{ EnumIter, IntoEnumIterator};

#[derive(Debug, Default, EnumIter, Clone, Copy)]
pub enum Game {
    #[default]
    Breakout,
    TicTacToe,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Game::Breakout => "Breakout",
            Game::TicTacToe => "Tic Tac Toe",
        };
        write!(f, "{}", s)
    }
}

impl Game {
    pub fn path(self) -> &'static str {
        match self {
            Game::Breakout => "/breakout",
            Game::TicTacToe => "/tictactoe",
        }        
    }
}

#[component]
pub fn Games() -> impl IntoView {
    view! {
        <h1>"Games"</h1>

        <main>
            {
                Game::iter().map(|game| {
                    let path = game.path();
                    view! {                        
                        <div>
                            <A href=path >
                                { format!("{:?}", game)}
                            </A>
                        </div>
                    }
                }).collect_view()
            }
            
        </main>
    }
}


#[component]
pub fn PlayGame(game: Game) -> impl IntoView {
    view! {
        <h1>{ format!("{:?}", game) }</h1>
    }
}