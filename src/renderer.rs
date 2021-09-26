use wgpu::*;
use crate::{ModelVertex, Vertex};
use std::ops::Range;


/// Renderer instance which owns its own `RenderPipeline`
pub struct Renderer {
    pipeline: RenderPipeline
}

impl Renderer {

    /// Creates a new Renderer instance using the device specified
    pub fn new(device: &Device, config: &RenderConfig) -> Renderer {
        Renderer { pipeline: Self::create_pipeline(device, config) }
    }

    /// Writes render-pass commands to the CommandEncoder specified
    pub fn render(&self, encoder: &mut CommandEncoder, params: &RenderParams) {

        // Begins render pass
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Renderer Render Pass"),
            color_attachments: &[],
            depth_stencil_attachment: None
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, params.vertex_buffer.slice(..));
        render_pass.set_index_buffer(params.vertex_buffer.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(params.index_range.clone(), 0, 0..1);
    }


    // ------------- Pipeline-creation -------------
    fn create_pipeline_layout(device: &Device) -> PipelineLayout {
        device.create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: Some("Renderer Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[]
            }
        )
    }

    fn create_shader_module(device: &Device) -> ShaderModule {
        device.create_shader_module(
            &ShaderModuleDescriptor {
                label: Some("Renderer Shader Module"),
                source: ShaderSource::Wgsl(include_str!("shader.wgsl").into())
            }
        )
    }

    fn create_vertex_state<'a>(module: &'a ShaderModule, buffers: &'a [VertexBufferLayout<'a>]) -> VertexState<'a> {
        VertexState {
            module,
            entry_point: "main",
            buffers
        }
    }

    fn create_primitive_state() -> PrimitiveState {
        PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: Some(IndexFormat::Uint32),
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            clamp_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false
        }
    }

    fn create_multisample_state() -> MultisampleState {
        MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        }
    }

    fn create_fragment_state<'a>(module: &'a ShaderModule, targets: &'a [ColorTargetState]) -> FragmentState<'a> {
        FragmentState {
            module,
            entry_point: "main",
            targets
        }
    }

    fn create_color_target_state(config: &RenderConfig) -> ColorTargetState {
        ColorTargetState {
            format: config.format,
            blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::ALL
        }
    }

    fn create_pipeline(device: &Device, config: &RenderConfig) -> RenderPipeline {
        let vertex_buffer_layouts = [ModelVertex::layout()];
        let color_target_states = [Self::create_color_target_state(config)];
        let shader_module = Self::create_shader_module(device);
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Renderer Pipeline"),
            layout: Some(&Self::create_pipeline_layout(device)),
            vertex: Self::create_vertex_state(&shader_module, &vertex_buffer_layouts),
            primitive: Self::create_primitive_state(),
            depth_stencil: None,
            multisample: Self::create_multisample_state(),
            fragment: Some(Self::create_fragment_state(&shader_module, &color_target_states))
        })
    }
}

/// Configuration for creating a `Renderer` instance
pub struct RenderConfig {
    pub format: TextureFormat
}

impl RenderConfig {

    /// Derives a `RenderConfig` from a `SurfaceConfiguration`
    fn from_surface_config(config: &SurfaceConfiguration) -> Self {
        RenderConfig {
            format: config.format
        }
    }
}

/// Parameters to pass into a `Renderer.draw`
pub struct RenderParams<'a> {

    /// Color attachment to draw to
    pub color_attachment: RenderPassColorAttachment<'a>,

    /// Vertices to use
    pub vertex_buffer: Buffer,

    /// Indices to use
    pub index_buffer: Buffer,

    /// Range of indices to use
    pub index_range: Range<u32>
}