use std::collections::HashMap;
use std::sync::Arc;
use egui_wgpu_backend::wgpu::{FrontFace, PrimitiveTopology};
use wgpu::{BindGroupLayout, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device, FragmentState, IndexFormat, MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModule, StencilState, TextureFormat, VertexBufferLayout, VertexState, VertexStepMode};
use crate::graphics::{ModelVertex, ShaderFeatures, ShaderProvider, Vertex};

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
        let shader_provider = &mut self.shader_provider;
        self.pipelines
            .entry(*features)
            .or_insert_with(|| {
                let shader = shader_provider.provide_or_create(device, features);
                Self::create_pipeline(device, &shader)
            })
    }

    /// Provides a `RenderPipeline` if it is already cached
    pub fn provide(&self, features: &ShaderFeatures) -> Option<&RenderPipeline> {
        self.pipelines.get(features)
    }

    fn create_pipeline(device: &Device, module: &ShaderModule) -> RenderPipeline {

        // Creates states and layout for pipeline
        let layout = Self::create_pipeline_layout(device);
        let vertex = VertexState {
            module,
            entry_point: "main",
            buffers: &[ModelVertex::layout()]
        };
        let fragment = Some(FragmentState {
            module,
            entry_point: "main",
            targets: &[
                ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: None,
                    write_mask: ColorWrites::ALL
                }
            ]
        });

        // Creates pipeline
        let desc = RenderPipelineDescriptor {
            label: Some("Model Render Pipeline"),
            layout: Some(&layout),
            vertex,
            fragment,
            primitive: Self::create_primitive_state(),
            depth_stencil: Some(Self::create_depth_stencil_state()),
            multisample: Self::create_multisample_state(),
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

    fn create_vertex_state<'a>(module: &'a ShaderModule, layout: &'a [VertexBufferLayout]) -> VertexState<'a> {
        VertexState {
            module,
            entry_point: "main",
            buffers: layout
        }
    }

    fn create_primitive_state() -> PrimitiveState {
        PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: Some(IndexFormat::Uint32),
            front_face: FrontFace::Ccw,
            cull_mode: None,
            clamp_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false
        }
    }

    fn create_depth_stencil_state() -> DepthStencilState {
        DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState::default(),
            bias: DepthBiasState::default()
        }
    }

    fn create_multisample_state() -> MultisampleState {
        MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        }
    }
}