use std::collections::HashMap;

use egui_wgpu_backend::wgpu::{FrontFace, PrimitiveTopology};
use wgpu::*;
use crate::graphics::*;
use crate::graphics::gbuffer::*;

/// Represents a permutation of features a pipeline should have
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModelPipelineFeatures {
    pub shader_features: ModelShaderFeatures
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
        features: ModelPipelineFeatures,
        shader_provider: &mut ModelShaderProvider,
        bind_group_layouts: &[&BindGroupLayout]
    ) -> &RenderPipeline {
        self.pipelines
            .entry(features)
            .or_insert_with(|| {
                let shader = shader_provider.prime(device, &features.shader_features);
                let pipeline = Self::create_pipeline(device, &shader, bind_group_layouts);
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
        bind_group_layouts: &[&BindGroupLayout]
    ) -> RenderPipeline {

        // Creates layout and states for pipeline
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Model Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[]
        });
        let vertex = VertexState {
            module,
            entry_point: "vert_main",
            buffers: &[
                ModelVertex::layout(),
                ModelInstance::layout()
            ]
        };
        let color_target_states = [
            ColorTargetState {
                format: GBuffer::POSITION_FORMAT,
                blend: None,
                write_mask: ColorWrites::ALL
            },
            ColorTargetState {
                format: GBuffer::NORMAL_FORMAT,
                blend: None,
                write_mask: ColorWrites::ALL
            },
            ColorTargetState {
                format: GBuffer::COLOR_FORMAT,
                blend: None,
                write_mask: ColorWrites::ALL
            }
        ];
        let depth_stencil_state =  Some(DepthStencilState {
            format: GBuffer::DEPTH_STENCIL_FORMAT,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState::default(),
            bias: DepthBiasState::default()
        });
        let fragment = Some(FragmentState {
            module,
            entry_point: "frag_main",
            targets: &color_target_states
        });
        let primitive = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            unclipped_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false
        };
        let multisample = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        };

        // Creates pipeline with layout and states
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Model Render Pipeline"),
            layout: Some(&layout),
            vertex,
            fragment,
            primitive,
            depth_stencil: depth_stencil_state,
            multisample,
            multiview: None
        });
        pipeline
    }
}