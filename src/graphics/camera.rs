use cgmath::{Vector3, Matrix4, Perspective, SquareMatrix, Ortho, Point3, EuclideanSpace};
use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Queue};
use wgpu::util::DeviceExt;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


/// Represents a camera
pub struct Camera {

    // Directly manipulated by user
    eye: Point3<f32>,
    direction: Vector3<f32>,
    up: Vector3<f32>,

    // Indirectly manipulated before writing
    projection: Matrix4<f32>,
    changed: bool,

    // Projection/view buffer
    buffer: Buffer,

    // Right-handedness flag
    is_right_handed: bool
}


impl Camera {

    /// Creates a right-handed camera using a `Device` to allocate buffers
    pub fn create_rh(
        device: &Device,
        eye: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
    ) -> Self {
        Self::create_handed(device, eye, direction, up, true)
    }

    /// Creates a left-handed camera using a `Device` to allocate buffers
    pub fn create_lh(
        device: &Device,
        eye: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
    ) -> Self {
        Self::create_handed(device, eye, direction, up, false)
    }

    /// Creates a right-handed camera using a `Device` to allocate buffers
    fn create_handed(
        device: &Device,
        eye: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
        is_right_handed: bool
    ) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Projection View Buffer"),
            size: std::mem::size_of::<[f32; 16]>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        Self {
            eye,
            direction,
            up,
            projection: Matrix4::identity(),
            changed: true,
            buffer,
            is_right_handed
        }
    }

    pub fn move_to(&mut self, eye: Point3<f32>) {
        self.eye = eye;
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

    /// Sets projection matrix to orthographic matrix
    pub fn set_ortho(&mut self, ortho: Ortho<f32>) {
        self.projection = ortho.into();
        self.changed = true;
    }

    /// Writes to internal
    pub fn write(&mut self, queue: &Queue) {
        if self.changed {
            let view = Matrix4::look_to_rh(self.eye, self.direction, self.up);
            let mut proj_view = self.projection * view;
            if self.is_right_handed {
                proj_view = OPENGL_TO_WGPU_MATRIX * proj_view;
            }
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
}