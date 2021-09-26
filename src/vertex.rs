use bytemuck::{Pod, Zeroable};
use wgpu::{VertexBufferLayout, BufferAddress, VertexStepMode, VertexAttribute, VertexFormat};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub color: [f32; 3]
}

impl ModelVertex {
    pub const BUFFER_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: std::mem::size_of::<ModelVertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            // Position
            VertexAttribute{
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float32x3
            },
            // Color
            VertexAttribute {
                offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                shader_location: 1,
                format: VertexFormat::Float32x3
            }
        ]
    };
}
