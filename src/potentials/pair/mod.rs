//! Nonbonded pair potentials.

pub mod group;

use crate::potentials::Potential;

pub use group::PairPotentialGroup;

/// Shared behavior for nonbonded pair potentials.
pub trait PairPotential<'a>: Potential<'a> {
    /// Returns the energy of a pair of atoms separated by a distance `r`.
    fn energy(&self, r: f64) -> f64;

    /// Returns the force acting on a pair of atoms separated by a distance `r`.
    fn force(&self, r: f64) -> f64;
}
