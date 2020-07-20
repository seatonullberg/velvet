use crate::system::System;

/// Trait to evaluate the potential energy of an individual atom.
pub trait EnergyEvaluator {
    fn evaluate_energy(&self, system: &System, index: usize) -> f32;
}
