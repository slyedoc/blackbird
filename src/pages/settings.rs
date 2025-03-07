
use crate::auth::*;
use leptos::prelude::*;

#[component]
pub fn Settings(action: ServerAction<Logout>) -> impl IntoView {
    view! {
        <h1>"Settings"</h1>
    }
}
