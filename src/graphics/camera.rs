use cgmath::{Vector3, Matrix4, Perspective, SquareMatrix, Ortho, Point3, EuclideanSpace, PerspectiveFov};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferAddress, BufferBindingType, BufferDescriptor, BufferUsages, Device, Queue, ShaderStages};
use wgpu::util::DeviceExt;


/// Represents a camera
pub struct Camera {
    eye: Point3<f32>,                       // Position of the eye (origin) of the camera
    direction: Vector3<f32>,                // Direction the camera is looking
    up: Vector3<f32>,                       // Orientation of the camera. Usually set to (0, 1, 0)
    projection: Matrix4<f32>,               // Projection matrix. Can be manipulated to give an orthographic or perspective look.
    coordinate_system: Matrix4<f32>,        // Matrix that transforms geometry from a foreign coordinate system to WGPU's coordinate system.
    buffer: Buffer,                         // Buffer that stores data
    bind_group: BindGroup,                  // Bind group for that data
    bind_group_layout: BindGroupLayout,     // Layout of that data
    changed: bool                           // Caching flag
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
            size: std::mem::size_of::<[f32; 16]>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
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
            bind_group_layout,
            changed: true
        }
    }

    pub fn move_to(&mut self, eye: Point3<f32>) {
        self.eye = eye;
        self.changed = true;
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.eye += translation;
        self.changed = true;
    }

    pub fn look_at(&mut self, point: Vector3<f32>) {
        self.direction = point - self.eye.to_vec();
        self.changed = true;
    }

    pub fn look_to(&mut self, direction: Vector3<f32>) {
        self.direction = direction;
        self.changed = true;
    }

    pub fn set_up(&mut self, up: Vector3<f32>) {
        self.up = up;
        self.changed = true;
    }

    /// Sets projection matrix to perspective matrix
    pub fn set_perspective(&mut self, perspective: Perspective<f32>) {
        self.projection = perspective.into();
        self.changed = true;
    }

    /// Sets projection matrix to perspective matrix
    pub fn set_perspective_fov(&mut self, fov: PerspectiveFov<f32>) {
        self.projection = fov.into();
        self.changed = true;
    }

    /// Sets projection matrix to orthographic matrix
    pub fn set_ortho(&mut self, ortho: Ortho<f32>) {
        self.projection = ortho.into();
        self.changed = true;
    }

    /// Sets expected coordinate system of geometry
    pub fn set_coordinate_system(&mut self, coordinate_system: Matrix4<f32>) {
        self.coordinate_system = coordinate_system;
    }

    /// Writes to internal
    pub fn write(&mut self, queue: &Queue) {
        if self.changed {
            let view = Matrix4::look_to_rh(self.eye, self.direction, self.up);
            let mut proj_view = self.coordinate_system * self.projection * view;
            let proj_view: [[f32; 4]; 4] = proj_view.into();
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[proj_view]));
            self.changed = false;
        }
    }

    /// Reference to projection-view buffer.
    /// This buffer is written to on invocations of 'write'.
    /// To be used in rendering pipelines.
    pub fn projection_view_buffer(&self) -> &Buffer {
        self.projection_view_buffer()
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}