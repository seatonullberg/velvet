use nalgebra::Vector3;

use crate::potential::ForceEvaluator;
use crate::system::System;

/// Required behaviors for a proper dihedral torsion interatomic potential.
pub trait ImproperPotential {
    /// Returns the potential energy of an atom in a quadruplet with an improper torsion angle `phi`.
    fn energy(&self, phi: f32) -> f32;
    /// Returns the magnitude of the force acting on an atom in a quadruplet due to improper torsion.
    fn force(&self, phi: f32) -> f32;
}

pub struct ImproperPotentialData {
    /// Heap allocated interatomic improper potential
    potential: Box<dyn ImproperPotential>,
    /// Improper type this potential applies to.
    dihedral: usize,
}

impl ForceEvaluator for ImproperPotentialData {
    fn evaluate_forces(&self, system: &System) -> Vec<Vector3<f32>> {
        unimplemented!()
    }
}
