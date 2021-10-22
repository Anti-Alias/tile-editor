use wgpu::{Buffer, VertexBufferLayout};
use std::rc::Rc;

/// Represents an indexed set of vertices
pub struct Mesh {

    /// Vertex data
    pub vertices: Buffer,

    /// Index data
    pub indices: Buffer,

    /// Color texture for mesh
    pub material: usize
}