use crate::system::System;

use nalgebra::{Dynamic, MatrixMN, U3};

pub trait ForceEvaluator {
    fn evaluate_force(&self, system: &System) -> MatrixMN<f32, Dynamic, U3>;
}
