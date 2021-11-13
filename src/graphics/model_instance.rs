use wgpu::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use bytemuck::{Pod, Zeroable};
use crate::graphics::Model;

/// Represents the instance data of a `Model`.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ModelInstance {
    pub world: [[f32; 4]; 4]
}

impl ModelInstance {

    /// The WGPU memory layout of a buffer storing a `ModelInstance`
    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelInstance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 4
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 5
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 6
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 7
                }
            ]
        }
    }
}

/// Represents a `Model` and a set of instances.
pub struct ModelInstanceSet {
    pub model: Model,
    pub instances: Vec<ModelInstance>,
    pub buffer: Buffer
}

impl ModelInstanceSet {

    /// Creates a new set
    /// * `device` - Device used to allocate buffer that houses model instances on the GPU
    /// * `model` - Main model to render
    /// * `instances` - Initial set of instances of said model to render
    pub fn new(device: &Device, model: Model, instances: Vec<ModelInstance>) -> ModelInstanceSet {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&instances),
            usage: BufferUsages::VERTEX
        });
        ModelInstanceSet {
            model,
            instances,
            buffer
        }
    }

    /// Number of instances stored
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// Buffer slice of all data
    pub fn buffer_slice(&self) -> BufferSlice {
        self.buffer.slice(..)
    }

    /// All instance data currently stored
    pub fn instances(&self) -> &[ModelInstance] {
        &self.instances[..]
    }
}