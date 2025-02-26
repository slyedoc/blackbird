use super::eventqueue::events::{
    ClientInEvents, EventProcessor, PluginOutEvents,
};
use super::eventqueue::plugin::DuplexEventsPlugin;
use super::state::{Shared, SharedResource, SharedState};
use bevy::prelude::*;

/// Represents the 3D Scene
#[derive(Clone)]
pub struct Scene {
    setup: fn() -> App,
    is_setup: bool,
    #[allow(dead_code)]
    canvas_id: String,
    evt_plugin: DuplexEventsPlugin,
    shared_state: Shared<SharedState>,
    processor: EventProcessor<ClientInEvents, PluginOutEvents>,
}


impl Scene {
    /// Create a new instance
    pub fn new(canvas_id: String, setup: fn() -> App) -> Scene {
        let plugin = DuplexEventsPlugin::new();
        Scene {
            setup,
            is_setup: false,
            canvas_id,
            evt_plugin: plugin.clone(),
            shared_state: SharedState::new(),
            processor: plugin.get_processor(),
        }
    }

    /// Get the shared state
    pub fn get_state(&self) -> Shared<SharedState> {
        self.shared_state.clone()
    }

    /// Get the event processor
    pub fn get_processor(
        &self,
    ) -> EventProcessor<ClientInEvents, PluginOutEvents> {
        self.processor.clone()
    }

    /// Setup and attach the bevy instance to the html canvas element
    pub fn setup(&mut self, ) {
        if self.is_setup {
            return;
        };

        let _app = (self.setup)()            
            .add_plugins(self.evt_plugin.clone())
            .insert_resource(SharedResource(self.shared_state.clone()))            
            .run();

        self.is_setup = true;
    }
}

