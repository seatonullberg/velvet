//! Data structures to hold physical information about the simulation environment.

pub mod cell;
pub mod elements;
pub mod species;

use nalgebra::Vector3;

use crate::internal::Float;
use crate::system::cell::Cell;
use crate::system::species::Specie;

/// Collection of atomic properties and bonding information.
#[derive(Clone, Debug)]
pub struct System {
    /// Number of atoms in the system.
    pub size: usize,
    /// Simulation cell with periodic boundary conditions.
    pub cell: Cell,
    /// Each specie in the system.
    pub species: Vec<Specie>,
    /// Mapping of each atom in the system to its index in the vec of species.
    pub specie_indices: Vec<usize>,
    /// Position of each atom in the system.
    pub positions: Vec<Vector3<Float>>,
    /// Velocity of each atom in the system.
    pub velocities: Vec<Vector3<Float>>,
}
