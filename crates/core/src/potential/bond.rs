use nalgebra::Vector3;

use crate::potential::ForceEvaluator;
use crate::system::System;

/// Required behaviors for a bond style interatomic potential.
pub trait BondPotential {
    /// Returns the potential energy of a bond with length `r`.
    fn energy(&self, r: f32) -> f32;
    /// Returns the magnitude of the force acting on an atom due to a bond with length `r`.
    fn force(&self, r: f32) -> f32;
}

/// Bond potential with required metadata.
pub struct BondPotentialData {
    /// Heap allocated interatomic bond potential.
    potential: Box<dyn BondPotential>,
    /// Bond type this potential applies to.
    bond: usize,
}

impl ForceEvaluator for BondPotentialData {
    fn evaluate_forces(&self, system: &System) -> Vec<Vector3<f32>> {
        unimplemented!()
    }
}
