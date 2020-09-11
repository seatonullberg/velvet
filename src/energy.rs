use crate::simcell::SimulationCell;

/// Trait to evaluate the potential energy of an individual atom.
pub trait EnergyEvaluator {
    fn evaluate_energy(&self, cell: &SimulationCell, index: usize) -> f32;
}
