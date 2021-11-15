use std::collections::HashMap;

use egui_wgpu_backend::wgpu::{FrontFace, PrimitiveTopology};
use wgpu::*;
use crate::graphics::*;
use crate::graphics::gbuffer::{ModelShaderFeatures, ModelShaderProvider};

/// Represents a permutation of features a pipeline should have
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModelPipelineFeatures {
    pub shader_features: ModelShaderFeatures,
    pub color_format: TextureFormat,
    pub depth_stencil_format: TextureFormat
}

/// Provides a pipeline based on features provided
pub struct ModelPipelineProvider {
    pipelines: HashMap<ModelPipelineFeatures, RenderPipeline>
}

impl ModelPipelineProvider {

    /// Creates empty provider
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new()
        }
    }

    /// Creates and returns a pipeline with the specified features.
    /// On subsequent invocations with the same permutation of features, the cached version wil be returned.
    pub fn prime(
        &mut self,
        device: &Device,
        features: &ModelPipelineFeatures,
        shader_provider: &mut ModelShaderProvider,
        bind_group_layouts: &[&BindGroupLayout]
    ) -> &RenderPipeline {
        self.pipelines
            .entry(*features)
            .or_insert_with(|| {
                let shader = shader_provider.prime(device, &features.shader_features);
                let pipeline = Self::create_pipeline(device, &shader, features, bind_group_layouts);
                log::info!("Created new pipeline");
                pipeline
            })
    }

    /// Provides a `RenderPipeline` if it is already cached
    pub fn provide(&self, features: &ModelPipelineFeatures) -> Option<&RenderPipeline> {
        self.pipelines.get(features)
    }

    fn create_pipeline(
        device: &Device,
        module: &ShaderModule,
        features: &ModelPipelineFeatures,
        bind_group_layouts: &[&BindGroupLayout]
    ) -> RenderPipeline {

        // Creates states and layout for pipeline
        let layout = Self::create_pipeline_layout(device, bind_group_layouts);
        let color_targets = [
            ColorTargetState {
                format: features.color_format,
                blend: None,
                write_mask: ColorWrites::ALL
            }
        ];
        let vertex = VertexState {
            module,
            entry_point: "main",
            buffers: &[
                ModelVertex::layout(),
                ModelInstance::layout()
            ]
        };
        let fragment = Some(FragmentState {
            module,
            entry_point: "main",
            targets: &color_targets
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

    fn create_pipeline_layout(device: &Device, bind_group_layouts: &[&BindGroupLayout]) -> PipelineLayout {
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Model Pipeline Layout"),
            bind_group_layouts,
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
            cull_mode: Some(Face::Back),
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