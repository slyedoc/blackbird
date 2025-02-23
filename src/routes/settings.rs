use leptos::prelude::*;
use crate::auth::*;
use super::Logout;

#[component]
pub fn Settings(action: ServerAction<Logout>) -> impl IntoView {
    view! {
        <h1>"Settings"</h1>
        <Logout action=action />
    }
}