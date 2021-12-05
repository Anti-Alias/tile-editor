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
    pub fn render<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        instances: &'a ModelInstanceSet,
        camera: &'a Camera
    ) {
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