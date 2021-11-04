use std::collections::HashMap;

use egui_wgpu_backend::wgpu::{FrontFace, PrimitiveTopology};
use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device, FragmentState, IndexFormat, MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderStages, StencilState, TextureFormat, VertexBufferLayout, VertexState, VertexStepMode};
use crate::graphics::{ModelVertex, ShaderFeatures, ShaderProvider, Vertex};

/// Represents a permutation of features a pipeline should have
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct PipelineFeatures {
    pub shader_features: ShaderFeatures,
    pub color_format: TextureFormat,
    pub depth_stencil_format: TextureFormat
}

/// Provides a pipeline based on features provided
pub struct PipelineProvider {
    pipelines: HashMap<PipelineFeatures, RenderPipeline>
}

impl PipelineProvider {

    /// Creates empty provider
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new()
        }
    }

    /// Provides a `RenderPipeline` based on features specified.
    /// If features have been seen before, uses cached `RenderPipeline`.
    /// Otherwise, creates a new one and caches.
    pub fn provide_or_create(
        &mut self,
        device: &Device,
        features: &PipelineFeatures,
        shader_provider: &mut ShaderProvider
    ) -> &RenderPipeline {
        self.pipelines
            .entry(*features)
            .or_insert_with(|| {
                let shader = shader_provider.provide_or_create(device, &features.shader_features);
                let pipeline = Self::create_pipeline(device, &shader, &features);
                log::info!("Created new pipeline");
                pipeline
            })
    }

    /// Provides a `RenderPipeline` if it is already cached
    pub fn provide(&self, features: &PipelineFeatures) -> Option<&RenderPipeline> {
        self.pipelines.get(features)
    }

    fn create_pipeline(device: &Device, module: &ShaderModule, features: &PipelineFeatures) -> RenderPipeline {

        // Creates states and layout for pipeline
        let layout = Self::create_pipeline_layout(device);
        let targets = [
            ColorTargetState {
                format: features.color_format,
                blend: None,
                write_mask: ColorWrites::ALL
            }
        ];
        let vertex = VertexState {
            module,
            entry_point: "main",
            buffers: &[ModelVertex::layout()]
        };
        let fragment = Some(FragmentState {
            module,
            entry_point: "main",
            targets: &targets
        });

        // Creates pipeline
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Model Render Pipeline"),
            layout: Some(&layout),
            vertex,
            fragment,
            primitive: Self::create_primitive_state(),
            depth_stencil: Some(Self::create_depth_stencil_state(features.depth_stencil_format)),
            multisample: Self::create_multisample_state(),
        })
    }

    fn create_pipeline_layout(device: &Device) -> PipelineLayout {
        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }]
        });
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Model Pipeline Layout"),
            bind_group_layouts: &[&layout],
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
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            clamp_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false
        }
    }

    fn create_depth_stencil_state(format: TextureFormat) -> DepthStencilState {
        DepthStencilState {
            format,
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