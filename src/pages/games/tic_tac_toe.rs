use leptos::prelude::Set;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_use::use_debounce_fn;

use crate::components::*;

#[component]
pub fn TicTacToe() -> impl IntoView {
    view! {
        <div class="flex w-full mx-auto max-w-[1400px] p-5 items-center">
            <Frame class="border-red-500 bg-red-500/5 flex-1">
                <h2 class="text-xl font-bold text-red-500 relative top-[-10px]">Bevy</h2>
                <div
                    class="aspect-[6/5] rounded-lg overflow-hidden"
                    style:max-width=format!("{}px", tic_tac_toe::RENDER_WIDTH)
                    style:max-height=format!("{}px", tic_tac_toe::RENDER_HEIGHT)
                >
                    <BevyCanvas
                        init=move || { tic_tac_toe::init_bevy_app() }
                        {..}
                        width=tic_tac_toe::RENDER_WIDTH
                        height=tic_tac_toe::RENDER_HEIGHT
                    />
                </div>
            </Frame>
        </div>
    }
}
