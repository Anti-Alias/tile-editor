use cgmath::Vector3;
use wgpu::{Buffer, BufferUsages, Device};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::graphics::{ModelVertex, Color};


/// Represents an indexed set of vertices
pub struct Mesh {
    pub vertices: Buffer,
    pub indices: Buffer,
    pub num_indices: u32
}

impl Mesh {

    const CUBE_INDICES: [u32; 36] = [
        0,1,2,2,3,0,        // NEAR
        4,5,6,6,7,4,        // LEFT
        8,9,10,10,11,8,     // RIGHT
        12,13,14,14,15,12,  // BOTTOM
        16,17,18,18,19,16,  // TOP
        20,21,22,22,23,20,  // FAR
    ];

    pub fn cube(device: &Device, color: Color, scale: Vector3<f32>) -> Mesh {
        let v = Self::create_cube_vertices(scale, color.rgba());
        let vertices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&v),
            usage: BufferUsages::VERTEX
        });
        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&Self::CUBE_INDICES),
            usage: BufferUsages::INDEX
        });

        // Done
        Self {
            vertices,
            indices,
            num_indices: Self::CUBE_INDICES.len() as u32
        }
    }

    fn create_cube_vertices(
        scale: Vector3<f32>,
        rgba: [f32; 4]
    ) -> [ModelVertex; 24] {
        let x0: f32 = 0.0;
        let x1: f32 = 1.0/4.0;
        let x2: f32 = 2.0/4.0;
        let x3: f32 = 3.0/4.0;
        let x4: f32 = 4.0/4.0;
        let y0: f32 = 0.0;
        let y1: f32 = 1.0/3.0;
        let y2: f32 = 2.0/3.0;
        let y3: f32 = 3.0/3.0;
        let mut v = [
            // NEAR
            ModelVertex {
                position: [-0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x1, y2]
            },
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x2, y2]
            },
            ModelVertex {
                position: [0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x2, y1]
            },
            ModelVertex {
                position: [-0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x1, y1]
            },

            // LEFT
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                normal: [-1.0, 0.0, 0.0],
                tangent: [0.0, 0.0, 1.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x0, y2]
            },
            ModelVertex {
                position: [-0.5, -0.5, 0.5],
                normal: [-1.0, 0.0, 0.0],
                tangent: [0.0, 0.0, 1.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x1, y2]
            },
            ModelVertex {
                position: [-0.5, 0.5, 0.5],
                normal: [-1.0, 0.0, 0.0],
                tangent: [0.0, 0.0, 1.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x1, y1]
            },
            ModelVertex {
                position: [-0.5, 0.5, -0.5],
                normal: [-1.0, 0.0, 0.0],
                tangent: [0.0, 0.0, 1.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x0, y1]
            },

            // RIGHT
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                normal: [1.0, 0.0, 0.0],
                tangent: [0.0, 0.0, -1.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x2, y2]
            },
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                normal: [1.0, 0.0, 0.0],
                tangent: [0.0, 0.0, -1.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x3, y2]
            },
            ModelVertex {
                position: [0.5, 0.5, -0.5],
                normal: [1.0, 0.0, 0.0],
                tangent: [0.0, 0.0, -1.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x3, y1]
            },
            ModelVertex {
                position: [0.5, 0.5, 0.5],
                normal: [1.0, 0.0, 0.0],
                tangent: [0.0, 0.0, -1.0],
                bitangent: [0.0, 1.0, 0.0],
                color: rgba,
                uv: [x2, y1]
            },

            // BOTTOM
            ModelVertex {
                position: [-0.5, -0.5, 0.5],
                normal: [0.0, -1.0, 0.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [x1, y2]
            },
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                normal: [0.0, -1.0, 0.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [x1, y3]
            },
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                normal: [0.0, -1.0, 0.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [x2, y3]
            },
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                normal: [0.0, -1.0, 0.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [x2, y2]
            },

            // TOP
            ModelVertex {
                position: [-0.5, 0.5, 0.5],
                normal: [0.0, 1.0, 0.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, -1.0],
                color: rgba,
                uv: [x1, y1]
            },
            ModelVertex {
                position: [0.5, 0.5, 0.5],
                normal: [0.0, 1.0, 0.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, -1.0],
                color: rgba,
                uv: [x2, y1]
            },
            ModelVertex {
                position: [0.5, 0.5, -0.5],
                normal: [0.0, 1.0, 0.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, -1.0],
                color: rgba,
                uv: [x2, y0]
            },
            ModelVertex {
                position: [-0.5, 0.5, -0.5],
                normal: [0.0, 1.0, 0.0],
                tangent: [1.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, -1.0],
                color: rgba,
                uv: [x1, y0]
            },

            // FAR
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
                tangent: [-1.0, 0.0, 0.0],
                bitangent: [0.0, 1.0, 1.0],
                color: rgba,
                uv: [x3, y2]
            },
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
                tangent: [-1.0, 0.0, 0.0],
                bitangent: [0.0, 1.0, 1.0],
                color: rgba,
                uv: [x4, y2]
            },
            ModelVertex {
                position: [-0.5, 0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
                tangent: [-1.0, 0.0, 0.0],
                bitangent: [0.0, 1.0, 1.0],
                color: rgba,
                uv: [x4, y1]
            },
            ModelVertex {
                position: [0.5, 0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
                tangent: [-1.0, 0.0, 0.0],
                bitangent: [0.0, 1.0, 1.0],
                color: rgba,
                uv: [x3, y1]
            },
        ];

        for vert in v.iter_mut() {
            vert.position[0] *= scale.x;
            vert.position[1] *= scale.y;
            vert.position[2] *= scale.z;
        }
        v
    }
}