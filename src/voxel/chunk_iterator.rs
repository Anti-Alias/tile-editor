use crate::voxel::{Chunk, Coords, RawChunk, Selection, VoxelMap};

/// Iterator over `Chunk`s in a `VoxelMap`
pub(crate) struct ChunkIterator<'map> {
    pub(crate) vmap: &'map mut VoxelMap,
    selection: Selection,
    current: Coords
}

impl<'map> ChunkIterator<'map> {
    pub fn new(vmap: &'map mut VoxelMap, selection: Selection) -> Self {
        Self { vmap, selection, current: selection.src }
    }
}

impl<'map> Iterator for ChunkIterator<'map> {
    type Item = Chunk<'map>;
    fn next(&mut self) -> Option<Self::Item> {

        // Unpack
        let src = self.selection.src;
        let dest = self.selection.dest;

        // If we're at the end, just quit
        if self.current.z > dest.z {
            None
        }

        // Otherwise...
        else {
            unsafe {

                // Gets chunk and meta
                let chunk_ptr = self.vmap.chunk_at(self.current) as *mut RawChunk;
                let chunk_ref = (&mut *chunk_ptr) as &'map mut RawChunk;
                let chunk_with_meta = Chunk {
                    coords: self.current,
                    size: self.vmap.chunk_size,
                    raw: chunk_ref
                };

                // Updates iterator position
                self.current.x += 1;
                if self.current.x > dest.x {
                    self.current.x = src.x;
                    self.current.y += 1;
                    if self.current.y > dest.y {
                        self.current.y = src.y;
                        self.current.z += 1;
                    }
                }

                // Return chunk from before
                Some(chunk_with_meta)
            }
        }
    }
}