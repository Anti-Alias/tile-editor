use wgpu::*;
use crate::graphics::*;

/// Represents a set of meshes associated with materials
/// Meshes and materials can only be rendered if their indices are placed in the associations vector
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub associations: Vec<(usize, usize)>
}

impl Model {
    pub fn iter(&self) -> impl Iterator<Item=(&Mesh, &Material)> {
        self.associations.iter().map(move |association| {
            let mesh_idx = association.0;
            let mat_idx = association.1;
            (&self.meshes[mesh_idx], &self.materials[mat_idx])
        })
    }
}