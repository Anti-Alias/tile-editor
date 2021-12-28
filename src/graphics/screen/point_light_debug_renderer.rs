use std::borrow::Cow;
use wgpu::*;
use crate::graphics::Camera;
use crate::graphics::light::{LightMesh, LightSet, PointLight};
use crate::graphics::util::string_with_lines;

/// Renderer used to visualize the position of point lights.
/// Typically used for debugging and is not intended to be used in a final product.
pub struct PointLightDebugRenderer {
    light_mesh: LightMesh,
    pipeline: RenderPipeline
}

impl PointLightDebugRenderer {

    /// Creates a new debug renderer
    pub fn new(
        device: &Device,
        light_mesh: LightMesh,
        screen_format: TextureFormat,
        depth_stencil_format: TextureFormat,
        camera_bind_group_layout: &BindGroupLayout
    ) -> Self {
        Self::create_from_shader(
            device,
            String::from(include_str!("point_light_debug_shader.wgsl")),
            light_mesh,
            screen_format,
            depth_stencil_format,
            camera_bind_group_layout
        )
    }

    /// Creates a new debug renderer
    pub fn create_from_shader(
        device: &Device,
        shader_source: String,
        light_mesh: LightMesh,
        screen_format: TextureFormat,
        depth_stencil_format: TextureFormat,
        camera_bind_group_layout: &BindGroupLayout
    ) -> Self {
        let module = Self::create_module(device, shader_source.as_str());
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Point Light Debug Renderer Pipeline Layout"),
            bind_group_layouts: &[camera_bind_group_layout],
            push_constant_ranges: &[]
        });
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Point Light Debug Renderer Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &module,
                entry_point: "vert_main",
                buffers: &[
                    LightMesh::vertex_buffer_layout(),
                    PointLight::vertex_buffer_layout()
                ]
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: Some(DepthStencilState {
                format: depth_stencil_format,
                depth_write_enabled: true,
                depth_compare: CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default()
            }),
            multisample: Default::default(),
            fragment: Some(FragmentState {
                module: &module,
                entry_point: "frag_main",
                targets: &[
                    ColorTargetState {
                        format: screen_format,
                        blend: None,
                        write_mask: Default::default()
                    }
                ]
            }),
            multiview: None
        });
        Self {
            light_mesh,
            pipeline
        }
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        point_lights: &'a LightSet<PointLight>,
        camera: &'a Camera
    )  {
        let num_lights = point_lights.lights.len() as u32;
        render_pass.set_vertex_buffer(0, self.light_mesh.vertices.slice(..));                   // Sets light mesh vertices
        render_pass.set_index_buffer(self.light_mesh.indices.slice(..), IndexFormat::Uint32);   // Sets light mesh indices
        render_pass.set_vertex_buffer(1, point_lights.instance_slice());                        // Sets light instance data
        render_pass.set_bind_group(0, camera.bind_group(), &[]);                                // Sets bind group for camera
        render_pass.set_pipeline(&self.pipeline);                                               // Sets pipeline
        render_pass.draw_indexed(0..self.light_mesh.num_indices, 0, 0..num_lights);             // Draws!
    }

    fn create_module(device: &Device, source: &str) -> ShaderModule {
        let source = Self::preprocess_source(source);
        log::info!("Preprocessed gbuffer shader source as:\n{}", string_with_lines(&source));
        let source = ShaderSource::Wgsl(Cow::from(source.as_str()));
        device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source
        })
    }

    fn preprocess_source(source: &str) -> String {

        // Prepares empty preprocessor context
        let mut context = gpp::Context::new();
        let macros = &mut context.macros;

        // Gbuffer bind group
        macros.insert(String::from("M_CAMERA_BIND_GROUP"), 0.to_string());
        macros.insert(String::from("M_CAMERA_BINDING"), 0.to_string());

        // Returns preprocessed string
        gpp::process_str(source, &mut context).unwrap()
    }
}