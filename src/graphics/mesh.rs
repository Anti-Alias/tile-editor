use wgpu::{Buffer, BufferUsages, Device, IndexFormat};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::graphics::{ModelVertex, RGBA, Vector2, Vector3};


/// Represents an indexed set of vertices
pub struct Mesh {
    pub vertices: Buffer,
    pub indices: Buffer,
    pub num_indices: u32
}

impl Mesh {
    fn cube(device: &Device, color: RGBA) -> Mesh {
        // Vertices (right-handed)
        let v = &[
            ModelVertex {                               // bottom/left/near
                position: Vector3::new(-0.5, -0.5, 0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
                color,
                uv: Vector2::new(0.0, 0.0)
            },
            ModelVertex {                               // bottom/right/near
                position: Vector3::new(0.5, -0.5, 0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
                color,
                uv: Vector2::new(0.0, 0.0)
            },
            ModelVertex {                               // top/right/near
                position: Vector3::new(0.5, 0.5, 0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
                color,
                uv: Vector2::new(0.0, 0.0)
            },
            ModelVertex {                               // top/left/near
                position: Vector3::new(-0.5, 0.5, 0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
                color,
                uv: Vector2::new(0.0, 0.0)
            },
            ModelVertex {                               // bottom/left/far
                position: Vector3::new(-0.5, -0.5, -0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
                color,
                uv: Vector2::new(0.0, 0.0)
            },
            ModelVertex {                               // bottom/right/far
                position: Vector3::new(0.5, -0.5, -0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
                color,
                uv: Vector2::new(0.0, 0.0)
            },
            ModelVertex {                               // top/right/far
                position: Vector3::new(0.5, 0.5, -0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
                color,
                uv: Vector2::new(0.0, 0.0)
            },
            ModelVertex {                               // top/left/far
                position: Vector3::new(-0.5, 0.5, -0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
                color,
                uv: Vector2::new(0.0, 0.0)
            },
        ];

        // Indices (Counter-clockwise)
        let i = &[
            0, 1, 2, 2, 3, 0,   // Near
            4, 0, 3, 3, 7, 4,   // Left
            1, 5, 6, 6, 2, 1,   // Right
            4, 5, 1, 1, 0, 4,   // Bottom
            3, 2, 6, 6, 7, 3,   // Top
            5, 4, 7, 7, 6, 5,   // Far
        ];

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
            num_indices: 36
        }
    }
}