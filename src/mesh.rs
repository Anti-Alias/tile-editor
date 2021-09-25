use wgpu::{Buffer, VertexBufferLayout};
use std::rc::Rc;
use crate::Texture;

/// Represents an indexed set of vertices
#[derive(Clone)]
struct Mesh {

    /// Vertex data
    pub vertices: Rc<Buffer>,

    /// Index data
    pub indices: Rc<Buffer>,

    /// Color texture for mesh
    pub color_tex: Texture
}

impl Mesh {
}

trait Vertex {
    fn layout() -> VertexBufferLayout<'static>;
}