use bevy::prelude::{Name, Transform, With};
use sync_app::init_bevy_app;

use leptos::prelude::Set;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_use::use_debounce_fn;

use crate::components::*;

#[component]
pub fn SyncApp() -> impl IntoView {
    let (selected, selected_query_duplex) = single_query_signal::<(Name,), With<Transform>>();

    view! {
        <div class="flex w-full mx-auto max-w-[1400px] p-5 items-center">
            <Frame class="border-red-500 bg-red-500/5 flex-1">
                <h2 class="text-xl font-bold text-red-500 relative top-[-10px]">Bevy</h2>
                <div
                    class="aspect-[6/5] rounded-lg overflow-hidden"
                    style:max-width=format!("{}px", sync_app::RENDER_WIDTH)
                    style:max-height=format!("{}px", sync_app::RENDER_HEIGHT)
                >
                    <BevyCanvas
                        init=move || {  init_bevy_app(selected_query_duplex) }
                        {..}
                        width=sync_app::RENDER_WIDTH
                        height=sync_app::RENDER_HEIGHT
                    />
                </div>
            </Frame>

            <Frame class="border-blue-500 bg-blue-500/5 max-w-[200px]">
                <h2 class="text-xl font-bold text-blue-500 relative top-[-10px]">Leptos</h2>

                <input
                    type="text"
                    prop:value=move || selected.read().as_ref().map(|(name,)| name.to_string()).unwrap_or_default()
                    on:input=move |ev| {
                        selected.write().as_mut().map(|(name,)| name.set(event_target_value(&ev)));
                    }
                />
            </Frame>
        </div>
    }
}
