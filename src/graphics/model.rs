use std::cmp::Ordering;
use std::collections::HashMap;
use wgpu::{Device, FragmentState, MultisampleState, PipelineLayout, PrimitiveState, RenderPipelineDescriptor, VertexState, DepthStencilState, RenderPipeline, PipelineLayoutDescriptor, ShaderModule, CommandEncoderDescriptor, RenderPassDescriptor, CommandEncoder, TextureView, RenderPassColorAttachment, Operations, LoadOp, RenderPassDepthStencilAttachment, IndexFormat, Queue, RenderPass, TextureFormat};
use crate::graphics::{Material, Mesh, PipelineProvider, Color, ShaderFeatures, ShaderProvider, PipelineFeatures};

// Helpful local constants
const VERTEX_BUFFER_SLOT: u32 = 0;
const INSTANCE_BUFFER_SLOT: u32 = 1;
const DIFFUSE_TEX_BIND_GROUP: u32 = 0;
const NORMAL_TEX_BIND_GROUP: u32 = 1;

/// Represents a set of meshes associated with materials
/// Meshes and materials can only be rendered if their indices are placed in the associations vector
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub associations: Vec<(usize, usize)>
}

impl Model {
    fn iter(&self) -> impl Iterator<Item=(&Mesh, &Material)> {
        self.associations.iter().map(move |association| {
            let mesh_idx = association.0;
            let mat_idx = association.1;
            (&self.meshes[mesh_idx], &self.materials[mat_idx])
        })
    }
}


/// Represents a set render targets that a `ModelRenderer` can render to
pub struct ModelFrameBuffer<'a> {
    pub color: &'a TextureView,
    pub depth_stencil: &'a TextureView
}

/// Renderer of a `Model`
pub struct ModelRenderer {
    shader_provider: ShaderProvider,        // Provider of shaders derived from an ubershader/material features
    pipeline_provider: PipelineProvider,    // Provider of pipelines derived from material features
    color_format: TextureFormat             // Expected format of texture view being drawn to
}
impl ModelRenderer {

    /// Creates a `ModelRenderer` with a default shader
    pub fn new(color_format: TextureFormat) -> ModelRenderer {
        let shader_source = String::from(include_str!("model_ubershader.wgsl"));
        Self::create_from_shader(shader_source, color_format)
    }

    /// Creates a `ModelRenderer` with the specified shader
    pub fn create_from_shader(shader_source: String, color_format: TextureFormat) -> ModelRenderer {
        ModelRenderer {
            shader_provider: ShaderProvider::new(shader_source),
            pipeline_provider: PipelineProvider::new(),
            color_format
        }
    }

    /// Renders a `Model`
    /// * `model` - Model to render
    /// * `device` - Device used to create encoder
    /// * `queue` - Location to encode draw commands
    /// * `fbo` - Location to draw to
    /// * `pipeline_provider` Provider of `RenderPipeline` objects
    pub fn render(
        &mut self,
        model: &Model,
        device: &Device,
        queue: &Queue,
        fbo: &ModelFrameBuffer
    ) {
        // Creates encoder
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("ModelRenderer encoder")
        });

        // Adds render commands to encoder
        self.render_to_encoder(model, &mut encoder, fbo);

        // Gets commands and writes them to queue
        let commands = encoder.finish();
        queue.submit(std::iter::once(commands));
    }

    /// Renders a `Model` using an existing `CommandEncoder`
    /// * `model` - Model to render
    /// * `encoder` - Command encoder to write commands to
    /// * `queue` - Location to encode draw commands
    /// * `fbo` - Location to draw to
    /// * `pipeline_provider` Provider of `RenderPipeline` objects
    pub fn render_to_encoder(
        &mut self,
        model: &Model,
        encoder: &mut CommandEncoder,
        fbo: &ModelFrameBuffer
    ) {

        // Creates attachments
        let color_attachments = &[
            RenderPassColorAttachment {
                view: &fbo.color,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(wgpu::Color {r: 0.5, g: 0.5, b: 0.5, a: 1.0}),
                    store: true
                }
            }
        ];
        let depth_stencil_attachment = RenderPassDepthStencilAttachment {
            view: &fbo.depth_stencil,
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: true
            }),
            stencil_ops: None
        };

        // Begins render pass with attachments
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Model Renderer Render Pass"),
            color_attachments,
            depth_stencil_attachment: Some(depth_stencil_attachment)
        });

        // Draws all meshes within the model using render pass
        self.render_model(model, &mut render_pass);
    }

    /// Creates pipeline objects for a particular Model ahead of time if they don't already exist
    pub fn prepare_for_model<'a>(
        &mut self,
        device: &Device,
        model: &'a Model
    ) {
        let pipeline_provider = &mut self.pipeline_provider;
        let shader_provider = &mut self.shader_provider;
        for (_, material) in model.iter() {
            let features = PipelineFeatures {
                shader_features: ShaderFeatures { material_flags: material.flags() },
                color_format: self.color_format
            };
            pipeline_provider.provide_or_create(device, &features, shader_provider);
        }
    }

    fn render_model<'a, 'b>(
        &'a self,
        model: &'b Model,
        render_pass: &mut RenderPass<'b>,
    ) where 'a: 'b {
        let pipeline_provider = &self.pipeline_provider;
        for (mesh, material) in model.iter() {
            let features = PipelineFeatures {
                shader_features: ShaderFeatures { material_flags: material.flags() },
                color_format: self.color_format
            };
            let pipeline = pipeline_provider
                .provide(&features)
                .expect("Missing pipeline with features specified");
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(VERTEX_BUFFER_SLOT, mesh.vertices.slice(..));
            render_pass.set_index_buffer(mesh.indices.slice(..), IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            log::info!("Drawing!!!");
        }
    }
}