use bytemuck::{Pod, Zeroable};
use wgpu::{Device, Queue, Buffer, BufferDescriptor, BufferAddress, BufferUsages};

pub struct LightSet<L: Pod + Zeroable> {
    lights: Vec<L>,
    buffer: Buffer
}

impl<L: Pod + Zeroable> LightSet<L> {
    pub fn new(device: &Device, max_lights: u32) -> Self {
        let lights: Vec<L> = Vec::with_capacity(max_lights as usize);
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: (lights.len() * std::mem::size_of::<L>()) as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        Self {
            lights,
            buffer
        }
    }

    pub fn flush(&self, queue: &Queue) {
        let len_slice = [self.lights.len()];
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&len_slice));
        queue.write_buffer(&self.buffer, 4, bytemuck::cast_slice(&self.lights));
    }
}