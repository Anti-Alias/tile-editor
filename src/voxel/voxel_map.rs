use std::collections::HashMap;
use crate::voxel::{Chunk, ChunkIterator, Coords, RawChunk, Selection, Size, Slot, SlotIterator};

/// A model struct storing voxel data
pub struct VoxelMap {
    pub(crate) chunk_size: Size,
    chunks: HashMap<Coords, RawChunk>
}

impl VoxelMap {
    pub fn new(chunk_size: Size) -> Self {
        Self {
            chunk_size,
            chunks: HashMap::new()
        }
    }

    pub fn slot_at(&mut self, coords: &Coords) -> Slot {
        let chunk_size = self.chunk_size;
        let chunk_coords = self.global_to_chunk_coords(coords);
        let global_chunk_coords = Coords {
            x: chunk_coords.x * chunk_size.width as i32,
            y: chunk_coords.y * chunk_size.height as i32,
            z: chunk_coords.z * chunk_size.depth as i32
        };
        let raw_chunk = self.raw_chunk_at(chunk_coords);
        Slot {
            relative_coords: Coords {
                x: coords.x - global_chunk_coords.x,
                y: coords.y - global_chunk_coords.y,
                z: coords.z - global_chunk_coords.z
            },
            chunk: Chunk {
                coords: chunk_coords,
                size: chunk_size,
                raw: raw_chunk
            }
        }
    }

    pub fn select_slots(&mut self, selection: Selection) -> impl Iterator<Item=Slot> {
        let chunk_sel = self.to_chunk_selection(selection);
        let chunk_iter = self.select_chunks(chunk_sel);
        SlotIterator::new(chunk_iter, selection)
    }

    pub(crate) fn raw_chunk_at(&mut self, chunk_coords: Coords) -> &mut RawChunk {
        self.chunks.entry(chunk_coords).or_insert(RawChunk::new())
    }

    fn select_chunks(&mut self, selection: Selection) -> ChunkIterator {
        ChunkIterator::new(self, selection)
    }

    fn to_chunk_selection(&self, global_selection: Selection) -> Selection {
        Selection {
            src: self.global_to_chunk_coords(&global_selection.src),
            dest: self.global_to_chunk_coords(&global_selection.dest)
        }
    }

    fn global_to_chunk_coords(&self, slot_coords: &Coords) -> Coords {
        Coords {
            x: Self::num_to_chunk_coord(slot_coords.x, self.chunk_size.width as i32),
            y: Self::num_to_chunk_coord(slot_coords.y, self.chunk_size.height as i32),
            z: Self::num_to_chunk_coord(slot_coords.z, self.chunk_size.depth as i32),
        }
    }

    fn num_to_chunk_coord(a: i32, chunk_size: i32) -> i32 {
        if a >= 0 {
            a / (chunk_size as i32)
        }
        else {
            (a-chunk_size+1) / chunk_size
        }
    }
}