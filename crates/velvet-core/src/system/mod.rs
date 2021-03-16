//! Data structures to hold physical information about the simulation environment.

use std::collections::HashMap;

pub mod cell;
pub mod elements;
pub mod species;

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::system::cell::Cell;
use crate::system::species::Specie;

/// Collection of atomic properties and bonding information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct System {
    /// Number of atoms in the system.
    pub size: usize,
    /// Simulation cell with periodic boundary conditions.
    pub cell: Cell,
    /// Species in the system mapped to their ID.
    pub species: HashMap<usize, Specie>,
    /// Specie ID of each atom in the system.
    pub specie_ids: Vec<usize>,
    /// Position of each atom in the system.
    pub positions: Vec<Vector3<Float>>,
    /// Velocity of each atom in the system.
    pub velocities: Vec<Vector3<Float>>,
}

// /// Constructor for the [`System`](velvet_core::system::System) type.
// pub struct SystemBuilder {
//     size: usize,
//     cell: Option<Cell>,
//     species: Option<Vec<Specie>>,
//     positions: Option<Vec<Vector3<Float>>>,
//     velocities: Option<Vec<Vector3<Float>>>,
// }

// impl SystemBuilder {
//     /// Returns a new system builder.
//     ///
//     /// # Arguments
//     ///
//     /// * `size` - The number of atoms in the system
//     pub fn new(size: usize) -> SystemBuilder {
//         SystemBuilder {
//             size,
//             cell: None,
//             species: None,
//             positions: None,
//             velocities: None,
//         }
//     }

//     /// Sets the system cell.
//     pub fn with_cell(mut self, cell: Cell) -> SystemBuilder {
//         self.cell = Some(cell);
//         self
//     }

//     /// Sets the specie types in the system.
//     pub fn with_species(mut self, species: Vec<Specie>) -> SystemBuilder {
//         self.species = Some(species);
//         self
//     }

//     pub fn with_specie_ids

//     /// Sets the position of each atom in the system.
//     pub fn with_positions(mut self, positions: Vec<Vector3<Float>>) -> SystemBuilder {
//         assert!(positions.len() == self.size);
//         self.positions = Some(positions);
//         self
//     }

//     /// Sets the velocity of each atom in the system.
//     pub fn with_velocities(mut self, velocities: Vec<Vector3<Float>>) -> SystemBuilder {
//         assert!(velocities.len() == self.size);
//         self.velocities = Some(velocities);
//         self
//     }

//     /// Finalizes the build and returns an initialized system.
//     pub fn build(self) -> System {
//         let cell = match self.cell {
//             Some(c) => c,
//             None => panic!("System requires `cell` attribute"),
//         };
//         let species = match self.species {
//             Some(e) => e,
//             None => panic!("System requires `species` attribute"),
//         };
//         let positions = match self.positions {
//             Some(p) => p,
//             None => panic!("System requires `positions` attribute"),
//         };
//         let velocities = match self.velocities {
//             Some(v) => v,
//             None => vec![Vector3::new(0.0, 0.0, 0.0); self.size],
//         };

//         System {
//             size: self.size,
//             cell,
//             species,
//             positions,
//             velocities,
//         }
//     }
// }
