#![warn(clippy::all)]

#[macro_use]
extern crate strum_macros;

pub mod cell;
pub mod elements;
pub mod species;

/// User facing exports.
pub mod prelude {
    pub use super::cell::Cell;
    pub use super::elements::Element;
    pub use super::species::Species;
    pub use super::System;
}

use nalgebra::Vector3;

use crate::cell::Cell;
use crate::species::Species;
use velvet_internals::float::Float;

/// Collection of atomic properties and topology information.
#[derive(Clone, Debug)]
pub struct System {
    /// Total number of atoms.
    pub n_atoms: usize,
    /// Simulation cell with periodic boundary conditions.
    pub cell: Cell,
    /// Chemical species of each atom.
    pub species: Vec<Species>,
    /// Position of each atom.
    pub positions: Vec<Vector3<Float>>,
    /// Velocity of each atom.
    pub velocities: Vec<Vector3<Float>>,
    /// Indices of each bonded pair.
    pub bonds: Vec<[usize; 2]>,
    /// Indices of each angle triplet.
    pub angles: Vec<[usize; 3]>,
    /// Indices of ach dihedral quartet.
    pub dihedrals: Vec<[usize; 4]>,
}
