use crate::system::System;

use nalgebra::{Dynamic, MatrixMN, U3};

pub trait EnergyEvaluator {
    fn evaluate_energy(&self, system: &System) -> MatrixMN<f32, Dynamic, U3>;
}
