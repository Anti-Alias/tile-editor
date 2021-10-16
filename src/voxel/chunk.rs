use std::fmt::{Debug, Formatter};
use crate::voxel::{Coords, Layer, Selection, Size, Slot};

// -------------------------------------------------------------------------
/// Raw Chunk in a `VoxelMap`.
/// Does not store position or size information.
pub(crate) struct RawChunk {
    layers: Vec<Layer>
}

impl RawChunk {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
}

/// Wraps a `RawChunk` and includes position and size information.
pub(crate) struct Chunk<'map> {
    pub(crate) coords: Coords,          // Offset of chunk in chunk-coords (not global).
    pub(crate) size: Size,              // Size of chunk in voxels
    pub(crate) raw: &'map mut RawChunk  // Inner chunk data
}

impl<'map> Chunk<'map> {

    pub fn slot_at(&mut self, coords: Coords) -> Slot<'map> {
        unsafe {
            let raw = self.raw as *mut RawChunk;
            Slot {
                relative_coords: coords,
                chunk: Chunk {
                    coords: self.coords,
                    size: self.size,
                    raw: (&mut *raw) as &'map mut RawChunk
                }
            }
        }
    }

    // Selection in global voxels
    pub fn selection(&self) -> Selection {
        let src = Coords {
            x: self.coords.x * self.size.width as i32,
            y: self.coords.y * self.size.height as i32,
            z: self.coords.z * self.size.depth as i32
        };
        Selection {
            src,
            dest: Coords {
                x: src.x + self.size.width as i32 - 1,
                y: src.y + self.size.height as i32 - 1,
                z: src.z + self.size.depth as i32 - 1
            }
        }
    }
}

impl<'map> Debug for Chunk<'map> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Chunk {{ coords: {:?}, size: {:?} }}", self.coords, self.size);
        Result::Ok(())
    }
}