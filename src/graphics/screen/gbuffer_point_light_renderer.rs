use std::borrow::Cow;
use std::collections::HashMap;
use wgpu::*;
use crate::graphics::Camera;
use crate::graphics::gbuffer::GBuffer;
use crate::graphics::light::{LightMesh, LightSet, PointLight};
use crate::graphics::screen::ScreenBuffer;
use crate::graphics::util::string_with_lines;

/// Responsible for rendering point lights to a screen while sampling from a `GBuffer`.
pub struct GBufferPointLightRenderer {
    shader_source: String,                  // Source of shader code
    modules: HashMap<u64, ShaderModule>,    // Flags (unused currently) -> module
    pipelines: HashMap<u64, RenderPipeline>,// Flags -> pipeline,
    screen_format: TextureFormat            // Format of the screen that will be rendered to
}

impl GBufferPointLightRenderer {

    /// Creates a new `GBufferRenderer` with a default shader
    pub fn new(screen_format: TextureFormat)-> Self {
        Self::create_from_shader(String::from(include_str!("gbuffer_ubershader.wgsl")), screen_format)
    }

    /// Creates a `GBufferRenderer` with the specified shader code
    pub fn create_from_shader(shader_source: String, screen_format: TextureFormat) -> Self {
        Self {
            shader_source,
            modules: HashMap::new(),
            pipelines: HashMap::new(),
            screen_format
        }
    }

    /// Primes the `GBufferRenderer` to render a `GBuffer` the the specified format
    pub fn prime(
        &mut self,
        device: &Device,
        gbuffer: &GBuffer,
        camera: &Camera
    ) {
        let screen_format = self.screen_format;
        let shader_source = self.shader_source.as_ref();
        let module = self.modules
            .entry(0)   // Flags not yet in use
            .or_insert_with(|| { Self::create_module(device, shader_source) });
        self.pipelines
            .entry(0)   // Flags not yet in use
            .or_insert_with(|| {
                Self::create_pipeline(
                    device,
                    module,
                    screen_format,
                    gbuffer,
                    camera
                )}
            );
    }

    /// Renders the gbuffer to the screen
    pub fn render(
        &self,
        device: &Device,
        queue: &Queue,
        screen: &TextureView,
        gbuffer: &GBuffer,
        lights: &LightSet<PointLight>,
        light_mesh: &LightMesh,
        camera: &Camera
    ) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        let color_attachments = &[
            RenderPassColorAttachment {
                view: screen,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }),
                    store: true
                }
            }
        ];

        // Render pass
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments,
                depth_stencil_attachment: None
            });
            let pipeline = &self.pipelines[&0];                                                 // Flags not yet implemented
            let num_lights = lights.lights.len() as u32;
            render_pass.set_vertex_buffer(0, light_mesh.vertices.slice(..));                    // Sets light mesh vertices
            render_pass.set_index_buffer(light_mesh.indices.slice(..), IndexFormat::Uint32);    // Sets light mesh indices
            render_pass.set_vertex_buffer(1, lights.instance_slice());                          // Sets light instance data
            render_pass.set_bind_group(0, gbuffer.bind_group(), &[]);                           // Sets bind group for GBuffer (collection of textures)
            render_pass.set_bind_group(1, camera.bind_group(), &[]);                            // Sets bind group for camera
            render_pass.set_pipeline(pipeline);                                                 // Sets pipeline
            render_pass.draw_indexed(0..light_mesh.num_indices, 0, 0..num_lights)               // Draws!
        }

        // Submits commands
        let commands = encoder.finish();
        queue.submit(std::iter::once(commands));
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

    fn create_pipeline(
        device: &Device,
        module: &ShaderModule,
        screen_format: TextureFormat,
        gbuffer: &GBuffer,
        camera: &Camera
    ) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                gbuffer.bind_group_layout(),
                camera.bind_group_layout()
            ],
            push_constant_ranges: &[]
        });
        let vertex = VertexState {
            module,
            entry_point: "main",
            buffers: &[
                LightMesh::layout(),
                PointLight::layout()
            ]
        };
        let color_targets = [
            ColorTargetState {
                format: screen_format,
                blend: Some(BlendState {
                    color: BlendComponent {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::One,
                        operation: BlendOperation::Add
                    },
                    alpha: BlendComponent::REPLACE
                }),
                write_mask: ColorWrites::ALL
            }
        ];
        let fragment = Some(FragmentState {
            module,
            entry_point: "main",
            targets: &color_targets
        });
        let primitive = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Front),
            clamp_depth: false,
            polygon_mode: Default::default(),
            conservative: false
        };
        let multisample = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        };
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex,
            primitive,
            depth_stencil: None,
            multisample,
            fragment
        })
    }


    fn preprocess_source(source: &str) -> String {

        // Prepares empty preprocessor context
        let mut context = gpp::Context::new();
        let macros = &mut context.macros;

        // Gbuffer bind group
        macros.insert(String::from("M_GBUFFER_BIND_GROUP"), String::from("0"));
        macros.insert(String::from("M_POSITION_TEXTURE_BINDING"), String::from("0"));
        macros.insert(String::from("M_NORMAL_TEXTURE_BINDING"), String::from("1"));
        macros.insert(String::from("M_COLOR_TEXTURE_BINDING"), String::from("2"));

        // Camera bind group
        macros.insert(String::from("M_CAMERA_BIND_GROUP"), String::from("1"));
        macros.insert(String::from("M_CAMERA_BINDING"), String::from("0"));

        // Returns preprocessed string
        gpp::process_str(source, &mut context).unwrap()
    }
}