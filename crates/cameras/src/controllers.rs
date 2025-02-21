use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod fps;
pub use fps::*;

mod orbit;
pub use orbit::*;

mod unreal;

pub use unreal::*;


#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! define_on_controller_enabled_changed(($ControllerStruct:ty) => {
        fn on_controller_enabled_changed(
            mut smoothers: Query<(&mut Smoother, &crate::CameraController), Changed<$ControllerStruct>>,
        ) {
            for (mut smoother, controller) in smoothers.iter_mut() {
                smoother.set_enabled(controller.enabled);
            }
        }
    });
}

#[derive(Clone, Component, Copy, Debug, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default, Debug)]
pub struct CameraController {
    pub enabled: bool,
}

impl Default for CameraController {
    fn default() -> Self {
        Self { enabled: true }
    }
}


