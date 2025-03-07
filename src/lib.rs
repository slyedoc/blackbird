#![allow(warnings)]
pub mod app;
pub mod auth;
pub mod components;
pub mod error_template;
pub mod errors;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod state;
pub mod todos;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}


pub mod prelude {
    pub use leptos::prelude::*;
    pub use leptos::Params;
    pub use leptos_router::params::Params;
    pub use leptos_router::hooks::{use_params, use_query};
    pub use leptos_meta::*;
    pub use leptos_icons::Icon;
    pub use icondata as i; // list at https://carloskiki.github.io/icondata/

    pub use leptos_bevy_canvas::prelude::*;

    pub use crate::{auth::*, components::*, pages::*};

}