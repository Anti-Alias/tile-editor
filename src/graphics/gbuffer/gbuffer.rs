use wgpu::*;


/// Geometry buffer that stores a multitude of color targets and a depth_stencil target
pub struct GBuffer {
    position: TextureView,
    normal: TextureView,
    depth_stencil: TextureView,
    diffuse: Option<TextureView>,
    specular: Option<TextureView>,
    emissive: Option<TextureView>,
    format: GBufferFormat
}
impl GBuffer {
    pub const DIFFUSE_BUFFER_BIT: u64 = 1;
    pub const SPECULAR_BUFFER_BIT: u64 = 1 << 1;
    pub const EMISSIVE_BUFFER_BIT: u64 = 1 << 2;

    /// Creates a GBuffer where each texture is of the specified size and have the formats specified
    /// in `format`.
    pub fn new(device: &Device, width: u32, height: u32, format: GBufferFormat) -> Self {
        let position = device
            .create_texture(&Self::descriptor_of(width, height, format.position))
            .create_view(&TextureViewDescriptor::default());
        let normal = device
            .create_texture(&Self::descriptor_of(width, height, format.normal))
            .create_view(&TextureViewDescriptor::default());
        let depth_stencil = device
            .create_texture(&Self::descriptor_of(width, height, format.depth_stencil))
            .create_view(&TextureViewDescriptor::default());
        let diffuse = format.diffuse.map(|tex_form| {
            device
                .create_texture(&Self::descriptor_of(width, height, tex_form))
                .create_view(&TextureViewDescriptor::default())
        });
        let specular = format.specular.map(|tex_form| {
            device
                .create_texture(&Self::descriptor_of(width, height, tex_form))
                .create_view(&TextureViewDescriptor::default())
        });
        let emissive = format.emissive.map(|tex_form| {
            device
                .create_texture(&Self::descriptor_of(width, height, tex_form))
                .create_view(&TextureViewDescriptor::default())
        });
        log::info!("Created GBuffer with format {:?}", format);
        Self {
            position,
            normal,
            depth_stencil,
            diffuse,
            specular,
            emissive,
            format
        }
    }

    /// Creates a simple frame buffer where all of the color buffers are of format `TextureFormat::Rgba32Float`
    /// and the depth stencil format is `TextureFormat::Depth32Float`
    pub fn create_simple(device: &Device, width: u32, height: u32, flags: u64) -> GBuffer {
        let position_format = TextureFormat::Rgba32Float;
        let normal_format = TextureFormat::Rgba32Float;
        let depth_stencil_format = TextureFormat::Depth32Float;
        let mut builder = GBufferFormatBuilder::new(
            position_format,
            normal_format,
            depth_stencil_format
        );
        if flags | Self::DIFFUSE_BUFFER_BIT != 0 {
            builder = builder.diffuse_format(TextureFormat::Rgba32Float);
        }
        if flags | Self::SPECULAR_BUFFER_BIT != 0 {
            builder = builder.specular_format(TextureFormat::Rgba32Float);
        }
        if flags | Self::EMISSIVE_BUFFER_BIT != 0 {
            builder = builder.emissive_format(TextureFormat::Rgba32Float);
        }
        let format = builder.build();
        Self::new(device, width, height, format)
    }
    pub fn position(&self) -> &TextureView { &self.position }
    pub fn normal(&self) -> &TextureView { &self.normal }
    pub fn depth_stecil(&self) -> &TextureView { &self.depth_stencil }
    pub fn diffuse(&self) -> Option<&TextureView> { self.diffuse.as_ref() }
    pub fn specular(&self) -> Option<&TextureView> { self.specular.as_ref() }
    pub fn emissive(&self) -> Option<&TextureView> { self.emissive.as_ref() }
    pub fn format(&self) -> &GBufferFormat { &self.format }

    fn optional_format(flags: u64, bit: u64, format: TextureFormat) -> Option<TextureFormat> {
        if flags | bit != 0 { Some(format) } else { None }
    }
    fn descriptor_of<'a>(width: u32, height: u32, format: TextureFormat) -> TextureDescriptor<'a> {
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
pub struct GBufferFormatBuilder { format: GBufferFormat }
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
        self.format.emissive = Some(emissive_format);
        self.format.flags |= GBuffer::EMISSIVE_BUFFER_BIT;
        self
    }

    /// Builds final format
    pub fn build(self) -> GBufferFormat {
        self.format
    }
}