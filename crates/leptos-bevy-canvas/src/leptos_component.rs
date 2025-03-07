use bevy::prelude::*;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::{
    events::{event_l2b, LeptosChannelEventSender},
    prelude::LeptosBevyApp,
};
#[cfg(target_arch = "wasm32")]
use bevy::window::WindowCloseRequested;

#[derive(Event, Debug)]
pub struct StopSignal;

/// Embeds a Bevy app in a Leptos component. It will add an HTML canvas element and start
/// running the Bevy app inside it.
#[component]
pub fn BevyCanvas(    
    /// This function is be called to initialize and return the Bevy app.
    #[allow(unused_variables)]
    init: impl FnOnce() -> App + 'static,
    /// Optional canvas id. Defaults to `bevy_canvas`.
    #[prop(into, default = "bevy_canvas".to_string())]
    canvas_id: String,
) -> impl IntoView {
    // We dont want bevy running during server side rendering
    // https://book.leptos.dev/ssr/24_hydration_bugs.html
    // TODO: tried this with Effect but it didn't work
    #[cfg(target_arch = "wasm32")]
    {
        let (stop_l, stop_b) = event_l2b::<StopSignal>();

        request_animation_frame(move || {
            let mut app = init();
            app.import_event_from_leptos(stop_b)
                .add_systems(Update, handle_stop_signal.run_if(on_event::<StopSignal>))
                .run();
        });

        Owner::on_cleanup(move || {
            info!("Cleaning up BevyCanvas");
            stop_l.send(StopSignal).ok();
        });
    }

    view! { <canvas class="flex-grow" id=canvas_id></canvas> }
}

#[cfg(target_arch = "wasm32")]
fn handle_stop_signal(
    window_entities: Query<(Entity, &Window)>,
    mut event_writer: EventWriter<WindowCloseRequested>,
    mut app_exit: EventWriter<AppExit>,
) {
    for (entity, _window) in window_entities.iter() {
        info!("closing window entity: {:x}", entity.to_bits());
        event_writer.send(WindowCloseRequested { window: entity });
    }
    app_exit.send(AppExit::Success);
}
