//! Data structures to hold physical information about the simulation environment.

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
    elements: Vec<Element>,
    /// Position of each atom in the system.
    positions: Vec<Vector3<f32>>,
    /// Velocity of each atom in the system.
    velocities: Vec<Vector3<f32>>,
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

    pub fn set_cell(&mut self, cell: Cell) {
        self.cell = cell;
    }

    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
    }

    /// Returns an iterator over the elements in the system.
    pub fn iter_elements(&self) -> impl Iterator<Item = &Element> {
        self.elements.iter()
    }

    pub fn iter_mut_elements(&mut self) -> impl Iterator<Item = &mut Element> {
        self.elements.iter_mut()
    }

    pub fn set_positions(&mut self, positions: Vec<Vector3<f32>>) {
        self.positions = positions;
    }

    /// Returns an iterator over the position vectors in the system
    pub fn iter_positions(&self) -> impl Iterator<Item = &Vector3<f32>> {
        self.positions.iter()
    }

    pub fn iter_mut_positions(&mut self) -> impl Iterator<Item = &mut Vector3<f32>> {
        self.positions.iter_mut()
    }

    pub fn set_velocities(&mut self, velocities: Vec<Vector3<f32>>) {
        self.velocities = velocities;
    }

    /// Returns an iterator over the velocity vectors in the system
    pub fn iter_velocities(&self) -> impl Iterator<Item = &Vector3<f32>> {
        self.velocities.iter()
    }

    pub fn iter_mut_velocities(&mut self) -> impl Iterator<Item = &mut Vector3<f32>> {
        self.velocities.iter_mut()
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
