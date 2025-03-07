use leptos::prelude::*;

#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("border-2 border-solid bg-white dark:bg-gray-800 {class} rounded-lg p-5")>{children()}</div> }
}
