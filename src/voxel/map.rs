use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::Infallible;
use crate::voxel::Voxel;

pub struct VoxelMap {
    chunk_width: u32,
    chunk_height: u32,
    chunk_depth: u32,
    chunks: HashMap<Coords, Chunk>
}

impl VoxelMap {
    pub fn new(chunk_width: u32, chunk_height: u32, chunk_depth: u32) -> Self {
        Self {
            chunk_width,
            chunk_height,
            chunk_depth,
            chunks: HashMap::new()
        }
    }

    pub fn chunk_at(&mut self, chunk_coords: Coords) -> &mut Chunk {
        self.chunks.entry(chunk_coords).or_insert(Chunk::new())
    }

    pub fn select_chunks(&mut self, selection: Selection) -> impl Iterator<Item=&mut Chunk> {
        ChunkIterator::new(self, selection)
    }

    fn to_chunk_coords(&self, slot_coords: Coords) -> Coords {
        Coords {
            x: Self::to_chunk_coord(slot_coords.x, self.chunk_width as i32),
            y: Self::to_chunk_coord(slot_coords.y, self.chunk_height as i32),
            z: Self::to_chunk_coord(slot_coords.z, self.chunk_depth as i32),
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

/// Represents a cell in a `VoxelMap` which can store 0 or more `Voxel`s
pub struct Cell<'map> {
    chunk_coords: Coords,
    local_coords: Coords,
    chunk: &'map mut Chunk
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct Coords {
    x: i32,
    y: i32,
    z: i32
}

impl Coords {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    src: Coords,
    dest: Coords
}

/// Selection in a 3D voxel space
/// Values of `src` and `dest` should be considered inclusive when iterating
impl Selection {

    pub fn new(sx: i32, sy: i32, sz: i32, dx: i32, dy: i32, dz: i32) -> Self {
        // Ensures that source values are less than dest values
        let (sx, dx) = Self::min_max(sx, dx);
        let (sy, dy) = Self::min_max(sy, dy);
        let (sz, dz) = Self::min_max(sz, dz);

        // Done
        Self {
            src: Coords::new(sx, sy, sz),
            dest: Coords::new(dx, dy, dz)
        }
    }

    /// Source position of selection
    pub fn src(&self) -> Coords { return self.src }

    /// Destination position of selection
    pub fn dest(&self) -> Coords { return self.dest }

    fn min_max(a: i32, b: i32) -> (i32, i32) {
        if a > b { (b, a) } else { (a, b)}
    }
}

struct Layer(Vec<Voxel>);

pub struct Chunk { layers: Vec<Layer> }
impl Chunk {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
}

pub struct ChunkIterator<'map> {
    map: &'map mut VoxelMap,
    selection: Selection,
    current: Coords
}

impl<'map> ChunkIterator<'map> {
    pub fn new(map: &'map mut VoxelMap, selection: Selection) -> Self {
        Self { map, selection, current: selection.src }
    }
}

impl<'map> Iterator for ChunkIterator<'map> {
    type Item = &'map mut Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let src = self.selection.src;
        let dest = self.selection.dest;
        if self.current.z > dest.z {
            None
        }
        else {
            unsafe {

                // Gets chunk
                let chunk = self.map.chunk_at(self.current) as *mut Chunk;

                // Updates current position
                self.current.x += 1;
                if self.current.x > dest.x {
                    self.current.x = src.x;
                    self.current.y += 1;
                    if self.current.y > dest.y {
                        self.current.y = src.y;
                        self.current.z += 1;
                    }
                }

                // Return chunk
                let chunk = (&mut *chunk) as &'map mut Chunk;
                Some(chunk)
            }
        }
    }
}