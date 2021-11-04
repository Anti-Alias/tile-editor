use crate::graphics::Texture;

/// A set of textures that determines how light interacts with a `Mesh`
pub struct Material {
    pub diffuse_texture: Option<Texture>,
    pub normal_texture: Option<Texture>,
}

impl Material {
    pub const DIFFUSE_BIT: u64 = 1 << 0;
    pub const NORMAL_BIT: u64 = 1 << 1;

    /// Creates an empty material with no textures
    pub fn empty() -> Material {
        Material {
            diffuse_texture: None,
            normal_texture: None
        }
    }

    /// Feature flags that should be set for this material
    pub fn flags(&self) -> u64 {
        let mut result = 0;
        result |= self.diffuse_texture.is_some() as u64;
        result |= (self.normal_texture.is_some() as u64) << 1;
        result
    }
}