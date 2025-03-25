use bevy::prelude::*;



#[derive(Event, Reflect)]
pub struct DeleteEvent(pub Entity);