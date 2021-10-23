use wgpu::{Buffer, IndexFormat};


/// Represents an indexed set of vertices
pub struct Mesh {
    pub vertices: Buffer,
    pub indices: Buffer,
    pub num_indices: u32,
    pub index_format: IndexFormat
}