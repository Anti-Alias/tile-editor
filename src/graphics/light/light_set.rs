use bytemuck::{Pod, Zeroable};
use wgpu::*;
use crate::graphics::light::PointLight;

pub struct LightSet<L: Pod + Zeroable> {
    pub lights: Vec<L>,
    pub buffer: Buffer
}

impl<L: Pod + Zeroable> LightSet<L> {

    /// Creates a new set of lights
    /// * `device` Device used to allocate buffer
    /// * `max_lights` How mahy lights to allocate on the CPU/GPU. Number of lights cannot exceed this value after the fact.
    pub fn new(device: &Device, max_lights: u32) -> Self {
        let lights: Vec<L> = Vec::with_capacity(max_lights as usize);
        let lights_size = std::mem::size_of::<L>() as u32;
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: (lights_size * max_lights) as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        Self {
            lights,
            buffer
        }
    }

    /// Flushes any updates to the lights on the CPU to the GPU
    pub fn flush(&self, queue: &Queue) {
        let header = [self.lights.len(), 0, 0, 0];                                  // length, pad, pad, pad
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&header));         // bytes 0-16 (length)
        queue.write_buffer(&self.buffer, 16, bytemuck::cast_slice(&self.lights));   // bytes 16-? (light data)
    }

    /// Slice buffer of usable lights.
    /// To be used as a vertex instance buffer.
    pub fn instance_slice(&self) -> BufferSlice {
        let size = std::mem::size_of::<L>();
        let len = self.lights.len();
        let bytes = (size*len) as BufferAddress;
        self.buffer.slice(16..bytes)
    }

    /// Slice buffer of usable lights.
    /// To be used as a uniform buffer.
    pub fn uniform_slice(&self) -> BufferSlice {
        let size = std::mem::size_of::<L>();
        let len = self.lights.len();
        let bytes = (size*len) as BufferAddress;
        self.buffer.slice(0..bytes)
    }
}

impl LightSet<PointLight> {

    /// Computes each light's radius based on their intensity and light attenuation.
    /// See attenuation: https://learnopengl.com/Lighting/Light-casters
    /// * `cutoff` - Cutoff attenuation value where light contribution is considered negligible. This value is usually close to 0, ie: 5.0/256.0
    /// * `constant` - Constant value from the equation. This value is usually 1.0.
    /// * `linear` - Linear value from the equation. Example from learnopengl.com uses 0.7
    /// * `quadratic` - Quadratic value from the equation. Example from learnopengl.com uses 1.8
    pub fn compute_radius(&mut self, cutoff: f32, constant: f32, linear: f32, quadratic: f32) {
        for light in &mut self.lights {
            light.compute_radius(cutoff, constant, linear, quadratic);
        }
    }
}