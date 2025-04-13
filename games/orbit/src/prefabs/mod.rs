pub mod solar_system;
pub mod ships;

use bevy::prelude::*;
use ships::Ship;

pub struct PrefabPlugin;
impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(solar_system::SolarSystemPlugin)
        .register_type::<Ship>();    
    }
}