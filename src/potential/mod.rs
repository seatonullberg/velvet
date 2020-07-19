pub mod pair;

use crate::energy::EnergyEvaluator;
use crate::force::ForceEvaluator;
use crate::system::System;

use nalgebra::{Dynamic, MatrixMN, U3};

trait EnergyForceEvaluator: EnergyEvaluator + ForceEvaluator {}

pub struct Potential {
    cutoff: f32,
    species: Vec<String>,
    evaluator: Box<dyn EnergyForceEvaluator>,
}

impl EnergyEvaluator for Potential {
    fn evaluate_energy(&self, system: &System) -> MatrixMN<f32, Dynamic, U3> {
        self.evaluator.evaluate_energy(system)
    }
}

impl ForceEvaluator for Potential {
    fn evaluate_force(&self, system: &System) -> MatrixMN<f32, Dynamic, U3> {
        self.evaluator.evaluate_force(system)
    }
}
