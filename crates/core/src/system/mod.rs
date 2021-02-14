//! Data structures to hold physical information about the simulation environment.

use std::collections::HashMap;

pub mod cell;
pub mod elements;

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::system::cell::Cell;
use crate::system::elements::Element;

/// Collection of atomic properties and bonding information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct System {
    /// Number of atoms in the system.
    size: usize,
    /// Simulation cell with periodic boundary conditions.
    cell: Cell,
    /// Element type for each atom in the system.
    pub elements: Vec<Element>,
    /// Position of each atom in the system.
    pub positions: Vec<Vector3<f32>>,
    /// Velocity of each atom in the system.
    pub velocities: Vec<Vector3<f32>>,
}

impl System {
    /// Returns the number of atoms in the system.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the simulation cell.
    pub fn cell(&self) -> &Cell {
        &self.cell
    }

    /// Returns a mapping of each element type to its number of occurrences in the system.
    pub fn element_counts(&self) -> HashMap<Element, usize> {
        let mut counts: HashMap<Element, usize> = HashMap::new();
        for i in 0..self.size() {
            let elem = self.elements[i];
            let count = match counts.get_mut(&elem) {
                Some(v) => *v,
                None => 1,
            };
            let _ = counts.insert(elem, count);
        }
        counts
    }
}

/// Constructor for the [`System`](velvet_core::system::System) type.
pub struct SystemBuilder {
    size: usize,
    cell: Option<Cell>,
    elements: Option<Vec<Element>>,
    positions: Option<Vec<Vector3<f32>>>,
    velocities: Option<Vec<Vector3<f32>>>,
}

impl SystemBuilder {
    /// Returns a new system builder.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of atoms in the system
    pub fn new(size: usize) -> SystemBuilder {
        SystemBuilder {
            size,
            cell: None,
            elements: None,
            positions: None,
            velocities: None,
        }
    }

    /// Sets the system cell.
    pub fn with_cell(mut self, cell: Cell) -> SystemBuilder {
        self.cell = Some(cell);
        self
    }

    /// Sets the element of each atom in the system.
    pub fn with_elements(mut self, elements: Vec<Element>) -> SystemBuilder {
        assert!(elements.len() == self.size);
        self.elements = Some(elements);
        self
    }

    /// Sets the position of each atom in the system.
    pub fn with_positions(mut self, positions: Vec<Vector3<f32>>) -> SystemBuilder {
        assert!(positions.len() == self.size);
        self.positions = Some(positions);
        self
    }

    /// Sets the velocity of each atom in the system.
    pub fn with_velocities(mut self, velocities: Vec<Vector3<f32>>) -> SystemBuilder {
        assert!(velocities.len() == self.size);
        self.velocities = Some(velocities);
        self
    }

    /// Finalizes the build and returns an initialized system.
    pub fn build(self) -> System {
        let cell = match self.cell {
            Some(c) => c,
            None => panic!("System requires `cell` attribute"),
        };
        let elements = match self.elements {
            Some(e) => e,
            None => panic!("System requires `elements` attribute"),
        };
        let positions = match self.positions {
            Some(p) => p,
            None => panic!("System requires `positions` attribute"),
        };
        let velocities = match self.velocities {
            Some(v) => v,
            None => vec![Vector3::new(0.0, 0.0, 0.0); self.size],
        };

        System {
            size: self.size,
            cell,
            elements,
            positions,
            velocities,
        }
    }
}
