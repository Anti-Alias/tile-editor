use crate::graphics::gbuffer::GBufferFormat;

/// Represents a permutation of features a pipeline should have
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct GBufferPipelineFeatures {
    pub gbuffer_format: GBufferFormat
}

pub struct GBufferPipelineProvider {

}

impl GBufferPipelineProvider {

}