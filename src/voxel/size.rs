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