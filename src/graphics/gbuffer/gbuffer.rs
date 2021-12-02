use wgpu::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};


/// Geometry buffer that stores a multitude of color targets and a depth_stencil target
pub struct GBuffer {
    position: TextureView,
    normal: TextureView,
    depth_stencil: TextureView,
    color: TextureView,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout
}
impl GBuffer {

    pub const POSITION_FORMAT: TextureFormat = TextureFormat::Rgba32Float;
    pub const NORMAL_FORMAT: TextureFormat = TextureFormat::Rgba32Float;
    pub const COLOR_FORMAT: TextureFormat = TextureFormat::Rgba32Float;
    pub const DEPTH_STENCIL_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    /// Creates a GBuffer.
    pub fn new(device: &Device, width: u32, height: u32) -> Self {
        let position = device
            .create_texture(&Self::descriptor_of(width, height, Self::POSITION_FORMAT))
            .create_view(&TextureViewDescriptor::default());
        let normal = device
            .create_texture(&Self::descriptor_of(width, height, Self::NORMAL_FORMAT))
            .create_view(&TextureViewDescriptor::default());
        let color = device
            .create_texture(&Self::descriptor_of(width, height, Self::COLOR_FORMAT))
            .create_view(&TextureViewDescriptor::default());
        let depth_stencil = device
            .create_texture(&Self::descriptor_of(width, height, Self::DEPTH_STENCIL_FORMAT))
            .create_view(&TextureViewDescriptor::default());
        let (bind_group, bind_group_layout) = Self::create_bind_group(
            device,
            &position,
            &normal,
            &color
        );
        log::info!("Created GBuffer");
        Self {
            position,
            normal,
            depth_stencil,
            color,
            bind_group,
            bind_group_layout
        }
    }

    pub fn position_view(&self) -> &TextureView { &self.position }
    pub fn normal_view(&self) -> &TextureView { &self.normal }
    pub fn depth_stencil_view(&self) -> &TextureView { &self.depth_stencil }
    pub fn color_view(&self) -> &TextureView { &self.color }
    pub fn bind_group(&self) -> &BindGroup { &self.bind_group }
    pub fn bind_group_layout(&self) -> &BindGroupLayout { &self.bind_group_layout }

    pub fn color_attachments(&self, clear: bool) -> [RenderPassColorAttachment; 3] {

        // Determines load operation
        let load =
            if clear { LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }) }
            else { LoadOp::Load };
        let ops = Operations { load, store: true };

        // Returns attachments
        [
            RenderPassColorAttachment {
                view: &self.position,
                resolve_target: None,
                ops
            },
            RenderPassColorAttachment {
                view: &self.normal,
                resolve_target: None,
                ops
            },
            RenderPassColorAttachment {
                view: &self.color,
                resolve_target: None,
                ops
            }
        ]
    }

    /// Both the color buffer and depth_stencil attachments
    pub fn depth_stencil_attachment(&self, clear: bool) -> RenderPassDepthStencilAttachment {
        let load = if clear { LoadOp::Clear(1.0)} else { LoadOp::Load };
        RenderPassDepthStencilAttachment {
            view: &self.depth_stencil,
            depth_ops: Some(Operations { load, store: true }),
            stencil_ops: None
        }
    }

    /// Resizes all of the textures in the gbuffer to conform to a new size
    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        *self = GBuffer::new(device, width, height);
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
        pos_view: &TextureView,
        nor_view: &TextureView,
        col_view: &TextureView
    ) -> (BindGroup, BindGroupLayout) {

        // Creates required layout entries
        let layout_entries = [
            // Position
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                },
                count: None
            },
            // Normal
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                },
                count: None
            },
            // Color
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                },
                count: None
            }
        ];

        // Creates required bind group entries
        let mut bind_group_entries = [
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(pos_view)
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(nor_view)
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(col_view)
            }
        ];

        // Creates layout and group, then finishes
        let layout_desc = BindGroupLayoutDescriptor {
            label: None,
            entries: &layout_entries
        };
        let layout = device.create_bind_group_layout(&layout_desc);
        let group_desc = BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &bind_group_entries
        };
        let bind_group = device.create_bind_group(&group_desc);
        log::debug!("Created GBuffer bind group layout: {:#?}", layout_desc);
        log::debug!("Created GBuffer bind group: {:#?}", group_desc);
        (bind_group, layout)
    }
}

/// Represents color attachments and depth-stencil attachments of a `GBuffer`.
pub struct GBufferAttachments<'a> {
    pub color_attachments: [RenderPassColorAttachment<'a>; 3],
    pub depth_stencil_attachment: RenderPassDepthStencilAttachment<'a>
}
