use bytemuck::{Pod, Zeroable};
use wgpu::*;
use crate::graphics::ModelInstance;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct PointLight {
    pub position: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    _pad: u32
}

impl PointLight {

    /// Craetes a new `PointLight`
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            radius: 0.0,
            color,
            _pad: 0
        }
    }

    /// Computes the light's radius based on its intensity and light attenuation.
    /// See attenuation: https://learnopengl.com/Lighting/Light-casters
    /// * `cutoff` - Cutoff attenuation value where light contribution is considered negligible. This value is usually close to 0, ie: 5.0/256.0
    /// * `constant` - Constant value from the equation. This value is usually 1.0.
    /// * `linear` - Linear value from the equation. Example from learnopengl.com uses 0.7
    /// * `quadratic` - Quadratic value from the equation. Example from learnopengl.com uses 1.8
    pub fn compute_radius(&mut self, cutoff: f32, constant: f32, linear: f32, quadratic: f32) {
        let imax = self.color[0]
            .max(self.color[1])
            .max(self.color[2]);
        let a = quadratic;
        let b = linear;
        let c = constant - imax/cutoff;
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
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 1
                },
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 2
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 3
                }
            ]
        }
    }
}
