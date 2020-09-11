use crate::simcell::SimulationCell;

use nalgebra::Vector3;

/// Trait to evaluate the force on an individual atom.
pub trait ForceEvaluator {
    fn evaluate_force(&self, cell: &SimulationCell, index: usize) -> Vector3<f32>;
}
