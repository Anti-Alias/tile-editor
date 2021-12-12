use wgpu::*;
use crate::graphics::light::{AmbientLight, DirectionalLight, LightSet, PointLight};

/// Represents a bundle of light sets of various types
pub struct LightBundle {
    pub point_lights: LightSet<PointLight>,
    pub directional_lights: LightSet<DirectionalLight>,
    pub ambient_lights: LightSet<AmbientLight>,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout
}

impl LightBundle {

    /// Creates a new LightBundle with existing LightSets
    pub fn new(
        device: &Device,
        point_lights: LightSet<PointLight>,
        directional_lights: LightSet<DirectionalLight>,
        ambient_lights: LightSet<AmbientLight>,
    ) -> Self {
        let (bind_group, bind_group_layout) = Self::create_bind_group(
            device,
            &point_lights.buffer,
            &directional_lights.buffer,
            &ambient_lights.buffer
        );
        LightBundle {
            point_lights,
            directional_lights,
            ambient_lights,
            bind_group,
            bind_group_layout
        }
    }

    /// Creates a new LightBundle while allocating new light sets
    pub fn create(
        device: &Device,
        max_point_lights: u32,
        max_directional_lights: u32,
        max_ambient_lights: u32
    ) -> Self {
        let point_lights = LightSet::new(device, max_point_lights);
        let directional_lights = LightSet::new(device, max_directional_lights);
        let ambient_lights = LightSet::new(device, max_ambient_lights);
        Self::new(device, point_lights, directional_lights, ambient_lights)
    }

    pub fn flush(&self, queue: &Queue) {
        self.point_lights.flush(queue);
        self.directional_lights.flush(queue);
        self.ambient_lights.flush(queue);
    }

    pub fn bind_group(&self) -> &BindGroup { &self.bind_group }

    pub fn bind_group_layout(&self) -> &BindGroupLayout { &self.bind_group_layout }

    fn create_bind_group(
        device: &Device,
        point_light_buffer: &Buffer,
        directional_light_buffer: &Buffer,
        ambient_light_buffer: &Buffer
    ) -> (BindGroup, BindGroupLayout) {
        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Light Bundle Bind Group Layout"),
            entries: &[
                // Point lights
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                },
                // Directional lights
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                },
                // Ambient lights
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ]
        });
        let group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Light Bundle Bind Group"),
            layout: &layout,
            entries: &[
                // Point lights
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: point_light_buffer,
                        offset: 0,
                        size: None
                    })
                },
                // Directional lights
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: directional_light_buffer,
                        offset: 0,
                        size: None
                    })
                },
                // Point lights
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: ambient_light_buffer,
                        offset: 0,
                        size: None
                    })
                }
            ]
        });
        (group, layout)
    }
}