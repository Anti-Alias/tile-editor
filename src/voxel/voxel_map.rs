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

    pub(crate) fn chunk_at(&mut self, chunk_coords: Coords) -> &mut RawChunk {
        self.chunks.entry(chunk_coords).or_insert(RawChunk::new())
    }

    pub(crate) fn select_chunks(&mut self, selection: Selection) -> impl Iterator<Item=Chunk> {
        self._select_chunks(selection)
    }

    pub fn select_slots(&mut self, selection: Selection) -> impl Iterator<Item=Slot> {
        let chunk_sel = self.to_chunk_selection(selection);
        let mut chunk_iter = self._select_chunks(chunk_sel);
        SlotIterator::new(chunk_iter, selection)
    }

    fn _select_chunks(&mut self, selection: Selection) -> ChunkIterator {
        ChunkIterator::new(self, selection)
    }

    fn to_chunk_selection(&self, global_selection: Selection) -> Selection {
        Selection {
            src: self.to_chunk_coords(global_selection.src),
            dest: self.to_chunk_coords(global_selection.dest)
        }
    }

    fn to_chunk_coords(&self, slot_coords: Coords) -> Coords {
        Coords {
            x: Self::to_chunk_coord(slot_coords.x, self.chunk_size.width as i32),
            y: Self::to_chunk_coord(slot_coords.y, self.chunk_size.height as i32),
            z: Self::to_chunk_coord(slot_coords.z, self.chunk_size.depth as i32),
        }
    }

    fn to_chunk_coord(a: i32, chunk_size: i32) -> i32 {
        if a >= 0 {
            a / (chunk_size as i32)
        }
        else {
            (a-chunk_size+1) / chunk_size
        }
    }
}