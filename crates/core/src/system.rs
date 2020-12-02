use std::collections::HashMap;

use nalgebra::Vector3;

use crate::cell::Cell;

#[derive(Clone, Debug)]
pub struct System {
    /// Simulation cell with periodic boundary conditions.
    pub cell: Cell,

    /// 3D position of each atom in the system.
    pub positions: Vec<Vector3<f32>>,
    /// 3D velocity of each atom in the system.
    pub velocities: Vec<Vector3<f32>>,
    /// Mass of each atom in the system.
    pub masses: Vec<f32>,
    /// Electrical charge of each atom in the system.
    pub charges: Vec<f32>,
    /// Specie IDs of each atom in the system.
    pub specie_ids: Vec<usize>,
    /// Molecule IDs of each atom in the system.
    pub molecule_ids: Vec<usize>,

    /// Map specie IDs to atoms of that specie.
    pub species: HashMap<usize, Vec<usize>>,
    /// Map molecule IDs to atoms of that molecule.
    pub molecules: HashMap<usize, Vec<usize>>,
    /// Map pair types to instances of that pair.
    pub pairs: HashMap<(usize, usize), Vec<(usize, usize)>>,
    /// Map bond IDs to instances of that bond.
    pub bonds: HashMap<usize, Vec<(usize, usize)>>,
    /// Map angle IDs to instances of that angle triplet.
    pub angles: HashMap<usize, Vec<(usize, usize, usize)>>,
    /// Map dihedral IDs to instances of that dihedral quadruplet.
    pub dihedrals: HashMap<usize, Vec<(usize, usize, usize, usize)>>,
    /// Map improper IDs to instances of that improper quadruplet.
    pub impropers: HashMap<usize, Vec<(usize, usize, usize, usize)>>,

    /// Number of atoms in the system.
    _size: usize,
}

impl System {
    /// Returns the number of atoms in the system.
    pub fn size(&self) -> usize {
        self._size
    }
}
