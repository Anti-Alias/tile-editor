#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ShaderFeatures {
    pub material_flags: u64 // See material.rs for flag bits
}
