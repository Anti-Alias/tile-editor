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