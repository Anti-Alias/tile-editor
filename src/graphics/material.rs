use crate::graphics::Texture;

/// A set of textures that determines how light interacts with a `Mesh`
pub struct Material {
    pub diffuse: Option<Texture>,
    pub specular: Option<Texture>,
    pub normal: Option<Texture>,
}

impl Material {
    pub const DIFFUSE_BIT: u64 = 1;
    pub const SPECULAR_BIT: u64 = 1 << 1;
    pub const NORMAL_BIT: u64 = 1 << 2;

    /// Creates an empty material with no textures
    pub fn new() -> Material {
        Material {
            diffuse: None,
            specular: None,
            normal: None
        }
    }

    pub fn with_diffuse(mut self, diffuse: Texture) -> Self {
        self.diffuse = Some(diffuse);
        self
    }

    pub fn with_specular(mut self, specular: Texture) -> Self {
        self.specular = Some(specular);
        self
    }

    pub fn with_normal(mut self, normal: Texture) -> Self {
        self.normal = Some(normal);
        self
    }

    /// Feature flags that should be set for this material
    pub fn flags(&self) -> u64 {
        let mut result = 0;
        result |= self.diffuse.is_some() as u64;
        result |= (self.specular.is_some() as u64) << 1;
        result |= (self.normal.is_some() as u64) << 2;
        result
    }
}