use bytemuck::{Pod, Zeroable};
use wgpu::*;
use crate::graphics::ModelInstance;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct PointLight {
    pub position: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    pub att_constant: f32,
    pub att_linear: f32,
    pub att_quadratic: f32,
    _pad0: u32,
    _pad1: u32
}

impl PointLight {

    /// Craetes a new `PointLight`
    pub fn new(
        position: [f32; 3],
        color: [f32; 3],
        att_constant: f32,
        att_linear: f32,
        att_quadratic: f32
    ) -> Self {
        Self {
            position,
            radius: 0.0,
            color,
            att_constant,
            att_linear,
            att_quadratic,
            _pad0: 0,
            _pad1: 0
        }
    }

    pub fn new_simple(
        position: [f32; 3],
        color: [f32; 3]
    ) -> Self {
        Self {
            position,
            radius: 0.0,
            color,
            att_constant: 0.0,
            att_linear: 0.0,
            att_quadratic: 1.0,
            _pad0: 0,
            _pad1: 0
        }
    }

    /// Computes the light's radius based on its intensity and light attenuation.
    /// See attenuation: https://learnopengl.com/Lighting/Light-casters
    pub fn compute_radius(&mut self, cutoff: f32) {
        let imax = self.color[0]
            .max(self.color[1])
            .max(self.color[2]);
        let a = self.att_quadratic;
        let b = self.att_linear;
        let c = self.att_constant - imax/cutoff;
        let det = b*b - 4.0*a*c;
        self.radius = (-b + det.sqrt()) / 2.0*a;
        log::debug!("Computed light radius of {}", self.radius);
    }

    /// The WGPU memory layout of a `PointLight` when stored in a vertex buffer, typically for instancing.
    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<PointLight>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                // Position
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 1
                },
                // Radius
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 2
                },
                // Color
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 3
                },
                // Attenuation Constant
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 7]>() as BufferAddress,
                    shader_location: 4
                },
                // Attenuation Linear
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 5
                },
                // Attenuation Quadratic
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 9]>() as BufferAddress,
                    shader_location: 6
                }
            ]
        }
    }
}
