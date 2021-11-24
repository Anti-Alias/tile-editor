use crate::graphics::light::{LightSet, PointLight, DirectionalLight};

pub struct LightBundle {
    pub point_lights: LightSet<PointLight>,
    pub directional_lights: LightSet<DirectionalLight>
}

impl LightBundle {
    pub fn new() -> LightBundle {
        todo!()
    }
}