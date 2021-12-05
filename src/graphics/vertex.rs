use bytemuck::{Pod, Zeroable};
use wgpu::{VertexBufferLayout, BufferAddress, VertexStepMode, VertexAttribute, VertexFormat};
use crevice::std140::AsStd140;

/// Describes a vertex layout
pub trait Vertex {
    fn layout<'a>() -> VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, AsStd140)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn rgb(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
    pub fn rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    pub fn argb(&self) -> [f32; 4] {
        [self.a, self.r, self.g, self.b]
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2]
}

impl Vertex for ModelVertex {
    fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                // Position (x, y, z)
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3
                },
                // Normal (x, y, z)
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3
                },
                // Tangent (x, y, z)
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x3
                },
                // Bitangent (x, y, z)
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 9]>() as BufferAddress,
                    shader_location: 3,
                    format: VertexFormat::Float32x3
                },
                // Color (r, g, b, a)
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 4,
                    format: VertexFormat::Float32x4
                },
                // UV (u, v)
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 16]>() as BufferAddress,
                    shader_location: 5,
                    format: VertexFormat::Float32x2
                }
            ]
        }
    }
}
