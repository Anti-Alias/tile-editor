use cgmath::{Point3, Vector3};
use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Queue};
use crate::graphics::Color;
use std::mem::size_of;

/// Represents a simple point light that does not cast shadows
pub struct PointLight {
    /// Position of the light in 3D space
    position: Point3<f32>,
    /// Color/intensity of the light
    color: Color,
    /// WGPU buffer that stores the light's contents
    buffer: Buffer
}

impl PointLight {
    pub fn new(device: &Device, position: Point3<f32>, color: Color) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Point Light Buffer"),
            size: (size_of::<Point3<f32>>() + size_of::<Color>()) as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        Self {
            position,
            color,
            buffer
        }
    }

    /// Writes light's state to it's WGPU buffer.
    /// Only need to be invoked on the light's state changing.
    pub fn flush(&self, queue: &Queue) {
        let position: [f32; 3] = self.position.into();
        let color: [f32; 4] = self.color.rgba();
        let mut floats: [f32; 8] = [0.0; 8];
        for i in 0..3 { floats[i] = position[i]; }
        for i in 4..8 { floats[i] = color[i]; }
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&floats));
    }
}

/// Represents a simple directional light that does not cast shadows
pub struct DirectionalLight {
    /// Direction of the rays being emitted from the light
    pub direction: Vector3<f32>,
    /// Color/intensity of the light
    pub color: Color,
    /// WGPU buffer that stores the light's contents
    pub buffer: Buffer
}