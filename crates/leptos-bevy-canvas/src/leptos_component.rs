use bevy::prelude::*;
use leptos::prelude::*;

#[allow(unused_imports)]
use crate::events::{event_l2b, LeptosChannelEventSender};

// redo this once i work out the bugs

#[derive(Event, Debug)]
pub struct Stop;

/// Embeds a Bevy app in a Leptos component. It will add an HTML canvas element and start
/// running the Bevy app inside it.

/// Not using this currently, but wanted to leave a working example here
#[component]
pub fn BevyCanvas(
    /// This function is be called to initialize and return the Bevy app.
    #[allow(unused_variables)]
    init: impl Fn() -> App,
    /// Optional canvas id. Defaults to `bevy_canvas`.
    #[prop(into, default = "bevy_canvas".to_string())]
    canvas_id: String,
) -> impl IntoView {    
    {
        let (stop_l, stop_b) = event_l2b::<Stop>();

        Effect::new(move |_| {
             let _stop = stop_b.clone();
             request_animation_frame(move || {                
                 //let mut app = init();
        //         app.import_event_from_leptos(stop)
        //             .add_systems(Update, handle_stop_signal.run_if(on_event::<Stop>))
                     //app.run();
             });
        });
        

        Owner::on_cleanup(move || {
             let _ = stop_l.send(Stop);
             info!("Cleaning up BevyCanvas");
             // errors on page refresh, but that's ok            
         });
    }

    view! { <canvas id=canvas_id></canvas> }
}

fn _handle_stop_signal(
    // window_entities: Query<(Entity, &Window)>,
    // mut event_writer: EventWriter<WindowCloseRequested>,
    mut app_exit: EventWriter<AppExit>,
) {
    //
    // for (entity, _window) in window_entities.iter() {
    //     info!("closing window entity: {:x}", entity.to_bits());
    //     event_writer.send(WindowCloseRequested { window: entity });
    // }
    app_exit.send(AppExit::Success);
}
