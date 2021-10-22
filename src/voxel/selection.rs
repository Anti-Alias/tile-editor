use std::cmp::{max, min};
use crate::voxel::Coords;

/// Selection in a 3D voxel space
/// Values of `src` and `dest` should be considered inclusive when iterating
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    pub src: Coords,
    pub dest: Coords
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
}