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

    const CUBE_INDICES: [u32; 6] = [
        0, 1, 2, 2, 3, 0,   // Near
//        4, 0, 3, 3, 7, 4,   // Left
//        1, 5, 6, 6, 2, 1,   // Right
//        4, 5, 1, 1, 0, 4,   // Bottom
//        3, 2, 6, 6, 7, 3,   // Top
//        5, 4, 7, 7, 6, 5,   // Far
    ];

    pub fn triangle(device: &Device, color: Color) -> Mesh {
        // Vertices (right-handed)
        let rgba = color.rgba();
        let v = &[
            ModelVertex {                               // bottom/left
                position: [-0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [0.0, 0.0]
            },
            ModelVertex {                               // bottom/right
                position: [0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [0.0, 0.0]
            },
            ModelVertex {                               // top/center
                position: [0.0, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [0.0, 0.0]
            }
        ];

        // Indices (Counter-clockwise)
        let i = &[0, 1, 2];

        // Creates vertex and index buffers
        let vertices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(v),
            usage: BufferUsages::VERTEX
        });
        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(i),
            usage: BufferUsages::INDEX
        });

        // Done
        Self {
            vertices,
            indices,
            num_indices: 3
        }
    }

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
    ) -> [ModelVertex; 4] {
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
                color: rgba,
                uv: [x1, y2]
            },
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [x2, y2]
            },
            ModelVertex {
                position: [0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [x2, y1]
            },
            ModelVertex {
                position: [-0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                color: rgba,
                uv: [x1, y1]
            }
        ];
        for vert in v.iter_mut() {
            vert.position[0] *= scale.x;
            vert.position[1] *= scale.y;
            vert.position[2] *= scale.z;
        }
        v
    }
}