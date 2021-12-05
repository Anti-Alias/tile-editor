use std::collections::HashMap;
use wgpu::{Device, Queue, SurfaceConfiguration};
use crate::graphics::light::{LightBundle, LightSet, PointLight};
use crate::graphics::{Camera, Model, ModelInstance, ModelInstanceSet, ModelView};
use crate::graphics::gbuffer::{GBuffer, ModelRenderer};
use crate::graphics::screen::{LightRenderer, PointLightDebugRenderer, PointLightRenderer};

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
    models: Vec<ModelInstanceSet>,
    light_bundle: LightBundle,
    camera: Camera,
    gbuffer: GBuffer,
    light_renderer: LightRenderer,
    model_renderer: ModelRenderer,
    point_light_renderer: PointLightRenderer,
    point_light_debug_renderer: Option<PointLightDebugRenderer>
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
                config.light_radius,
                surface_config.format,
                GBuffer::DEPTH_STENCIL_FORMAT,
                camera_bgl
            )
        });

        // Done
        Self {
            models: Vec::new(),
            light_bundle,
            camera,
            gbuffer,
            light_renderer,
            model_renderer,
            point_light_renderer,
            point_light_debug_renderer
        }
    }

    pub fn model_view<'a, 'b>(&'a mut self, index: usize, queue: &'b Queue) -> ModelView<'a, 'b> {
        ModelView {
            instances: &mut self.models[index],
            queue
        }
    }

    pub fn light_view<'a, 'b>(&'a mut self, queue: &'b Queue) -> LightView<'a, 'b> {
        todo!()
    }
}