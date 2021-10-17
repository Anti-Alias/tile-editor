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

/// Unsigned coordinates
pub struct UCoords {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

impl UCoords {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}

/*
impl From<Coords> for Coords {
    fn from(coords: Coords) -> Self {
        Coords {
            x: coords.x,
            y: coords.y,
            z: coords.z
        }
    }
}
 */