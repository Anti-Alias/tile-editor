use wgpu::{Device, Queue};
use crate::graphics::light::{LightSet, PointLight, DirectionalLight};

/// Represents a bundle of lights of various types
pub struct LightBundle {
    pub point_lights: LightSet<PointLight>,
    pub directional_lights: LightSet<DirectionalLight>,
}

impl LightBundle {
    pub fn new(device: &Device, max_point_lights: u32, max_directional_lights: u32) -> Self {
        Self {
            point_lights: LightSet::new(device, max_point_lights),
            directional_lights: LightSet::new(device, max_directional_lights)
        }
    }

    pub fn point_lights(&mut self) -> &mut LightSet<PointLight> {
        &mut self.point_lights
    }

    pub fn directional_lights(&mut self) -> &mut LightSet<DirectionalLight> {
        &mut self.directional_lights
    }

    pub fn flush(&self, queue: &Queue) {
        self.point_lights.flush(queue);
        self.directional_lights.flush(queue);
    }
}