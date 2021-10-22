use std::collections::HashMap;
use crate::graphics::Model;
use crate::voxel::Coords;

pub struct VoxelMapGraphics {
    chunks: HashMap<Coords, Model>
}