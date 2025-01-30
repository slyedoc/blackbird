use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCameraController {
    pub zoom: f32,
    pub yaw: f32,
    pub pitch: f32,

    pub target_zoom: f32,    
    pub target_yaw: f32,
    pub target_pitch: f32,
}

impl PlayerCameraController {
    pub fn new(zoom: f32, yaw: f32, pitch: f32 ) -> Self {
        Self {
            zoom: zoom,
            yaw: yaw,
            pitch: pitch,

            target_zoom: zoom,
            target_yaw: yaw,
            target_pitch: pitch,
        }
    }

    pub fn add_zoom(&mut self, zoom: f32) {
        self.zoom += zoom;
        self.zoom = self.zoom.clamp(0.1, 1.0);
    }

    pub fn add_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch += pitch;

        self.pitch = self.pitch.clamp(-89.0, 89.0);
        self.yaw = self.yaw % 360.0;

        
    }
}

