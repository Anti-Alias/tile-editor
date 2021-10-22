use bytemuck::{Pod, Zeroable};
use wgpu::{VertexBufferLayout, BufferAddress, VertexStepMode, VertexAttribute, VertexFormat};

/// Describes a vertex layout
pub trait Vertex {
    fn layout<'a>() -> VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub color: RGBA,
    pub uv: Vector2
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
                // Color (r, g, b, a)
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x4
                },
                // UV (u, v)
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 10]>() as BufferAddress,
                    shader_location: 3,
                    format: VertexFormat::Float32x2
                }
            ]
        }
    }
}
