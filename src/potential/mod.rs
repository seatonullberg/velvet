pub mod pair;

use crate::energy::EnergyEvaluator;
use crate::force::ForceEvaluator;
use crate::potential::pair::PairPotential;
use crate::system::System;

use nalgebra::Vector3;

/// Any interatomic potential.
pub struct Potential<T> {
    cutoff: f32,
    species: Vec<String>,
    evaluator: T,
}

// TODO
impl<T: PairPotential> EnergyEvaluator for Potential<T> {
    fn evaluate_energy(&self, system: &System, index: usize) -> f32 {
        0.0
    }
}

// TODO
impl<T: PairPotential> ForceEvaluator for Potential<T> {
    fn evaluate_force(&self, system: &System, index: usize) -> Vector3<f32> {
        Vector3::default()
    }
}
