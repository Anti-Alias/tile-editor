use cgmath::{Point3, Vector3};
use crate::graphics::Color;

/// Represents a simple point light that does not cast shadows
pub struct PointLight {
    pub position: Point3<f32>,
    pub color: Color
}

/// Represents a simple directional light that does not cast shadows
pub struct DirectionalLight {
    pub direction: Vector3<f32>,
    pub color: Color
}