use std::ops::Index;
use wgpu::*;

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix, Matrix3, Matrix4, SquareMatrix, Transform, Vector3};
use crate::graphics::Model;

/// Represents the instance data of a `Model`.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ModelInstance {
    pub world: [[f32; 4]; 4],
    normal: [[f32; 3]; 3]
}

impl ModelInstance {

    pub fn new(world: Matrix4<f32>) -> Self {
        let mut result = Self {
            world: world.into(),
            normal: Matrix3::identity().into()
        };
        result.compute_normal();
        result
    }

    pub fn compute_normal(&mut self) {
        let world3 = Matrix3::new(
            self.world[0][0], self.world[0][1], self.world[0][2],
            self.world[1][0], self.world[1][1], self.world[1][2],
            self.world[2][0], self.world[2][1], self.world[2][2]
        );
        self.normal = world3.invert().unwrap().transpose().into();
    }

    /// The WGPU memory layout of a buffer storing a `ModelInstance`
    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelInstance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                // Model matrix
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 6
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 7
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 8
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 9
                },

                // Normal matrix
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 16]>() as BufferAddress,
                    shader_location: 10
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 19]>() as BufferAddress,
                    shader_location: 11
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 22]>() as BufferAddress,
                    shader_location: 12
                }
            ]
        }
    }
}

/// Represents a `Model` and a set of instances.
pub struct ModelInstanceSet {
    pub model: Model,
    instances: Vec<ModelInstance>,
    buffer: Buffer
}

impl ModelInstanceSet {

    /// Creates a new set
    /// * `device` - Device used to allocate buffer that houses model instances on the GPU
    /// * `model` - Main model to render
    /// * `max_instances` The maximum number of instances allowed
    pub fn new(device: &Device, model: Model, max_instances: usize) -> ModelInstanceSet {
        let instances = Vec::with_capacity(max_instances as usize);
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model Instance Buffer"),
            size: (std::mem::size_of::<ModelInstance>() * max_instances) as BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        ModelInstanceSet {
            model,
            instances,
            buffer
        }
    }

    /// Adds a `ModelInstance`
    pub fn push(&mut self, instance: ModelInstance) -> &mut Self {
        self.check_capacity();
        self.instances.push(instance);
        self
    }

    /// Adds a `ModelInstance` at the specified index
    pub fn insert(&mut self, index: usize, instance: ModelInstance) -> &mut Self {
        self.check_capacity();
        self.instances.insert(index, instance);
        self
    }

    /// Removes a `ModelInstance` at the specified index
    pub fn remove(&mut self, index: usize) -> ModelInstance {
        self.instances.swap_remove(index)
    }

    /// Number of instances stored
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=&ModelInstance> {
        self.instances.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut ModelInstance> {
        self.instances.iter_mut()
    }

    /// Maximum number of instances that can be stored
    pub fn capacity(&self) -> usize {
        self.instances.capacity()
    }

    /// Buffer slice of all data
    pub fn buffer_slice(&self) -> BufferSlice {
        let end = (std::mem::size_of::<ModelInstance>() * self.instances.capacity()) as BufferAddress;
        self.buffer.slice(0..end)
    }

    pub fn flush(&self, queue: &Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(self.instances.as_slice()));
    }

    fn check_capacity(&self) {
        if self.len() == self.capacity() {
            panic!("ModelInstanceSet reached max capacity {}. Cannot exceed.", self.capacity());
        }
    }
}

impl Index<usize> for ModelInstanceSet {
    type Output = ModelInstance;
    fn index(&self, index: usize) -> &Self::Output {
        &self.instances[index]
    }
}