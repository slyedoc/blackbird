pub mod app;
pub mod auth;
pub mod components;
pub mod error_template;
pub mod errors;
pub mod pages;
pub mod todos;

pub mod prelude {
    pub use leptos::prelude::*;
    pub use leptos::Params;
    pub use leptos_router::params::Params;
    pub use leptos_router::hooks::{use_navigate, use_params, use_query};
    pub use leptos_meta::*;
    pub use icondata as i; // list at https://carloskiki.github.io/icondata/

    pub use leptos_bevy_canvas::prelude::*;

    pub use crate::{app::*, auth::*, components::*, pages::*, error_template::*, todos::*};
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Info);
    
    leptos::mount::hydrate_body(prelude::App);
}
    