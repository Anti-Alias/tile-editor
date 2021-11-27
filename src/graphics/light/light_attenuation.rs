use egui_wgpu_backend::wgpu::{BindGroupLayoutEntry, BindingType};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindingResource, Buffer, BufferAddress, BufferBinding, BufferBindingType, BufferDescriptor, BufferUsages, Device, Queue, ShaderStages};

/// Light attenuation of a particular environment
pub struct LightAttenuation {
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout
}

impl LightAttenuation {

    pub fn new(device: &Device, constant: f32, linear: f32, quadratic: f32) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Light Attenuation Buffer"),
            size: std::mem::size_of::<[f32; 3]>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Light Attenuation Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
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
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Light Attenuation Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &buffer,
                        offset: 0,
                        size: None
                    })
                }
            ]
        });
        Self {
            constant,
            linear,
            quadratic,
            buffer,
            bind_group,
            bind_group_layout
        }
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

    /// Flushes updates to the GPU
    pub fn flush(&self, queue: &Queue) {
        let data = [self.constant, self.linear, self.quadratic];
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));
    }
}