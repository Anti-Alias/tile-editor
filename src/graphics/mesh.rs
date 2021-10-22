use wgpu::{Buffer};


/// Represents an indexed set of vertices
pub struct Mesh {
    /// Vertex data
    pub vertices: Buffer,

    /// Index data
    pub indices: Buffer,

    /// Color texture for mesh
    pub material: usize
}