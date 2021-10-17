pub enum Voxel {
    Empty,
    Cube
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VoxelId {
    Empty,
    Index {
        set_idx: u16,
        voxel_idx: u16
    }
}

impl Default for Voxel {
    fn default() -> Self {
        Self::Empty
    }
}