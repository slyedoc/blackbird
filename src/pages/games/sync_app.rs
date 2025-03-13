use bevy::
    prelude::{*, With as With};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::components::*;

#[component]
pub fn SyncApp() -> impl IntoView {
    let (selected, selected_query_duplex) = single_query_signal::<(Name,), With<Transform>>();

    let (exit_leptos_tx, exit_bevy_rx) = event_l2b::<StopSignal>();

    Effect::new(move |_| {
        
        let selected = selected_query_duplex.clone();
        let bevy_rx = exit_bevy_rx.clone();
        request_animation_frame(move || {
            sync_app::init_bevy_app()
                .sync_leptos_signal_with_query(selected)
                .import_event_from_leptos(bevy_rx)
                .add_systems(Update, handle_stop_signal.run_if(on_event::<StopSignal>))
                .run();
        });
    });

    Owner::on_cleanup(move || {
        // on page refresh this will fail, but that's ok
        let _ = exit_leptos_tx.send(StopSignal);
        log::info!("Cleaning up BevyCanvas");
    });

    view! {
      <div class="flex items-center p-5 mx-auto w-full max-w-[1400px]">
        <Frame class="flex-1 border-red-500 bg-red-500/5">
          <h2 class="relative text-xl font-bold text-red-500 top-[-10px]">Bevy</h2>
          <div class="overflow-hidden rounded-lg aspect-[6/5]">
            // style:max-width=format!("{}px", sync_app::RENDER_WIDTH)
            // style:max-height=format!("{}px", sync_app::RENDER_HEIGHT)
            <div class="w-full">
              <canvas class="bg-white dark:bg-black w-full" id="bevy_canvas"></canvas>
            </div>
          </div>
        </Frame>

        <Frame class="border-blue-500 bg-blue-500/5 max-w-[200px]">
          <h2 class="relative text-xl font-bold text-blue-500 top-[-10px]">Leptos</h2>

          <input
            type="text"
            prop:value=move || {
              selected.read().as_ref().map(|(name,)| name.to_string()).unwrap_or_default()
            }
            on:input=move |ev| {
              selected.write().as_mut().map(|(name,)| name.set(event_target_value(&ev)));
            }
          />
        </Frame>
      </div>
    }
}

fn handle_stop_signal(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
