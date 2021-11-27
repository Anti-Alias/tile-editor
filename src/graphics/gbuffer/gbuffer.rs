use wgpu::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};


/// Geometry buffer that stores a multitude of color targets and a depth_stencil target
pub struct GBuffer {
    position: TextureView,
    normal: TextureView,
    depth_stencil: Option<TextureView>,
    color: Option<TextureView>,
    format: GBufferFormat,
    sampler: Sampler,
    size_buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
    view_count: u32                     // Number of views. Useful for pre-allocating color attachment vector
}
impl GBuffer {

    /// Flag that determines if a depth/stencil buffer is enabled for a `GBuffer`
    pub const DEPTH_STENCIL_BUFFER_BIT: u64 = 1;

    /// Flag that determines if a color buffer encoding diffuse, specular and emissive data is enabled for a `GBuffer`
    pub const COLOR_BUFFER_BIT: u64 = 1 << 1;

    /// Creates a GBuffer where each texture is of the specified size and have the formats specified
    /// in `format`.
    pub fn new(device: &Device, width: u32, height: u32, format: GBufferFormat) -> Self {
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });
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
        let color = format.color.map(|tex_form| {
            device
                .create_texture(&Self::descriptor_of(width, height, tex_form))
                .create_view(&TextureViewDescriptor::default())
        });
        let size_slice = [width as f32, height as f32];
        let size_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&size_slice),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        let (bind_group, bind_group_layout) = Self::create_bind_group(
            device,
            &size_buffer,
            &sampler,
            &position,
            &normal,
            color.as_ref()
        );
        let mut view_count = 2;
        if color.is_some() { view_count += 1 };
        log::info!("Created GBuffer with format {:?}", format);
        Self {
            position,
            normal,
            depth_stencil,
            color,
            format,
            sampler,
            size_buffer,
            bind_group,
            bind_group_layout,
            view_count
        }
    }

    pub fn position_view(&self) -> &TextureView { &self.position }
    pub fn normal_view(&self) -> &TextureView { &self.normal }
    pub fn depth_stencil_view(&self) -> Option<&TextureView> { self.depth_stencil.as_ref() }
    pub fn color_view(&self) -> Option<&TextureView> { self.color.as_ref() }
    pub fn format(&self) -> GBufferFormat { self.format }
    pub fn bind_group(&self) -> &BindGroup { &self.bind_group }
    pub fn bind_group_layout(&self) -> &BindGroupLayout { &self.bind_group_layout }

    /// Both the color buffer and depth_stencil attachments
    pub fn attachments(&self) -> GBufferAttachments {

        // Creates color attachments
        let mut color_attachments = Vec::<RenderPassColorAttachment>::with_capacity(self.view_count as usize);
        let ops = Operations {
            load: LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }),
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
        if let Some(view) = &self.color {
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

    /// Resizes all of the textures in the gbuffer to conform to a new size
    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        *self = GBuffer::new(device, width, height, self.format);
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

    fn create_bind_group(
        device: &Device,
        size_buffer: &Buffer,
        sampler: &Sampler,
        pos_view: &TextureView,
        nor_view: &TextureView,
        col_view: Option<&TextureView>
    ) -> (BindGroup, BindGroupLayout) {

        // Creates required layout entries
        let mut layout_entries = Vec::with_capacity(3);
        layout_entries.push(
            // Size
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            },
        );
        layout_entries.push(
            // Sampler
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler {
                    filtering: false,
                    comparison: false
                },
                count: None
            }
        );
        layout_entries.push(
            // Position
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                },
                count: None
            },
        );
        layout_entries.push(
            // Normal
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                },
                count: None
            },
        );

        // Creates required bind group entries
        let mut bind_group_entries = Vec::with_capacity(3);
        bind_group_entries.push(BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: size_buffer,
                offset: 0,
                size: None
            })
        });
        bind_group_entries.push(BindGroupEntry {
            binding: 1,
            resource: BindingResource::Sampler(sampler)
        });
        bind_group_entries.push(BindGroupEntry {
            binding: 2,
            resource: BindingResource::TextureView(pos_view)
        });
        bind_group_entries.push(BindGroupEntry {
            binding: 3,
            resource: BindingResource::TextureView(nor_view)
        });

        // Adds optional entries
        if let Some(col_view) = col_view {
            layout_entries.push(
                // Color
                BindGroupLayoutEntry {
                    binding: 4,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false
                    },
                    count: None
                },
            );
            bind_group_entries.push(BindGroupEntry {
                binding: 4,
                resource: BindingResource::TextureView(col_view)
            });
        }

        // Creates layout and group, then finishes
        let layout_desc = BindGroupLayoutDescriptor {
            label: None,
            entries: layout_entries.as_slice()
        };
        let layout = device.create_bind_group_layout(&layout_desc);
        let group_desc = BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: bind_group_entries.as_slice()
        };
        let bind_group = device.create_bind_group(&group_desc);
        log::debug!("Created GBuffer bind group layout: {:#?}", layout_desc);
        log::debug!("Created GBuffer bind group: {:#?}", group_desc);
        (bind_group, layout)
    }
}

/// Represents the format of a `GBuffer`
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct GBufferFormat {
    position: TextureFormat,
    normal: TextureFormat,
    depth_stencil: Option<TextureFormat>,
    color: Option<TextureFormat>,
    flags: u64
}

impl GBufferFormat {
    pub fn new(flags: u64) -> GBufferFormat {
        let position = TextureFormat::Rgba32Float;
        let normal = TextureFormat::Rgba32Float;
        let mut f = 0;
        let mut color = None;
        let mut depth_stencil = None;
        if flags & GBuffer::COLOR_BUFFER_BIT != 0 {
            color = Some(TextureFormat::Rgba32Float);
            f |= GBuffer::COLOR_BUFFER_BIT;
        }
        if flags & GBuffer::DEPTH_STENCIL_BUFFER_BIT != 0 {
            depth_stencil = Some(TextureFormat::Depth32Float);
            f |= GBuffer::DEPTH_STENCIL_BUFFER_BIT;
        }
        GBufferFormat {
            position,
            normal,
            depth_stencil,
            color,
            flags: f
        }
    }
    pub fn position(&self) -> TextureFormat { self.position }
    pub fn normal(&self) -> TextureFormat { self.normal }
    pub fn depth_stencil(&self) -> Option<TextureFormat> { self.depth_stencil }
    pub fn color(&self) -> Option<TextureFormat> { self.color }
    pub fn flags(&self) -> u64 { self.flags }
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