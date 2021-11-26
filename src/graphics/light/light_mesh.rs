use wgpu::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use std::f32::consts::PI;

type Vertex = [f32; 3];

/// Simple light mesh
pub struct LightMesh {
    pub vertices: Buffer,
    pub indices: Buffer,
    pub num_indices: u32
}

impl LightMesh {

    /// Creates a new `LightVolume`.
    /// * `device` - Device that will allocate buffers
    /// * `horizontal_count` - Number of horizontal slices in the sphere. Must be at least 1. Excludes top and bottom points.
    /// * `vertical_count` - Number of vertical slices in the sphere. Must be at least 3.
    pub fn new(device: &Device, horizontal_count: u32, vertical_count: u32) -> Self {
        if horizontal_count == 0 { panic!("horizontal_count must be >= 1"); }
        if vertical_count < 3 { panic!("vertical_count must be >= 3"); }
        let (vdata, idata) = Self::create_mesh_data(horizontal_count, vertical_count);
        let vertices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vdata.as_slice()),
            usage: BufferUsages::VERTEX
        });
        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(idata.as_slice()),
            usage: BufferUsages::INDEX
        });
        LightMesh {
            vertices,
            indices,
            num_indices: idata.len() as u32
        }
    }

    // I meshed up...
    fn create_mesh_data(horizontal_count: u32, vertical_count: u32) -> (Vec<Vertex>, Vec<u32>) {

        // Allocates vecs
        let hv = horizontal_count * vertical_count;
        let num_vertices = 2 + hv;
        let num_indices = hv * 6;
        let mut vertices = Vec::with_capacity(num_vertices as usize);
        let mut indices = Vec::with_capacity(num_indices as usize);

        // Adds vertices
        let last_segment = (horizontal_count + 1) as f32; // Number of segments - 1 as a float
        vertices.push([0.0, -1.0, 0.0]);
        for r in 1..=horizontal_count {
            let rf = r as f32;
            let r_theta = PI * (rf / last_segment);
            let y = -r_theta.cos();
            let ring_radius = r_theta.sin();
            for v in 0..vertical_count {
                let h_ratio = v as f32 / vertical_count as f32;
                let h_radians = PI * 2.0 * h_ratio;
                let x = -h_radians.sin() * ring_radius;
                let z = h_radians.cos() * ring_radius;
                vertices.push([x, y, z]);
            }
        }
        vertices.push([0.0, 1.0, 0.0]);

        // Adds bottom indices
        for i in 0..vertical_count {
            indices.push(0);
            indices.push(i+1);
            indices.push((i+1) % vertical_count + 1);
        }

        // Adds middle indices
        for ring in 0..(horizontal_count-1) {
            for face in 0..vertical_count {
                let off = ring*vertical_count;
                let b = off + face + 1;
                let a = off + 1 + b % vertical_count;
                let c = b + vertical_count;
                let d = a + vertical_count;
                indices.push(a);
                indices.push(b);
                indices.push(c);
                indices.push(c);
                indices.push(d);
                indices.push(a);
            }
        }

        // Adds top indices
        let l = num_vertices-1;    // last
        let s = l-vertical_count;  // start
        for i in 0..vertical_count {
            indices.push(s+(i+1)%vertical_count);
            indices.push(s+i);
            indices.push(l);
        }
        (vertices, indices)
    }

    /// The WGPU memory layout of a `LightMesh`.
    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0
                }
            ]
        }
    }
}