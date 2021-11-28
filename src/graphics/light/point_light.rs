use bytemuck::{Pod, Zeroable};
use wgpu::*;
use crate::graphics::ModelInstance;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct PointLight {
    /// Position of the light
    pub position: [f32; 3],

    /// Radius of the light
    pub radius: f32,

    /// Color of the lights
    pub color: [f32; 3],

    /// Attenuation coefficients (constant, linear, quadratic respectively)
    pub coefficients: [f32; 3],

    // Padding for uniform buffer
    _pad0: u32,
    _pad1: u32
}

impl PointLight {

    /// Craetes a new `PointLight`
    pub fn new(
        position: [f32; 3],
        color: [f32; 3],
        coefficients: [f32; 3]
    ) -> Self {
        Self {
            position,
            radius: 0.0,
            color,
            coefficients,
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
            coefficients: [0.0, 0.0, 1.0],
            _pad0: 0,
            _pad1: 0
        }
    }

    pub fn constant(&self) -> f32 {
        self.coefficients[0]
    }

    pub fn linear(&self) -> f32 {
        self.coefficients[1]
    }

    pub fn quadratic(&self) -> f32 {
        self.coefficients[2]
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, quadratic: f32) {
        self.coefficients[0] = constant;
        self.coefficients[1] = linear;
        self.coefficients[2] = quadratic;
    }

    /// Computes the light's radius based on its intensity and light attenuation.
    /// See attenuation: https://learnopengl.com/Lighting/Light-casters
    pub fn compute_radius(&mut self, cutoff: f32) {
        let imax = self.color[0]
            .max(self.color[1])
            .max(self.color[2]);
        let a = self.quadratic();
        let b = self.linear();
        let c = self.constant() - imax/cutoff;
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
                // Attenuation Constant, Linear, Quadratic
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 7]>() as BufferAddress,
                    shader_location: 4
                }
            ]
        }
    }
}
