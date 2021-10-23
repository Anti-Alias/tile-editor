use std::collections::HashMap;
use wgpu::{DepthStencilState, Device, FragmentState, MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor, VertexState};
use crate::graphics::{ShaderFeatures, ShaderProvider};

/// Provides a pipeline based on features provided
pub struct PipelineProvider {
    shader_provider: ShaderProvider,
    pipelines: HashMap<ShaderFeatures, RenderPipeline>
}

impl PipelineProvider {

    /// Provides a `RenderPipeline` based on features specified.
    /// If features have been seen before, uses cached `RenderPipeline`.
    /// Otherwise, creates a new one and caches.
    pub fn provide_or_create(&mut self, device: &Device, features: &ShaderFeatures) -> &RenderPipeline {
        self.pipelines
            .entry(*features)
            .or_insert_with(|| Self::create_pipeline(device, features))
    }

    /// Provides a `RenderPipeline` if it is already cached
    pub fn provide(&self, features: &ShaderFeatures) -> Option<&RenderPipeline> {
        self.pipelines.get(features)
    }

    fn create_pipeline(device: &Device, features: &ShaderFeatures) -> RenderPipeline {
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