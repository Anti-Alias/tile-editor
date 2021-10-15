pub enum Voxel {
    Empty,
    Cube
}

impl Default for Voxel {
    fn default() -> Self {
        Self::Empty
    }
}