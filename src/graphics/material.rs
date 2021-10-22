use crate::graphics::Texture;

pub struct Material {
    pub name: String,
    pub diffuse_texture: Option<Texture>,
    pub normal_texture: Option<Texture>
}

impl Material {
    fn permutation(&self) -> u64 {
        let mut result = 0;
        result |= self.diffuse_texture.is_some() as u64;
        result |= (self.normal_texture.is_some() as u64) << 1;
        result
    }
}