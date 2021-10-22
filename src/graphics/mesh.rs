use wgpu::{Buffer, VertexBufferLayout};
use std::rc::Rc;

/// Represents an indexed set of vertices
struct Mesh {

    /// Vertex data
    pub vertices: Buffer,

    /// Index data
    pub indices: Buffer,

    /// Color texture for mesh
    pub material: usize
}

trait Vertex {
    fn layout() -> VertexBufferLayout<'static>;
}