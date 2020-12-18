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
    pub cell: Cell,

    /// Element type for each atom in the system.
    pub elements: Vec<Element>,
    /// Molecule type for each atom in the system.
    pub molecules: Vec<usize>,
    /// Position of each atom in the system.
    pub positions: Vec<Vector3<f32>>,
    /// Velocity of each atom in the system.
    pub velocities: Vec<Vector3<f32>>,
    /// Electronic charge of each atom in the system.
    pub charges: Vec<f32>,

    /// Collection of bond indices grouped by bond type.
    pub bonds: Vec<Vec<(usize, usize)>>,
    /// Collection of angle triplet indices grouped by angle type.
    pub angles: Vec<Vec<(usize, usize, usize)>>,
    /// Collection of dihedral quadruplet indices grouped by dihedral type.
    pub dihedrals: Vec<Vec<(usize, usize, usize, usize)>>,
}

impl System {
    /// Returns the number of atoms in the system.
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Constructor for the [`System`](velvet_core::system::System) type.
pub struct SystemBuilder {
    size: usize,
    cell: Option<Cell>,
    elements: Option<Vec<Element>>,
    molecules: Option<Vec<usize>>,
    positions: Option<Vec<Vector3<f32>>>,
    velocities: Option<Vec<Vector3<f32>>>,
    charges: Option<Vec<f32>>,
    bonds: Option<Vec<Vec<(usize, usize)>>>,
    angles: Option<Vec<Vec<(usize, usize, usize)>>>,
    dihedrals: Option<Vec<Vec<(usize, usize, usize, usize)>>>,
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
            molecules: None,
            positions: None,
            velocities: None,
            charges: None,
            bonds: None,
            angles: None,
            dihedrals: None,
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

    /// Sets the molecule of each atom in the system.
    pub fn with_molecules(mut self, molecules: Vec<usize>) -> SystemBuilder {
        assert!(molecules.len() == self.size);
        self.molecules = Some(molecules);
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

    /// Sets the charge of each atom in the system.
    pub fn with_charges(mut self, charges: Vec<f32>) -> SystemBuilder {
        assert!(charges.len() == self.size);
        self.charges = Some(charges);
        self
    }

    /// Sets the pairwise bonds in the system.
    pub fn with_bonds(mut self, bonds: Vec<Vec<(usize, usize)>>) -> SystemBuilder {
        self.bonds = Some(bonds);
        self
    }

    /// Sets the angle triplets in the system.
    pub fn with_angles(mut self, angles: Vec<Vec<(usize, usize, usize)>>) -> SystemBuilder {
        self.angles = Some(angles);
        self
    }

    /// Sets the dihedral quadruplets in the system.
    pub fn with_dihedrals(
        mut self,
        dihedrals: Vec<Vec<(usize, usize, usize, usize)>>,
    ) -> SystemBuilder {
        self.dihedrals = Some(dihedrals);
        self
    }

    /// Finalizes the build and returns an initialized system.
    pub fn finish(self) -> System {
        let cell = match self.cell {
            Some(c) => c,
            None => panic!("System requires `cell` attribute"),
        };
        let elements = match self.elements {
            Some(e) => e,
            None => panic!("System requires `elements` attribute"),
        };
        let molecules = match self.molecules {
            Some(m) => m,
            None => vec![0 as usize; self.size],
        };
        let positions = match self.positions {
            Some(p) => p,
            None => panic!("System requires `positions` attribute"),
        };
        let velocities = match self.velocities {
            Some(v) => v,
            None => vec![Vector3::new(0.0, 0.0, 0.0); self.size],
        };
        let charges = match self.charges {
            Some(c) => c,
            None => vec![0.0; self.size],
        };
        let bonds = match self.bonds {
            Some(b) => b,
            None => Vec::new(),
        };
        let angles = match self.angles {
            Some(a) => a,
            None => Vec::new(),
        };
        let dihedrals = match self.dihedrals {
            Some(d) => d,
            None => Vec::new(),
        };
        System {
            size: self.size,
            cell,
            elements,
            molecules,
            positions,
            velocities,
            charges,
            bonds,
            angles,
            dihedrals,
        }
    }
}
