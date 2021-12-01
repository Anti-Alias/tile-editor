use cgmath::{Vector3, Matrix4, Perspective, SquareMatrix, Ortho, Point3, PerspectiveFov};
use wgpu::*;
use bytemuck::{Pod, Zeroable};


/// Represents a camera.
/// May be either perspective or orthographic depending on how the projection matrix is configured.
pub struct Camera {
    eye: Point3<f32>,                       // Position of the eye (origin) of the camera
    direction: Vector3<f32>,                // Direction the camera is looking
    up: Vector3<f32>,                       // Orientation of the camera. Usually set to (0, 1, 0)
    projection: Matrix4<f32>,               // Projection matrix. Can be manipulated to give an orthographic or perspective look.
    coordinate_system: Matrix4<f32>,        // Matrix that transforms geometry from a foreign coordinate system to WGPU's coordinate system.
    buffer: Buffer,                         // Buffer that stores data
    bind_group: BindGroup,                  // Bind group for that data
    bind_group_layout: BindGroupLayout,     // Layout of that data
}


impl Camera {

    #[rustfmt::skip]
    pub const WGPU_COORDINATE_SYSTEM: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    #[rustfmt::skip]
    pub const OPENGL_COORDINATE_SYSTEM: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    /// Creates a camera with a custom projection
    pub fn create(
        device: &Device,
        eye: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
        projection: Matrix4<f32>
    ) -> Self {
        let mut cam = Self::_create(device, eye, direction, up);
        cam.projection = projection;
        cam
    }

    /// Creates an orthographic camera
    pub fn create_ortho(
        device: &Device,
        eye: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
        ortho: Ortho<f32>
    ) -> Self {
        let mut cam = Self::_create(device, eye, direction, up);
        cam.set_ortho(ortho);
        cam
    }

    /// Creates a perspective camera
    pub fn create_perspective(
        device: &Device,
        eye: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
        perspective: Perspective<f32>
    ) -> Self {
        let mut cam = Self::_create(device, eye, direction, up);
        cam.set_perspective(perspective);
        cam
    }

    /// Creates a perspective camera
    pub fn create_perspective_fov(
        device: &Device,
        eye: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
        fov: PerspectiveFov<f32>
    ) -> Self {
        let mut cam = Self::_create(device, eye, direction, up);
        cam.set_perspective_fov(fov);
        cam
    }

    /// Creates a right-handed camera using a `Device` to allocate buffers
    fn _create(
        device: &Device,
        eye: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>
    ) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Projection View Buffer"),
            size: std::mem::size_of::<RawData>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }]
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding()
            }]
        });
        Self {
            eye,
            direction,
            up,
            projection: Matrix4::identity(),
            coordinate_system: Matrix4::identity(),
            buffer,
            bind_group,
            bind_group_layout
        }
    }

    pub fn move_to(&mut self, eye: Point3<f32>) {
        self.eye = eye;
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.eye += translation;
    }

    pub fn look_at(&mut self, point: Point3<f32>) {
        self.direction = point - self.eye;
    }

    pub fn look_to(&mut self, direction: Vector3<f32>) {
        self.direction = direction;
    }

    pub fn set_up(&mut self, up: Vector3<f32>) {
        self.up = up;
    }

    /// Sets projection matrix to a perspective matrix
    pub fn set_perspective(&mut self, perspective: Perspective<f32>) {
        self.projection = perspective.into();
    }

    /// Sets projection matrix to a perspective matrix
    pub fn set_perspective_fov(&mut self, fov: PerspectiveFov<f32>) {
        self.projection = fov.into();
    }

    /// Sets projection matrix to an orthographic matrix
    pub fn set_ortho(&mut self, ortho: Ortho<f32>) {
        self.projection = ortho.into();
    }

    /// Sets expected coordinate system of the geometry being rendered
    pub fn set_coordinate_system(&mut self, coordinate_system: Matrix4<f32>) {
        self.coordinate_system = coordinate_system;
    }

    /// Writes camera's state to it's WGPU buffer.
    /// Only need to be invoked on the camera's state changing.
    pub fn flush(&self, queue: &Queue) {
        let view = Matrix4::look_to_rh(self.eye, self.direction, self.up);
        let proj_view = self.coordinate_system * self.projection * view;
        let raw_data = RawData {
            eye: self.eye.into(),
            pad: 0,
            proj_view: proj_view.into()
        };
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&raw_data));
    }

    /// Reference to projection-view buffer.
    /// This buffer is written to on invocations of 'write'.
    /// To be used in rendering pipelines.
    pub fn projection_view_buffer(&self) -> &Buffer {
        self.projection_view_buffer()
    }

    /// Underlying WGPU buffer
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// WGPU bind group for this camera
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    /// Layout of the WGPU bind group
    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct RawData {
    pub eye: [f32; 3],
    pub pad: u32,
    pub proj_view: [[f32; 4]; 4]
}