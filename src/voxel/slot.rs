use crate::voxel::{Chunk, ChunkIterator, Coords, Selection};

/// Represents a slot in a `VoxelMap` which can store 0 or more `Voxel`s
#[derive(Debug)]
pub struct Slot<'map> {
    pub(crate) relative_coords: Coords,
    pub(crate) chunk: Chunk<'map>
}

impl<'map> Slot<'map> {
    pub fn coords(&self) -> Coords {
        let chunk_coords = self.chunk.coords;
        let chunk_size = self.chunk.size;
        let rel_coords = self.relative_coords;
        Coords {
            x: chunk_coords.x * chunk_size.width as i32 + rel_coords.x,
            y: chunk_coords.y * chunk_size.height as i32 + rel_coords.y,
            z: chunk_coords.z * chunk_size.depth as i32 + rel_coords.z,
        }
    }
}

/// Iterator over `Slot`s in a `VoxelMap`
pub(crate) struct SlotIterator<'map> {
    chunk_iter: ChunkIterator<'map>,    // Iterator over chunks w/ meta
    global_selection: Selection,        // Selection over all slots in global coords
    current_chunk: Chunk<'map>,         // Current chunk w/ meta we're iterating on
    relative_selection: Selection,      // Selection of slots in local coords (local to current chunk)
    current: Coords                     // Current coords of local_selection. Should reset when chunk changes
}

impl<'map> SlotIterator<'map> {
    pub fn new(mut chunk_iter: ChunkIterator<'map>, global_selection: Selection) -> Self {
        let current_chunk = chunk_iter.next().unwrap();
        let relative_selection = Self::relative_intersection(&global_selection, &current_chunk);
        let mut iter = Self {
            chunk_iter,
            current_chunk,
            global_selection,
            relative_selection,
            current: relative_selection.src
        };
        iter
    }

    // Intersects global selection with chunk's global selection, and relativizes it to the chunk's space
    fn relative_intersection(global_selection: &Selection, chunk: &Chunk) -> Selection {
        let mut inter = chunk.selection().intersect(global_selection);
        let global = Coords {
            x: chunk.coords.x * chunk.size.width as i32,
            y: chunk.coords.y * chunk.size.height as i32,
            z: chunk.coords.z * chunk.size.depth as i32
        };
        inter.src.x -= global.x;
        inter.src.y -= global.y;
        inter.src.z -= global.z;
        inter.dest.x -= global.x;
        inter.dest.y -= global.y;
        inter.dest.z -= global.z;
        inter
    }
}

impl<'map> Iterator for SlotIterator<'map> {
    type Item = Slot<'map>;
    fn next(&mut self) -> Option<Self::Item> {

        // Gets src and dest of relative selection
        let (rsrc, rdest) = (self.relative_selection.src, self.relative_selection.dest);

        // If we're at the end of the current chunk, begin the next chunk and invoke next() again
        if self.current.z > rdest.z {
            let next_chunk = self.chunk_iter.next();
            match next_chunk {
                Some(next_chunk) => {
                    self.relative_selection = Self::relative_intersection(&self.global_selection, &next_chunk);
                    self.current = self.relative_selection.src;
                    self.current_chunk = next_chunk;
                    self.next()
                }
                None => None
            }
        }

        // Otherwise, get/return the current slot and update our position in the relative selection
        else {
            let slot = self.current_chunk.slot_at(self.current);
            self.current.x += 1;
            if self.current.x > rdest.x {
                self.current.x = rsrc.x;
                self.current.y += 1;
                if self.current.y > rdest.y {
                    self.current.y = rsrc.y;
                    self.current.z += 1;
                }
            }
            Some(slot)
        }
    }
}