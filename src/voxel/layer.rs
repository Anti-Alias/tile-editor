use crate::voxel::{Voxel, VoxelId};

/// Layer in a `RawChunk`
pub(crate) struct Layer(pub Vec<VoxelId>);