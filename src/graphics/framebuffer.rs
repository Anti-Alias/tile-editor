use wgpu::*;
use crate::graphics::*;

/// Geometry buffer that stores a multitude of color targets and a depth_stencil target
pub struct GBuffer {
    position: TextureView,
    normal: TextureView,
    depth_stencil: TextureView,
    diffuse: Option<TextureView>,
    specular: Option<TextureView>,
    emissive: Option<TextureView>,
    format: GBufferFormat,
    flags: u64
}
impl GBuffer {
    pub const DEPTH_STENCIL_BUFFER_BIT: u64 = 1;
    pub const DIFFUSE_BUFFER_BIT: u64 = 1 << 1;
    pub const SPECULAR_BUFFER_BIT: u64 = 1 << 2;
    pub const EMISSIVE_BUFFER_BIT: u64 = 1 << 3;
    pub fn new(device: &Device, width: u32, height: u32, format: GBufferFormat) -> GBuffer {
        let position_view = device
            .create_texture(&Self::descriptor_of(width, height, format.position))
            .create_view(&TextureViewDescriptor::default());
        let normal_view
            = device
            .create_texture(&Self::descriptor_of(width, height, format.normal))
            .create_view(&TextureViewDescriptor::default());
        let depth_stencil = device
            .create_texture(&depth_stencil_descriptor)
            .create_view(&TextureViewDescriptor::default());
        let diffuse_view = format.diffuse.map(|tex_form| {
            device
                .create_texture(&Self::descriptor_of(width, height, tex_form))
                .create_view(&TextureViewDescriptor::default())
        });
        let specular_view = format.specular.map(|tex_form| {
            device
                .create_texture(&Self::descriptor_of(width, height, tex_form))
                .create_view(&TextureViewDescriptor::default())
        });
        let emissive_view = format.emissive.map(|tex_form| {
            device
                .create_texture(&Self::descriptor_of(width, height, tex_form))
                .create_view(&TextureViewDescriptor::default())
        });
        GBuffer {
            
        }
    }
    pub fn position(&self) -> &TextureView { &self.position }
    pub fn normal(&self) -> &TextureView { &self.normal }
    pub fn depth_stecil(&self) -> &TextureView { &self.depth_stencil }
    pub fn diffuse(&self) -> Option<&TextureView> { self.diffuse.as_ref() }
    pub fn specular(&self) -> Option<&TextureView> { self.specular.as_ref() }
    pub fn emissive(&self) -> Option<&TextureView> { self.emissive.as_ref() }
    pub fn flags(&self) -> u64 { self.flags }

    fn descriptor_of(width: u32, height: u32, format: TextureFormat) -> TextureDescriptor {
        TextureDescriptor {
            label: None,
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING
        }
    }

    fn create_texture_view()
}

/// Represents the format of a `GBuffer`
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct GBufferFormat {
    position: TextureFormat,
    normal: TextureFormat,
    depth_stencil: TextureFormat,
    diffuse: Option<TextureFormat>,
    specular: Option<TextureFormat>,
    emissive: Option<TextureFormat>,
    flags: u64
}

impl GBufferFormat {
    pub fn position(&self) -> TextureFormat { self.position }
    pub fn normal(&self) -> TextureFormat { self.normal }
    pub fn depth_stencil(&self) -> TextureFormat { self.depth_stencil }
    pub fn diffuse(&self) -> Option<TextureFormat> { self.diffuse }
    pub fn specular(&self) -> Option<TextureFormat> { self.specular }
    pub fn emissive(&self) -> Option<TextureFormat> { self.emissive }
}

/// Builds a GBufferFormat
struct GBufferFormatBuilder {
    format: GBufferFormat
}
impl GBufferFormatBuilder {
    pub fn new(
        position_format: TextureFormat,
        normal_format: TextureFormat,
        depth_stencil_format: TextureFormat
    ) -> GBufferFormatBuilder {
        GBufferFormatBuilder {
            format: GBufferFormat {
                position: position_format,
                normal: normal_format,
                depth_stencil: depth_stencil_format,
                diffuse: None,
                specular: None,
                emissive: None,
                flags: 0
            }
        }
    }

    pub fn diffuse_format(mut self, diffuse_format: TextureFormat) -> Self {
        self.format.diffuse = Some(diffuse_format);
        self.format.flags |= GBuffer::DIFFUSE_BUFFER_BIT;
        self
    }

    pub fn specular_format(mut self, specular_format: TextureFormat) -> Self {
        self.format.specular = Some(specular_format);
        self.format.flags |= GBuffer::SPECULAR_BUFFER_BIT;
        self
    }

    pub fn emissive_format(mut self, emissive_format: TextureFormat) -> Self {
        self.format.specular = Some(emissive_format);
        self.format.flags |= GBuffer::EMISSIVE_BUFFER_BIT;
        self
    }
}

/// Main frame buffer to be rendered to after the geometry pass.
/// Does not own texture views as they tend to change frequently each frame.
pub struct ScreenBuffer<'a> {
    pub color: &'a TextureView,
    pub depth_stencil: &'a TextureView
}