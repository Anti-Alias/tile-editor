use wgpu::{Device, FragmentState, MultisampleState, PipelineLayout, PrimitiveState, RenderPipelineDescriptor, VertexState, DepthStencilState, RenderPipeline};
use crate::graphics::{Material, Mesh};

pub struct Model {
    pub mesh: Mesh,
    pub materials: Vec<Material>
}

pub struct ModelRenderer {

}

impl ModelRenderer {
    pub fn new(device: &Device) -> Self {

        let pipeline = Self::create_render_pipeline(device);
        ModelRenderer {

        }
    }

    fn create_pipeline_layout() -> PipelineLayout {
        todo!()
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
        let layout = Self::create_pipeline_layout();
        let vertex = Self::create_vertex_state();
        let primitive = Self::create_primitive_state();
        let depth_stencil = Some(Self::create_depth_stencil_state());
        let multisample = Self::create_multisample_state();
        let fragment = Some(Self::create_fragment_state());
        let desc = RenderPipelineDescriptor {
            label: Some("Model Render Pipeline"),
            layout: Some(&layout),
            vertex,
            primitive,
            depth_stencil,
            multisample,
            fragment
        };
        device.create_render_pipeline(&desc)
    }
}