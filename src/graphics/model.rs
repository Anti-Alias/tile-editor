use std::cmp::Ordering;
use std::collections::HashMap;
use wgpu::{Device, FragmentState, MultisampleState, PipelineLayout, PrimitiveState, RenderPipelineDescriptor, VertexState, DepthStencilState, RenderPipeline, PipelineLayoutDescriptor, ShaderModule, CommandEncoderDescriptor, RenderPassDescriptor, CommandEncoder, TextureView, RenderPassColorAttachment, Operations, LoadOp, Color, RenderPassDepthStencilAttachment, IndexFormat, Queue};
use crate::graphics::{Material, Mesh, PipelineProvider, ShaderFeatures, ShaderProvider};

const VERTEX_BUFFER_SLOT: u32 = 0;
const COLOR_TEX_BIND_GROUP: u32 = 0;
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


/// Represents render targets that a `ModelRenderer` can render to
pub struct ModelFrameBuffer {
    color: TextureView,
    depth_stencil: TextureView
}


/// Renderer of a `Model`
pub struct ModelRenderer;
impl ModelRenderer {

    /// Renders a model
    pub fn render(
        &self,
        model: &Model,
        device: &Device,
        queue: &Queue,
        fbo: &ModelFrameBuffer,
        pipeline_provider: &mut PipelineProvider
    ) {
        for (mesh, material) in model.iter() {
            self.render_mesh(mesh, material, device, queue, fbo, pipeline_provider)
        }
    }

    fn render_mesh(
        &self,
        mesh: &Mesh,
        material: &Material,
        device: &Device,
        queue: &Queue,
        fbo: &ModelFrameBuffer,
        pipeline_provider: &mut PipelineProvider
    ) {

        // Acquires pipeline
        let pipeline = pipeline_provider.provide(device);

        // Sets attachments
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

        // Encodes render pass
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Model Renderer Encoder")
        });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Model Renderer Render Pass"),
                color_attachments,
                depth_stencil_attachment: Some(depth_stencil_attachment)
            });
            render_pass.set_vertex_buffer(VERTEX_BUFFER_SLOT, mesh.vertices.slice(..));
            render_pass.set_index_buffer(mesh.indices.slice(..), mesh.index_format);
            render_pass.set_pipeline(pipeline);
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }

        // Submits commands
        let cmd_buffer = encoder.finish();
        queue.submit(std::iter::once(cmd_buffer));
    }
}