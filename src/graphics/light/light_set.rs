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
            label: Some("Light Set Buffer"),
            size: (16 + lights_size*max_lights) as BufferAddress,
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
        self.buffer.slice(16..)
    }

    /// Slice buffer of usable lights.
    /// To be used as a uniform buffer.
    pub fn uniform_slice(&self) -> BufferSlice {
        self.buffer.slice(0..)
    }
}

impl LightSet<PointLight> {

    /// Computes each light's radius based on their intensity and light attenuation.
    /// See attenuation: https://learnopengl.com/Lighting/Light-casters
    pub fn compute_radiuses(&mut self, cutoff: f32) {
        for light in &mut self.lights {
            light.compute_radius(cutoff);
        }
    }
}