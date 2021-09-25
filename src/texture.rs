use wgpu::{Sampler, TextureView};
use std::rc::Rc;

/// Struct with a wgpu Texture handle, a TextureView handle and a SamplerS handle.
#[derive(Clone)]
pub struct Texture {
    pub texture: Rc<wgpu::Texture>,
    pub view: Rc<TextureView>,
    pub sampler: Rc<Sampler>,
}