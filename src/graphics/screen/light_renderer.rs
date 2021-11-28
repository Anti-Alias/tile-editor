use std::borrow::Cow;
use wgpu::*;
use crate::graphics::gbuffer::GBuffer;
use crate::graphics::Camera;
use crate::graphics::util::string_with_lines;

/// Responsible for rendering ambient and directional lights to a screen using a `GBuffer`.
pub struct LightRenderer {
    pipeline: RenderPipeline
}

impl LightRenderer {

    /// Creates a new `LightRenderer` with a default shader
    pub fn new(device: &Device, screen_format: TextureFormat, gbuffer: &GBuffer)-> Self {
        Self::create_from_shader(
            device,
            String::from(include_str!("light_shader.wgsl")),
            screen_format,
            gbuffer
        )
    }

    /// Creates a `LightRenderer` with the specified shader code
    pub fn create_from_shader(device: &Device, shader_source: String, screen_format: TextureFormat, gbuffer: &GBuffer) -> Self {
        let module = Self::create_module(device, &shader_source);
        let pipeline = Self::create_pipeline(device, &module, screen_format, gbuffer);
        Self { pipeline }
    }

    /// Renders ambient and diffuse lights to the screen using a `GBuffer`.
    pub fn render(&self, device: &Device, queue: &Queue, screen: &TextureView, gbuffer: &GBuffer) {

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        let color_attachments = &[
            RenderPassColorAttachment {
                view: screen,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
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
            render_pass.set_bind_group(0, gbuffer.bind_group(), &[]);
            render_pass.set_pipeline(&self.pipeline);   // Sets pipeline
            render_pass.draw(0..6, 0..1);               // Draws!
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
        gbuffer: &GBuffer
    ) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Light Rnederer Pipeline Layout"),
            bind_group_layouts: &[gbuffer.bind_group_layout()],
            push_constant_ranges: &[]
        });
        let vertex = VertexState {
            module,
            entry_point: "main",
            buffers: &[]
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
        let fragment = FragmentState {
            module,
            entry_point: "main",
            targets: &color_targets
        };
        let primitive = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            clamp_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false
        };
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Light Renderer Render Pipeline"),
            layout: Some(&layout),
            vertex,
            primitive,
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(fragment)
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

        // Returns preprocessed string
        gpp::process_str(source, &mut context).unwrap()
    }
}