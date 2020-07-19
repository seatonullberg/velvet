use crate::system::System;

use nalgebra::{Dynamic, MatrixMN, U3};

/// Trait for force evaluation of the system as a whole.
pub trait ForceEvaluator {
    fn evaluate_force(&self, system: &System) -> MatrixMN<f32, Dynamic, U3>;
}
