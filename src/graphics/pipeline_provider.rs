use wgpu::{DepthStencilState, Device, FragmentState, MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor, VertexState};
use crate::graphics::ShaderProvider;

/// Provides a pipeline based on features provided
pub struct PipelineProvider {
    shader_provider: ShaderProvider
}

impl PipelineProvider {
    pub fn provide(&self, device: &Device) -> &RenderPipeline {
        /*
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
         */
        //device.create_render_pipeline(&desc)
        todo!()
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