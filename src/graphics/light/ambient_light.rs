use wgpu::*;
use bytemuck::{Pod, Zeroable};

/// Represents an ambient light that affects all objects uniformly
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct AmbientLight {
    pub color: [f32; 3],
    pad: u32
}

impl AmbientLight {

    /// Constructs a new `AmbientLight`
    pub fn new(color: [f32; 3]) -> Self {
        Self {
            color,
            pad: 0
        }
    }
}