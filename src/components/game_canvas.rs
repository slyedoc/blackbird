use crate::prelude::*;

use bevy::prelude::*;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[derive(bevy::prelude::Event, Debug)]
pub struct StopSignal;

/// Embeds a Bevy app in a Leptos component. It will add an HTML canvas element and start
/// running the Bevy app inside it.
#[component]
pub fn GameCanvas(#[allow(unused_variables)] game: Game) -> impl IntoView {
    let canvas_id = "bevy_canvas".to_string();
    let (exit_leptos_tx, exit_bevy_rx) = event_l2b::<StopSignal>();

    Effect::new(move |_| {
        #[allow(unused_variables)]
        let bevy_rx = exit_bevy_rx.clone();
        request_animation_frame(move || {
            let game_app = game.init();
            if let Some(mut app) = game_app {
                app.import_event_from_leptos(bevy_rx)
                    .add_systems(Update, handle_stop_signal.run_if(on_event::<StopSignal>))
                    .run();
            }
        });
    });

    Owner::on_cleanup(move || {
        // on page refresh this will fail, but that's ok
        let _ = exit_leptos_tx.send(StopSignal);
        log::info!("Cleaning up BevyCanvas");
    });

    view! {
      <div class="w-full">
        <canvas class="bg-white dark:bg-black w-full" id=canvas_id></canvas>
      </div>
    }
}

#[allow(dead_code)]
fn handle_stop_signal(
    mut app_exit: EventWriter<AppExit>,
) {
    app_exit.send(AppExit::Success);
}
