// Shamelessly copied from https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[cfg(test)]
mod tests {
    use cgmath::{Vector4};
    use crate::graphics::matrices::OPENGL_TO_WGPU_MATRIX;

    #[test]
    fn test_transform() {
        let v = Vector4::new(0.0, 0.0, 1.0, 1.0);
        let v2 = OPENGL_TO_WGPU_MATRIX * v;
        assert_eq!(v2, Vector4::new(0.0, 0.0, 0.0, 1.0));
    }
}