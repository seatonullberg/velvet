pub mod pair;

use crate::energy::EnergyEvaluator;
use crate::force::ForceEvaluator;
use crate::system::System;

use nalgebra::Vector3;

trait EnergyForceEvaluator: EnergyEvaluator + ForceEvaluator {}

/// Any interatomic potential.
pub struct Potential {
    cutoff: f32,
    species: Vec<String>,
    evaluator: Box<dyn EnergyForceEvaluator>,
}

impl EnergyEvaluator for Potential {
    fn evaluate_energy(&self, system: &System, index: usize) -> f32 {
        self.evaluator.evaluate_energy(system, index)
    }
}

impl ForceEvaluator for Potential {
    fn evaluate_force(&self, system: &System, index: usize) -> Vector3<f32> {
        self.evaluator.evaluate_force(system, index)
    }
}

impl EnergyForceEvaluator for Potential {}
