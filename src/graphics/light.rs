use cgmath::{Point3, Vector3};
use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Queue};
use crate::graphics::Color;
use std::mem::size_of;
use crevice::std140::AsStd140;

/// Represents a simple point light that does not cast shadows
#[derive(Copy, Clone, Debug, AsStd140)]
pub struct PointLight {
    /// Position of the light in 3D space
    pub position: Point3<f32>,
    /// Color/intensity of the light
    pub color: Color,
}

/// Represents a simple directional light that does not cast shadows
#[derive(Copy, Clone, Debug, AsStd140)]
pub struct DirectionalLight {
    /// Direction of the rays being emitted from the light
    pub direction: Vector3<f32>,
    /// Color/intensity of the light
    pub color: Color,
}

/// Represents a set of lights of a given type
pub struct LightSet<L> {
    lights: Vec<L>,
    buffer: Buffer
}

impl<L> LightSet<L> {

    /// Creates a new `LightSet`.
    /// * `device` - Device used to create WGPU buffer. Buffer's capacity will match that of `lights` and is not growable.
    /// * `lights` - Client-side lights that should be flushed when changed.
    pub fn new(device: &Device, lights: Vec<L>) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<L>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        Self {
            lights,
            buffer
        }
    }

    /// Writes lights's state to it's WGPU buffer.
    /// Only need to be invoked on the light's state changing.
    pub fn flush(&self, queue: &Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.lights));
    }
}

/// Represents a bundle of lights
pub struct LightBundle {
    pub directional_lights: LightSet<DirectionalLight>,
    pub point_lights: LightSet<PointLight>
}

impl LightBundle {

    /// Determines if directional light(s) will be used
    pub const DIRECTIONAL_LIGHT_BIT: u64 = 1;

    /// Determines if point light(s) will be used
    pub const POINT_LIGHT_BIT: u64 = 1 << 1;

    /// Bit pattern where each bit determines the presence of an array of light types in the bundle.
    /// Bit order starting from LSB: DIRECTIONAL_LIGHTS, POINT_LIGHTS
    /// IE:
    ///     ...001 = DIRECTIONAL_LIGHTS
    ///     ...010 = POINT_LIGHTS
    ///     ...011 = DIRECTIONAL_LIGHTS + POINT_LIGHTS
    ///     ...etc
    fn flags() -> u64 {
        todo!()
    }
}

/// Represents a builder of a `LightBundle`.
struct LightBundleBuilder {
    directional_lights: Vec<DirectionalLight>,
    point_lights: Vec<PointLight>
}

impl LightBundleBuilder {

    pub fn new() -> Self {
        Self {
            directional_lights: Vec::new(),
            point_lights: Vec::new()
        }
    }

    pub fn directional_light(mut self, light: DirectionalLight) -> Self {
        self.directional_lights.push(light);
        self
    }

    pub fn point_light(mut self, light: PointLight) -> Self {
        self.point_lights.push(light);
        self
    }

    pub fn build(self) -> LightBundle {
        todo!()
    }
}