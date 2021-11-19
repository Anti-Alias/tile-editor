use wgpu::*;


/// Geometry buffer that stores a multitude of color targets and a depth_stencil target
pub struct GBuffer {
    position: TextureView,
    normal: TextureView,
    depth_stencil: Option<TextureView>,
    diffuse: Option<TextureView>,
    specular: Option<TextureView>,
    emissive: Option<TextureView>,
    format: GBufferFormat,
    view_count: u32                     // Number of views. Useful for pre-allocating color attachment vector
}
impl GBuffer {
    pub const DEPTH_STENCIL_BUFFER_BIT: u64 = 1;
    pub const DIFFUSE_BUFFER_BIT: u64 = 1 << 1;
    pub const SPECULAR_BUFFER_BIT: u64 = 1 << 2;
    pub const EMISSIVE_BUFFER_BIT: u64 = 1 << 3;

    /// Creates a GBuffer where each texture is of the specified size and have the formats specified
    /// in `format`.
    pub fn new(device: &Device, width: u32, height: u32, format: GBufferFormat) -> Self {
        let position = device
            .create_texture(&Self::descriptor_of(width, height, format.position))
            .create_view(&TextureViewDescriptor::default());
        let normal = device
            .create_texture(&Self::descriptor_of(width, height, format.normal))
            .create_view(&TextureViewDescriptor::default());
        let depth_stencil = format.depth_stencil.map(|tex_form| {
            device
                .create_texture(&Self::descriptor_of(width, height, tex_form))
                .create_view(&TextureViewDescriptor::default())
        });
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
        let mut view_count = 2;
        if diffuse.is_some() { view_count += 1 };
        if specular.is_some() { view_count += 1 };
        if emissive.is_some() { view_count += 1 };
        log::info!("Created GBuffer with format {:?}", format);
        Self {
            position,
            normal,
            depth_stencil,
            diffuse,
            specular,
            emissive,
            format,
            view_count
        }
    }

    /// Creates a simple frame buffer where all of the color buffers are of format `TextureFormat::Rgba32Float`
    /// and the depth stencil format is `TextureFormat::Depth32Float`
    pub fn create_simple(device: &Device, width: u32, height: u32, flags: u64) -> GBuffer {
        let position_format = TextureFormat::Rgba32Float;
        let normal_format = TextureFormat::Rgba32Float;
        let mut builder = GBufferFormatBuilder::new(
            position_format,
            normal_format
        );
        if flags | Self::DEPTH_STENCIL_BUFFER_BIT != 0 {
            builder = builder.diffuse_format(TextureFormat::Depth32Float);
        }
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
    pub fn position_view(&self) -> &TextureView { &self.position }
    pub fn normal_view(&self) -> &TextureView { &self.normal }
    pub fn depth_stencil_view(&self) -> Option<&TextureView> { self.depth_stencil.as_ref() }
    pub fn diffuse_view(&self) -> Option<&TextureView> { self.diffuse.as_ref() }
    pub fn specular_view(&self) -> Option<&TextureView> { self.specular.as_ref() }
    pub fn emissive_view(&self) -> Option<&TextureView> { self.emissive.as_ref() }
    pub fn format(&self) -> GBufferFormat { self.format }

    /// Both the color buffer and depth_stencil attachments
    pub fn attachments(&self) -> GBufferAttachments {

        // Creates color attachments
        let mut color_attachments = Vec::<RenderPassColorAttachment>::with_capacity(self.view_count as usize);
        let ops = Operations {
            load: LoadOp::Clear(wgpu::Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 }),
            store: true
        };
        color_attachments.push (
            RenderPassColorAttachment {
                view: &self.position,
                resolve_target: None,
                ops
            }
        );
        color_attachments.push(
            RenderPassColorAttachment {
                view: &self.normal,
                resolve_target: None,
                ops
            }
        );
        if let Some(view) = &self.diffuse {
            color_attachments.push(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops
            });
        }
        if let Some(view) = &self.specular {
            color_attachments.push(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops
            });
        }
        if let Some(view) = &self.emissive {
            color_attachments.push(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops
            });
        }

        // Depth/stencil attachment
        let depth_stencil_attachment = self.depth_stencil.as_ref().map(|view| {
            RenderPassDepthStencilAttachment {
                view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: true
                }),
                stencil_ops: None
            }
        });

        // Bundles attachments
        GBufferAttachments {
            color_attachments,
            depth_stencil_attachment
        }
    }

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
    depth_stencil: Option<TextureFormat>,
    diffuse: Option<TextureFormat>,
    specular: Option<TextureFormat>,
    emissive: Option<TextureFormat>,
    flags: u64
}

impl GBufferFormat {
    pub fn position(&self) -> TextureFormat { self.position }
    pub fn normal(&self) -> TextureFormat { self.normal }
    pub fn depth_stencil(&self) -> Option<TextureFormat> { self.depth_stencil }
    pub fn diffuse(&self) -> Option<TextureFormat> { self.diffuse }
    pub fn specular(&self) -> Option<TextureFormat> { self.specular }
    pub fn emissive(&self) -> Option<TextureFormat> { self.emissive }
    pub fn flags(&self) -> u64 { self.flags }
}

/// Builds a GBufferFormat
pub struct GBufferFormatBuilder { format: GBufferFormat }
impl GBufferFormatBuilder {
    pub fn new(
        position_format: TextureFormat,
        normal_format: TextureFormat
    ) -> GBufferFormatBuilder {
        GBufferFormatBuilder {
            format: GBufferFormat {
                position: position_format,
                normal: normal_format,
                depth_stencil: None,
                diffuse: None,
                specular: None,
                emissive: None,
                flags: 0
            }
        }
    }

    pub fn depth_stencil_format(mut self, depth_stencil_format: TextureFormat) -> Self {
        self.format.depth_stencil = Some(depth_stencil_format);
        self.format.flags |= GBuffer::DEPTH_STENCIL_BUFFER_BIT;
        self
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

/// Represents color attachments and depth-stencil attachments of a `GBuffer`.
pub struct GBufferAttachments<'a> {
    color_attachments: Vec<RenderPassColorAttachment<'a>>,
    depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>
}

impl<'a> GBufferAttachments<'a> {
    pub fn color_attachments(&self) -> &[RenderPassColorAttachment<'a>] {
        self.color_attachments.as_slice()
    }

    pub fn depth_stencil_attachment(&self) -> Option<RenderPassDepthStencilAttachment<'a>> {
        self.depth_stencil_attachment.clone()
    }
}