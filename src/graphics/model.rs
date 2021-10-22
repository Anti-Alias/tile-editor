use std::collections::HashMap;
use wgpu::{Device, FragmentState, MultisampleState, PipelineLayout, PrimitiveState, RenderPipelineDescriptor, VertexState, DepthStencilState, RenderPipeline, PipelineLayoutDescriptor, ShaderModule};
use crate::graphics::{Material, Mesh, ShaderFeatures};

pub struct Model {
    pub mesh: Mesh,
    pub materials: Vec<Material>
}

pub struct ModelRenderer {
    pipeline: RenderPipeline,
    shaders: HashMap<u64, ShaderModule>
}

impl ModelRenderer {
    pub fn new(device: &Device) -> Self {
        let shaders = todo!();
        ModelRenderer {
            pipeline: Self::create_render_pipeline(device),
            shaders
        }
    }

    fn create_pipeline_layout(device: &Device) -> PipelineLayout {
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Model Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        })
    }

    fn create_vertex_state<'a>() -> VertexState<'a> {
        todo!()
    }

    fn create_primitive_state() -> PrimitiveState {
        todo!()
    }

    fn create_depth_stencil_state() -> DepthStencilState {
        todo!()
    }

    fn create_multisample_state() -> MultisampleState {
        todo!()
    }

    fn create_fragment_state<'a>() -> FragmentState<'a> {
        todo!()
    }

    fn create_render_pipeline(device: &Device) -> RenderPipeline {
        let layout = Self::create_pipeline_layout(device);
        let vertex = Self::create_vertex_state();
        let fragment = Some(Self::create_fragment_state());
        let primitive = Self::create_primitive_state();
        let depth_stencil = Some(Self::create_depth_stencil_state());
        let multisample = Self::create_multisample_state();
        let desc = RenderPipelineDescriptor {
            label: Some("Model Render Pipeline"),
            layout: Some(&layout),
            vertex,
            fragment,
            primitive,
            depth_stencil,
            multisample,
        };
        device.create_render_pipeline(&desc)
    }
}