use crate::voxel::{Chunk, Coords};

/// Represents a slot in a `VoxelMap` which can store 0 or more `Voxel`s
#[derive(Debug)]
pub struct Slot<'map> {
    pub(crate) relative_coords: Coords,
    pub(crate) chunk: Chunk<'map>
}

impl<'map> Slot<'map> {
    pub fn coords(&self) -> Coords {
        let cc = self.chunk.coords;
        let cs = self.chunk.size;
        let rc = self.relative_coords;
        Coords {
            x: cc.x * cs.width as i32 + rc.x,
            y: cc.y * cs.height as i32 + rc.y,
            z: cc.z * cs.depth as i32 + rc.z,
        }
    }
}