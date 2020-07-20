use crate::system::System;

use nalgebra::Vector3;

/// Trait to evaluate the force on an individual atom.
pub trait ForceEvaluator {
    fn evaluate_force(&self, system: &System, index: usize) -> Vector3<f32>;
}
