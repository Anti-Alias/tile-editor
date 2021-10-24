use std::cmp::Ordering;
use std::collections::HashMap;
use wgpu::{Device, FragmentState, MultisampleState, PipelineLayout, PrimitiveState, RenderPipelineDescriptor, VertexState, DepthStencilState, RenderPipeline, PipelineLayoutDescriptor, ShaderModule, CommandEncoderDescriptor, RenderPassDescriptor, CommandEncoder, TextureView, RenderPassColorAttachment, Operations, LoadOp, Color, RenderPassDepthStencilAttachment, IndexFormat, Queue, RenderPass};
use crate::graphics::{Material, Mesh, PipelineProvider, RGBA, ShaderFeatures, ShaderProvider, Vector3};

// Helpful local constants
const VERTEX_BUFFER_SLOT: u32 = 0;
const INSTANCE_BUFFER_SLOT: u32 = 1;
const DIFFUSE_TEX_BIND_GROUP: u32 = 0;
const NORMAL_TEX_BIND_GROUP: u32 = 1;

/// Represents a set of meshes associated with materials
/// Meshes and materials can only be rendered if their indices are placed in the associations vector
pub struct Model {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
    associations: Vec<(usize, usize)>
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
pub struct ModelFrameBuffer {
    color: TextureView,
    depth_stencil: TextureView
}


/// Renderer of a `Model`
pub struct ModelRenderer;
impl ModelRenderer {

    /// Renders a `Model`
    /// * `model` - Model to render
    /// * `device` - Device used to create encoder
    /// * `queue` - Location to encode draw commands
    /// * `fbo` - Location to draw to
    /// * `pipeline_provider` Provider of `RenderPipeline` objects
    pub fn render(
        &self,
        model: &Model,
        device: &Device,
        queue: &Queue,
        fbo: &ModelFrameBuffer,
        pipeline_provider: &mut PipelineProvider
    ) {
        // Creates encoder
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("ModelRenderer encoder")
        });

        // Adds render commands to encoder
        self.render_to_encoder(model, &mut encoder, fbo, pipeline_provider);

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
        &self,
        model: &Model,
        encoder: &mut CommandEncoder,
        fbo: &ModelFrameBuffer,
        pipeline_provider: &mut PipelineProvider
    ) {

        // Creates attachments
        let color_attachments = &[
            RenderPassColorAttachment {
                view: &fbo.color,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
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
        self.render_model(model, &mut render_pass, pipeline_provider);
    }

    /// Creates pipeline objects for a particular Model ahead of time if they don't already exist
    pub fn prepare_for_model<'a>(
        &self,
        device: &Device,
        model: &'a Model,
        render_pass: &mut RenderPass<'a>,
        pipeline_provider: &'a mut PipelineProvider
    ) {
        for (_, material) in model.iter() {
            let features = ShaderFeatures { material_flags: material.flags() };
            pipeline_provider.provide_or_create(device, &features);
        }
    }

    fn render_model<'a>(
        &self,
        model: &'a Model,
        render_pass: &mut RenderPass<'a>,
        pipeline_provider: &'a mut PipelineProvider
    ) {
        for (mesh, material) in model.iter() {
            let features = ShaderFeatures { material_flags: material.flags() };
            let pipeline: &RenderPipeline = pipeline_provider
                .provide(&features)
                .expect("Missing pipeline with features specified");
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(VERTEX_BUFFER_SLOT, mesh.vertices.slice(..));
            render_pass.set_index_buffer(mesh.indices.slice(..), IndexFormat::Uint32);
        }
    }
}