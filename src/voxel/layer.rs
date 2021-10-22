use crate::voxel::{VoxelId};

/// Layer in a `RawChunk`
pub(crate) struct Layer(pub Vec<VoxelId>);