use nalgebra::Vector3;

use crate::potential::ForceEvaluator;
use crate::system::System;

/// Required behaviors for a proper dihedral torsion interatomic potential.
pub trait DihedralPotential {
    /// Returns the potential energy of an atom in a quad with a proper dihedral angle `phi`.
    fn energy(&self, phi: f32) -> f32;
    /// Returns the magnitude of the force acting on an atom in a quad due to proper dihedral torsion.
    fn force(&self, phi: f32) -> f32;
}

pub struct DihedralPotentialData {
    /// Heap allocated interatomic dihedral potential
    potential: Box<dyn DihedralPotential>,
    /// Dihedral type this potential applies to.
    dihedral: usize,
}

impl ForceEvaluator for DihedralPotentialData {
    fn evaluate_forces(&self, system: &System) -> Vec<Vector3<f32>> {
        unimplemented!()
    }
}
