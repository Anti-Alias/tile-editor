use wgpu::*;
use crate::graphics::*;
use crate::graphics::gbuffer::*;

/// Renderer of a `Model`
pub struct ModelRenderer {
    shader_provider: ModelShaderProvider,       // Provider of shaders derived from an ubershader/material features
    pipeline_provider: ModelPipelineProvider,   // Provider of pipelines derived from material features
}

impl ModelRenderer {

    // Helpful local constants
    const VERTEX_BUFFER_SLOT: u32 = 0;
    const INSTANCE_BUFFER_SLOT: u32 = 1;
    const DIFFUSE_TEX_BIND_GROUP: u32 = 0;
    const NORMAL_TEX_BIND_GROUP: u32 = 1;

    /// Creates a `ModelRenderer` with a default shader
    pub fn new() -> ModelRenderer {
        let shader_source = String::from(include_str!("model_ubershader.wgsl"));
        Self::create_from_shader(shader_source)
    }

    /// Creates a `ModelRenderer` with the specified shader
    pub fn create_from_shader(shader_source: String) -> ModelRenderer {
        ModelRenderer {
            shader_provider: ModelShaderProvider::new(shader_source),
            pipeline_provider: ModelPipelineProvider::new(),
        }
    }

    /// Renders a `Model`
    /// * `model` - Model to render
    /// * `device` - Device used to create encoder
    /// * `queue` - Queue to encode draw commands
    /// * `gbuffer` - GBuffer to draw to
    /// * `pipeline_provider` Provider of `RenderPipeline` objects
    pub fn render(
        &self,
        device: &Device,
        queue: &Queue,
        instances: &ModelInstanceSet,
        camera: &Camera,
        gbuffer: &GBuffer,
        clear: bool
    ) {

        // Creates encoder
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("ModelRenderer encoder")
        });

        // Begins render pass with gbuffer's attachments
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Model Renderer Render Pass"),
                color_attachments: &gbuffer.color_attachments(clear),
                depth_stencil_attachment: Some(gbuffer.depth_stencil_attachment(clear))
            });

            // Draws all meshes within the model using render pass
            self.render_with_render_pass(&mut render_pass, instances, camera);
        }

        // Gets commands and writes them to queue
        let commands = encoder.finish();
        queue.submit(std::iter::once(commands));
    }

    fn render_with_render_pass<'a, 'b>(
        &'a self,
        render_pass: &mut RenderPass<'b>,
        instances: &'b ModelInstanceSet,
        camera: &'b Camera
    ) where 'a: 'b {

        // Unpacks environment
        let model = &instances.model;

        // For all mesh/material associations...
        for (mesh, material) in model.iter() {

            // Gets appropriate pipeline for the set of features from material/gbuffer
            let features = ModelPipelineFeatures {
                shader_features: ModelShaderFeatures {
                    material_flags: material.flags()
                }
            };
            let pipeline = &self.pipeline_provider
                .provide(&features)
                .expect("Missing pipeline with features specified");

            // Configures render pass and draws instances
            let num_instances = instances.len() as u32;
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, camera.bind_group(), &[]);
            render_pass.set_bind_group(1, material.bind_group(), &[]);
            render_pass.set_vertex_buffer(Self::VERTEX_BUFFER_SLOT, mesh.vertices.slice(..));
            render_pass.set_vertex_buffer(Self::INSTANCE_BUFFER_SLOT, instances.buffer_slice());
            render_pass.set_index_buffer(mesh.indices.slice(..), IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..num_instances);
        }
    }

    /// Pre-generates/caches pipeline and shader objects ahead of time.
    /// Essentially, this method "primes" for future invocations of `render`.
    /// If the user of `ModelRenderer` neglects to call this before using `render`, they can expect runtime errors.
    /// Multiple invocations with the same arguments have no affect and are essentially free.
    pub fn prime<'a>(
        &mut self,
        device: &Device,
        model: &Model,
        camera_bind_group_layout: &BindGroupLayout
    ) {
        // Generates pipelines and shaders ahead of time
        let pipeline_provider = &mut self.pipeline_provider;
        let shader_provider = &mut self.shader_provider;
        for (_, material) in model.iter() {
            let features = ModelPipelineFeatures {
                shader_features: ModelShaderFeatures {
                    material_flags: material.flags(),
                }
            };
            pipeline_provider.prime(
                device,
                features,
                shader_provider,
                &[
                    camera_bind_group_layout,
                    material.bind_group_layout(),
                ]
            );
        }
    }
}