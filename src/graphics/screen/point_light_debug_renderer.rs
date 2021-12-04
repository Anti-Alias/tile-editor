use wgpu::*;

/// Renderer used to visualize the position of point lights.
/// Typically used for debugging and is not intended to be used in a final product.
pub struct PointLightDebugRenderer { pipeline: RenderPipeline }

impl PointLightDebugRenderer {

    /// Creates a new debug renderer
    pub fn new(device: &Device, light_radius: f32) -> Self {
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Point Light Debug Renderer Pipeline"),
            layout: None,
            vertex: VertexState {
                module: todo!(),
                entry_point: "main",
                buffers: todo!()
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                clamp_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default()
            }),
            multisample: Default::default(),
            fragment: None
        });
        Self { pipeline }
    }

    pub fn create_shader_module(src: &str) -> ShaderModule {

    }
}