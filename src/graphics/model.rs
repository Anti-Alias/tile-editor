

use wgpu::*;
use crate::graphics::*;

/// Represents a set of meshes associated with materials
/// Meshes and materials can only be rendered if their indices are placed in the associations vector
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub associations: Vec<(usize, usize)>
}

impl Model {
    pub fn iter(&self) -> impl Iterator<Item=(&Mesh, &Material)> {
        self.associations.iter().map(move |association| {
            let mesh_idx = association.0;
            let mat_idx = association.1;
            (&self.meshes[mesh_idx], &self.materials[mat_idx])
        })
    }
}

/// Represents a set of render
pub struct GBuffer<'a> {
    position: &'a TextureView,
    normal: &'a TextureView,
    diffuse: Option<&'a TextureView>,
    specular: Option<&'a TextureView>,
    flags: u64
}

impl<'a> GBuffer<'a> {
    /// Determines if diffuse gbuffer will be used
    pub const DIFFUSE_BUFFER_BIT: u64 = 1;

    /// Determines if specular gbuffer will be used
    pub const SPECULAR_BUFFER_BIT: u64 = 1 << 1;
}

pub struct GBufferBuilder<'a> {
    position: &'a TextureView,
    normal: &'a TextureView,
    diffuse: Option<&'a TextureView>,
    specular: Option<&'a TextureView>,
    flags: u64
}

impl<'a> GBufferBuilder<'a> {

    /// Creates a new GBufferBuilder with a required position and normal texture view
    pub fn new(position: &'a TextureView, normal: &'a TextureView) -> Self {
        Self {
            position,
            normal,
            diffuse: None,
            specular: None,
            flags: 0
        }
    }

    /// Adds diffuse texture view
    pub fn diffuse(mut self, diffuse: &'a TextureView) -> Self {
        self.diffuse = Some(diffuse);
        self.flags |= GBuffer::DIFFUSE_BUFFER_BIT;
        self
    }

    /// Adds specular texture view
    pub fn specular(mut self, specular: &'a TextureView) -> Self {
        self.specular = Some(specular);
        self.flags |= GBuffer::SPECULAR_BUFFER_BIT;
        self
    }

    /// Builds resulting `GBuffer`
    pub fn build(self) -> GBuffer<'a> {
        GBuffer {
            position: self.position,
            normal: self.normal,
            diffuse: self.diffuse,
            specular: self.specular,
            flags: self.flags
        }
    }
}

/// Represents the
pub struct FrameBuffer<'a> {
    pub color: &'a TextureView,
    pub depth_stencil: &'a TextureView
}