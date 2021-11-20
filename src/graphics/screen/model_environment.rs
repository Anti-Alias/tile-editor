use crate::graphics::{Camera, DirectionalLight, ModelInstanceSet, PointLight};

/// Represents data needed for a `ModelRenderer` to render a set of model instances.
pub struct ModelEnvironment<'a> {

    /// Model instances to render
    pub instance_set: &'a ModelInstanceSet,

    /// Camera that determines our point-of-view
    pub camera: &'a Camera,

    /// All non-shadowed point lights used to illuminate the scene
    pub point_lights: &'a [PointLight],

    /// All non-shadow-casting directional lights used to illuminate the models
    pub directional_lights: &'a [DirectionalLight]
}