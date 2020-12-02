use nalgebra::Vector3;

use crate::potential::{ForceEvaluator, Scope};
use crate::system::System;

/// Required behaviors for a coulombic interatomic potential.
pub trait BondPotential {
    /// Returns the potential energy between two charged particles.
    fn energy(&self, r: f32, q1: f32, q2: f32) -> f32;
    /// Returns the magnitude of the force of point charge `q2` acting on `q1`.
    fn force(&self, r: f32, q1: f32, q2: f32) -> f32;
}

/// Coulomb potential with required metadata.
pub struct CoulombPotentialData {
    /// Heap allocated interatomic bond potential.
    potential: Box<dyn BondPotential>,
    /// Pair of species this potential applies to.
    species: (usize, usize),
    /// Applicability of the potential.
    scope: Scope,
    /// Cutoff radius within which the potential is applied.
    cutoff: f32,
}

impl ForceEvaluator for CoulombPotentialData {
    fn evaluate_forces(&self, system: &System) -> Vec<Vector3<f32>> {
        unimplemented!()
    }
}
