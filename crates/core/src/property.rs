use nalgebra::Vector3;

use crate::system::System;

/// Valid output types for system properties.
pub enum Output {
    Scalar(f32),
    Vector(Vec<f32>),
    Matrix(Vec<Vector3<f32>>),
}

/// Calculates a system-wide property.
pub trait Property {
    fn calculate(&self, system: &System) -> Output;
}
