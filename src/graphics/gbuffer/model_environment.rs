use crate::graphics::{Camera, ModelInstanceSet};
use crate::graphics::light::{LightBundle};

/// Represents data needed for a `ModelRenderer` to render a set of model instances.
pub struct ModelEnvironment<'a> {

    /// Model instances to render
    pub instance_set: &'a ModelInstanceSet,

    /// A bundle of lights
    pub light_bundle: &'a LightBundle,

    /// Camera that determines our point-of-view
    pub camera: &'a Camera
}