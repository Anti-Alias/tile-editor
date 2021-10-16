use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use crate::voxel::Voxel;

/// A model struct storing voxel data
pub struct VoxelMap {
    chunk_size: Size,
    chunks: HashMap<Coords, RawChunk>
}

impl VoxelMap {
    pub fn new(chunk_size: Size) -> Self {
        Self {
            chunk_size,
            chunks: HashMap::new()
        }
    }

    pub fn chunk_at(&mut self, chunk_coords: Coords) -> &mut RawChunk {
        self.chunks.entry(chunk_coords).or_insert(RawChunk::new())
    }

    pub fn select_chunks(&mut self, selection: Selection) -> impl Iterator<Item=Chunk> {
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


// -------------------------------------------------------------------------
/// Represents a slot in a `VoxelMap` which can store 0 or more `Voxel`s
#[derive(Debug)]
pub struct Slot<'map> {
    relative_coords: Coords,
    chunk: Chunk<'map>
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


// -------------------------------------------------------------------------
/// Raw Chunk in a `VoxelMap`.
/// Does not store position or size information.
pub struct RawChunk {
    layers: Vec<Layer>
}

impl RawChunk {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
}

/// Wraps a `RawChunk` and includes position and size information.
pub struct Chunk<'map> {
    coords: Coords,             // Offset of chunk in chunk-coords (not global).
    size: Size,                 // Size of chunk in voxels
    raw: &'map mut RawChunk     // Inner chunk data
}

impl<'map> Debug for Chunk<'map> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Chunk {{ coords: {:?}, size: {:?} }}", self.coords, self.size);
        Result::Ok(())
    }
}

impl<'map> Chunk<'map> {

    fn slot_at(&mut self, coords: Coords) -> Slot<'map> {
        let (x, y, z) = (self.coords.z, self.coords.y, self.coords.z);
        let (w, h, d) = (self.size.width as i32, self.size.height as i32, self.size.depth as i32);
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
    fn selection(&self) -> Selection {
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


// -------------------------------------------------------------------------
/// Layer in a `RawChunk`
struct Layer(Vec<Voxel>);


// -------------------------------------------------------------------------
/// Iterator over `Chunk`s in a `VoxelMap`
pub struct ChunkIterator<'map> {
    vmap: &'map mut VoxelMap,
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


// -------------------------------------------------------------------------
/// Iterator over `Slot`s in a `VoxelMap`
pub struct SlotIterator<'map> {
    chunk_iter: ChunkIterator<'map>,    // Iterator over chunks w/ meta
    global_selection: Selection,        // Selection over all slots in global coords
    current_chunk: Chunk<'map>,         // Current chunk w/ meta we're iterating on
    relative_selection: Selection,      // Selection of slots in local coords (local to current chunk)
    current: Coords                     // Current coords of local_selection. Should reset when chunk changes
}

impl<'map> SlotIterator<'map> {
    fn new(mut chunk_iter: ChunkIterator<'map>, global_selection: Selection) -> Self {
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


// -------------------------------------------------------------------------
/// Selection in a 3D voxel space
/// Values of `src` and `dest` should be considered inclusive when iterating
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    src: Coords,
    dest: Coords
}

impl Selection {

    pub fn new(sx: i32, sy: i32, sz: i32, dx: i32, dy: i32, dz: i32) -> Self {
        Self {
            src: Coords::new(sx, sy, sz),
            dest: Coords::new(dx, dy, dz)
        }
    }

    /// Source position of selection
    pub fn src(&self) -> Coords { return self.src }

    /// Destination position of selection
    pub fn dest(&self) -> Coords { return self.dest }

    /// Intersection of this selection and another
    pub fn intersect(&self, rhs: &Selection) -> Selection {
        Selection {
            src: Coords {
                x: max(self.src.x, rhs.src.x),
                y: max(self.src.y, rhs.src.y),
                z: max(self.src.z, rhs.src.z)
            },
            dest: Coords {
                x: min(self.dest.x, rhs.dest.x),
                y: min(self.dest.y, rhs.dest.y),
                z: min(self.dest.z, rhs.dest.z)
            }
        }
    }

    fn min_max(a: i32, b: i32) -> (i32, i32) {
        if a > b { (b, a) } else { (a, b)}
    }
}


// -------------------------------------------------------------------------
/// Coordinates for `Voxel`s or `Chunk`s
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Coords {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}


// -------------------------------------------------------------------------
/// Size of something
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

impl Size {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Size { width, height, depth }
    }
}