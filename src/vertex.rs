use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3]
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}