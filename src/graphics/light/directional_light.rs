use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct DirectionalLight {
    pub direction: [f32; 3],
    _pad0: u32,
    pub color: [f32; 3],
    _pad1: u32
}

impl DirectionalLight {
    pub fn new(direction: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            direction,
            _pad0: 0,
            color,
            _pad1: 0
        }
    }
}