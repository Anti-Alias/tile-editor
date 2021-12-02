use cgmath::{BaseFloat, Deg, Matrix4, Rad, Vector3};
use wgpu::{Device, Extent3d, Surface, SurfaceConfiguration, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor};


/// Adds line numbers to multi-line strings
pub fn string_with_lines(source: &str) -> String {
    let mut result = String::new();
    for (i, line) in source.lines().enumerate() {
        let header = format!("{:>4}|  ", i+1);
        result.push_str(&header);
        result.push_str(line);
        result.push('\n');
    }
    result
}

pub trait Matrix4Ext<S: BaseFloat> {
    fn translate(self, vector: Vector3<S>) -> Self;
    fn rotate(self, axis: Vector3<S>, rotation: impl Into<Rad<S>>) -> Self;
    fn rotate_radians(self, axis: Vector3<S>, radians: S) -> Self;
    fn rotate_degrees(self, axis: Vector3<S>, degrees: S) -> Self;
    fn scale(self, scale: Vector3<S>) -> Self;
}

impl<S: BaseFloat> Matrix4Ext<S> for Matrix4<S> {
    fn translate(self, translation: Vector3<S>) -> Self {
        self * Matrix4::from_translation(translation)
    }
    fn rotate(self, axis: Vector3<S>, rotation: impl Into<Rad<S>>) -> Self {
        self * Matrix4::from_axis_angle(axis, rotation)
    }
    fn rotate_radians(self, axis: Vector3<S>, radians: S) -> Self {
        self * Matrix4::from_axis_angle(axis, Rad(radians))
    }
    fn rotate_degrees(self, axis: Vector3<S>, degrees: S) -> Self {
        self * Matrix4::from_axis_angle(axis, Deg(degrees))
    }
    fn scale(self, scale: Vector3<S>) -> Self {
        self * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z)
    }
}