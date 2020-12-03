pub mod cell;
pub mod element;

use nalgebra::Vector3;

use crate::system::cell::Cell;
use crate::system::element::Element;

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
    /// Mass of each atom in the system.
    pub masses: Vec<f32>,
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
