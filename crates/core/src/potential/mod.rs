pub mod angle;
pub mod bond;
pub mod coulomb;
pub mod dihedral;
pub mod improper;
pub mod pair;

use nalgebra::Vector3;

use crate::system::System;

/// Evaluates the forces acting on all atoms in the system.
pub trait ForceEvaluator {
    fn evaluate_forces(&self, system: &System) -> Vec<Vector3<f32>>;
}

/// Restrictions which can be applied to a potential.
#[derive(Clone, Copy, Debug)]
pub enum Scope {
    /// Applies to all atoms in the system.
    Global,
    /// Applies only to atoms in separate molecules.
    Intermolecular,
    /// Applies only to atoms within the same molecule.
    Intramolecular,
}
