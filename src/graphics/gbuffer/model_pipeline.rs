use std::collections::HashMap;

use egui_wgpu_backend::wgpu::{FrontFace, PrimitiveTopology};
use wgpu::*;
use crate::graphics::*;
use crate::graphics::gbuffer::*;

/// Represents a permutation of features a pipeline should have
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModelPipelineFeatures {
    pub gbuffer_format: GBufferFormat,
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
                let pipeline = Self::create_pipeline(device, &shader, &features, bind_group_layouts);
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

        // Creates layout and states for pipeline
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Model Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[]
        });
        let vertex = VertexState {
            module,
            entry_point: "main",
            buffers: &[
                ModelVertex::layout(),
                ModelInstance::layout()
            ]
        };
        let color_targets = Self::create_color_targets(&features.gbuffer_format);
        let depth_stencil_target = features.gbuffer_format.depth_stencil().map(|format| {
            DepthStencilState {
                format,
                depth_write_enabled: true,
                depth_compare: CompareFunction::LessEqual,
                stencil: StencilState::default(),
                bias: DepthBiasState::default()
            }
        });
        let fragment = Some(FragmentState {
            module,
            entry_point: "main",
            targets: color_targets.as_slice()
        });
        let primitive = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            clamp_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false
        };
        let multisample = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        };

        // Creates pipeline with layout and states
        let desc = RenderPipelineDescriptor {
            label: Some("Model Render Pipeline"),
            layout: Some(&layout),
            vertex,
            fragment,
            primitive,
            depth_stencil: depth_stencil_target,
            multisample
        };
        device.create_render_pipeline(&desc)
    }

    fn create_color_targets(format: &GBufferFormat) -> Vec<ColorTargetState> {
        let mut targets = vec![
            ColorTargetState {
                format: format.position(),
                blend: None,
                write_mask: ColorWrites::ALL
            },
            ColorTargetState {
                format: format.normal(),
                blend: None,
                write_mask: ColorWrites::ALL
            }
        ];
        if let Some(format) = format.color() {
            targets.push(ColorTargetState {
                format,
                blend: None,
                write_mask: ColorWrites::ALL
            });
        }
        targets
    }
}