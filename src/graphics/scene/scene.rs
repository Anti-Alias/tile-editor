use std::collections::HashMap;
use wgpu::{CommandEncoder, Device, Queue, SurfaceConfiguration, TextureView};
use crate::graphics::light::{LightBundle, LightMesh, LightSet, PointLight};
use crate::graphics::{Camera, Model, ModelInstance, ModelInstanceSet};
use crate::graphics::gbuffer::{GBuffer, ModelRenderer};
use crate::graphics::screen::{LightRenderer, PointLightDebugRenderer, PointLightRenderer, Screen};

type ModelHandle = u32;
type PointLightHandle = u32;
type DirectionalLightHandle = u32;
type AmbientLighthandle = u32;

#[derive(Copy, Clone, Debug, Default)]
pub struct LightConfig {
    pub max_point_lights: u32,
    pub max_directional_lights: u32,
    pub max_ambient_lights: u32
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PointLightDebugConfig {
    pub light_radius: f32
}

pub struct SceneConfig {
    pub light_config: LightConfig,
    pub surface_config: SurfaceConfiguration,
    pub point_light_debug_config: Option<PointLightDebugConfig>,
    pub camera: Camera
}

/// Represents a set
pub struct Scene {
    sequence: u32,                                                  // Sequence used for generating "handles"
    models: Vec<ModelInstanceSet>,                                  // All models with their respective instances
    model_handles: Vec<ModelHandle>,                                // Parallel array to `models`
    light_bundle: LightBundle,                                      // Ambient, directional and point lights in one bundle
    light_mesh: LightMesh,                                          // Mesh used to render light volumes during deferred rendering
    camera: Camera,                                                 // Camera of the scene
    gbuffer: GBuffer,                                               // GBuffer used for deferred rendering
    light_renderer: LightRenderer,                                  // Renders ambient and directional lights while sampling from gbuffer
    model_renderer: ModelRenderer,                                  // Renders models to the gbuffer
    point_light_renderer: PointLightRenderer,                       // Renders point lights while sampling from gbuffer
    point_light_debug_renderer: Option<PointLightDebugRenderer>     // Optional debug renderer that renders the point lights themselves. Helps figuring out where point lights are in space.
}

impl Scene {

    /// Creates a new scene
    fn new(device: &Device, config: SceneConfig) -> Self {
        // Unpacks
        let light_config = &config.light_config;
        let surface_config = &config.surface_config;

        // Creates "bindables"
        let light_bundle = LightBundle::new(
            device,
            LightSet::new(device, light_config.max_point_lights),
            LightSet::new(device, light_config.max_directional_lights),
            LightSet::new(device, light_config.max_ambient_lights)
        );
        let camera = config.camera;
        let gbuffer = GBuffer::new(device, surface_config.width, surface_config.height);

        // Gets layouts of "bindables"
        let gbuffer_bgl = gbuffer.bind_group_layout();
        let light_bundle_bgl = light_bundle.bind_group_layout();
        let camera_bgl = camera.bind_group_layout();

        // Creates renderers
        let light_renderer = LightRenderer::new(
            device,
            surface_config.format,
            gbuffer_bgl,
            light_bundle_bgl,
            camera_bgl
        );
        let model_renderer = ModelRenderer::new();
        let point_light_renderer = PointLightRenderer::new(
            device,
            surface_config.format,
            gbuffer_bgl,
            camera_bgl
        );
        let point_light_debug_renderer = config.point_light_debug_config.map(|config| {
            PointLightDebugRenderer::new(
                device,
                LightMesh::new(&device, 4, 8, 5.0),
                surface_config.format,
                GBuffer::DEPTH_STENCIL_FORMAT,
                camera_bgl
            )
        });
        let light_mesh = LightMesh::new(&device, 8, 16, 1.0);

        // Done
        Self {
            sequence: 0,
            models: Vec::new(),
            model_handles: Vec::new(),
            light_bundle,
            light_mesh,
            camera,
            gbuffer,
            light_renderer,
            model_renderer,
            point_light_renderer,
            point_light_debug_renderer
        }
    }

    /// Adds a model to the scene.
    /// Keep in mind that this is likely to result in a pipeline build and is considered expensive.
    /// Should be done upfront.
    pub fn add_model(&mut self, device: &Device, model: Model, max_instances: usize) -> ModelHandle {
        self.add_model_and_instances(device, ModelInstanceSet::new(device, model, max_instances))
    }

    /// Adds a model and its instances to the scene.
    /// Keep in mind that this is likely to result in a pipeline build and is considered expensive.
    /// Should be done upfront.
    pub fn add_model_and_instances(&mut self, device: &Device, instances: ModelInstanceSet) -> ModelHandle {
        self.model_renderer.prime(device, &instances.model, self.camera.bind_group_layout());
        self.models.push(instances);
        let handle = self.sequence;
        self.model_handles.push(handle);
        self.sequence += 1;
        handle
    }

    /// Removes a particular model.
    pub fn remove_model(&mut self, handle: ModelHandle) -> ModelInstanceSet {
        let index = self.model_handles.binary_search(&handle).expect("Could not find model with handle");
        self.model_handles.remove(index);
        self.models.remove(index)
    }

    /// Retrieves a view of all `ModelInstanceSet`s
    pub fn model_instances<'a>(&'a mut self, queue: &'a Queue, handle: ModelHandle) -> impl View<'a, ModelInstanceSet> {
        let index = self.model_handles.binary_search(&handle).expect("Could not find model with handle");
        let instance_set = &mut self.models[index];
        ModelView {
            resource: instance_set,
            queue
        }
    }

    /// Retrieves view of the scene's `LightBundle`.
    pub fn lights<'a>(&'a mut self, queue: &'a Queue) -> impl View<'a, LightBundle> {
        LightView {
            queue,
            resource: &mut self.light_bundle
        }
    }


    /// Renders this scene
    /// * `screen` - Screen to render to
    /// * `encoder` - CommandEncoder used to encode commands
    pub fn render(&mut self, screen: &Screen, encoder: &mut CommandEncoder) {

        // Renders models to gbuffer
        {
            let mut render_pass = self.gbuffer.begin_render_pass(encoder, true);
            for instance_set in &self.models {
                self.model_renderer.render(&mut render_pass, instance_set, &self.camera);
            }
        }

        // Renders lights to screen using gbuffer
        {
            let mut render_pass = screen.begin_render_pass(encoder);
            self.point_light_renderer.render(
                &mut render_pass,
                &self.gbuffer,
                &self.light_bundle.point_lights,
                &self.light_mesh,
                &self.camera
            );
            self.light_renderer.render(
                &mut render_pass,
                &self.gbuffer,
                &self.light_bundle,
                &self.camera
            );
        }
    }
}

/// Represents a view on some underlying resource.
/// When this view is "dropped", the underlying resource should be flushed.
pub trait View<'a, T>: Drop {
    fn resource(&'a mut self) -> &'a mut T;
}



// -------------------- Model view --------------------
struct ModelView<'a> {
    queue: &'a Queue,
    resource: &'a mut ModelInstanceSet
}
impl<'a> View<'a, ModelInstanceSet> for ModelView<'a> {
    fn resource(&'a mut self) -> &'a mut ModelInstanceSet {
        self.resource
    }
}
impl<'a> Drop for ModelView<'a> {
    fn drop(&mut self) {
        self.resource.flush(self.queue);
    }
}

// -------------------- Light view --------------------
struct LightView<'a> {
    queue: &'a Queue,
    resource: &'a mut LightBundle
}
impl<'a> View<'a, LightBundle> for LightView<'a> {
    fn resource(&'a mut self) -> &'a mut LightBundle {
        self.resource
    }
}
impl<'a> Drop for LightView<'a> {
    fn drop(&mut self) {
        self.resource.flush(self.queue);
    }
}