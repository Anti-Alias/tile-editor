use std::io::Write;
use cgmath::{Point3, Vector3};
use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Queue};
use bytemuck::{Pod, Zeroable};
use crate::graphics::{Color};

/// Represents a simple point light that does not cast shadows.
/// Padded to the std140 specification.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PointLight {
    pub position: [f32; 3], // 0..2
    _pad: u32,              // 3
    pub color: [f32; 4],    // 4..7
}

impl PointLight {
    pub fn new(position: Point3<f32>, color: Color) -> Self {
        Self {
            position: [position.x, position.y, position.z],
            _pad: 0,
            color: color.rgba()
        }
    }
}

/// Represents a simple directional light that does not cast shadows.
/// Padded to the std140 specification.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct DirectionalLight {
    pub direction: [f32; 3],    // 0..2
    pub _pad: u32,              // 3
    pub color: [f32; 4],        // 4..7
}

impl DirectionalLight {
    pub fn new(direction: Vector3<f32>, color: Color) -> Self {
        Self {
            direction: [direction.x, direction.y, direction.z],
            _pad: 0,
            color: color.rgba()
        }
    }
}

/// Represents a set of lights of a given type
pub struct LightSet<L: Pod + Zeroable> {
    lights: Vec<L>,
    buffer: Buffer
}

impl<L: Pod + Zeroable> LightSet<L> {
    /// Creates a new `LightSet`.
    /// * `device` - Device used to create WGPU buffer. Buffer's capacity will match that of `lights` and is not growable.
    /// * `lights` - Client-side lights that should be flushed when changed.
    pub fn new(device: &Device, lights: Vec<L>) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: (std::mem::size_of::<L>() * lights.capacity()) as BufferAddress,
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
    directional_lights: LightSet<DirectionalLight>,
    point_lights: LightSet<PointLight>,
    flags: u64
}

impl LightBundle {

    /// Determines if directional light(s) will be used
    pub const DIRECTIONAL_LIGHT_BIT: u64 = 1;

    /// Determines if point light(s) will be used
    pub const POINT_LIGHT_BIT: u64 = 1 << 1;

    /// All directional lights
    fn directional_lights(&self) -> &LightSet<DirectionalLight> {
        &self.directional_lights
    }

    /// All point lights
    fn point_lights(&self) -> &LightSet<PointLight> {
        &self.point_lights
    }

    /// Bit pattern where each bit determines the presence of an array of light types in the bundle.
    /// Bit order starting from LSB: DIRECTIONAL_LIGHTS, POINT_LIGHTS
    /// IE:
    ///     ...001 = DIRECTIONAL_LIGHTS
    ///     ...010 = POINT_LIGHTS
    ///     ...011 = DIRECTIONAL_LIGHTS + POINT_LIGHTS
    ///     ...etc
    fn flags(&self) -> u64 { self.flags }
}

/// Represents a builder of a `LightBundle`.
struct LightBundleBuilder {
    directional_lights: Vec<DirectionalLight>,
    point_lights: Vec<PointLight>,
    flags: u64
}

impl LightBundleBuilder {
    pub fn new() -> Self {
        Self {
            directional_lights: Vec::new(),
            point_lights: Vec::new(),
            flags: 0
        }
    }

    pub fn directional_light(mut self, light: DirectionalLight) -> Self {
        self.directional_lights.push(light);
        self.flags |= LightBundle::DIRECTIONAL_LIGHT_BIT;
        self
    }

    pub fn point_light(mut self, light: PointLight) -> Self {
        self.point_lights.push(light);
        self.flags |= LightBundle::POINT_LIGHT_BIT;
        self
    }

    pub fn build(self, device: &Device) -> LightBundle {
        LightBundle {
            directional_lights: LightSet::new(device, self.directional_lights),
            point_lights: LightSet::new(device, self.point_lights),
            flags: self.flags
        }
    }
}