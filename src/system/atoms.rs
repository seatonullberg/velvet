//! Collection of all atoms in the simulation environment.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use nalgebra::Vector3;

/// Representation of a specific type of atom or chemical species.
#[derive(Clone, Copy, Debug, Default)]
pub struct AtomType {
    id: u16,
    mass: f64,
    charge: f64,
}

impl AtomType {
    /// Returns the ID of this atom type.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the mass of this atom type.
    pub fn mass(&self) -> f64 {
        self.mass
    }

    /// Returns the electronic charge of this atom type.
    pub fn charge(&self) -> f64 {
        self.charge
    }
}

impl Hash for AtomType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for AtomType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for AtomType {}

/// Type representing a unique molecule in the simulation environment.
pub type MoleculeID = u16;

/// Collection of all atoms in the simulation environment.
///
/// This object should not be instantiated directly. Refer to
/// [`ChemfilesBuilder`][`super::ChemfilesBuilder`] for the
/// preferred constructor.
#[derive(Clone, Debug, Default)]
pub struct Atoms {
    atom_types: Vec<AtomType>,
    molecule_ids: Vec<MoleculeID>,
    positions: Vec<Vector3<f64>>,
    velocities: Vec<Vector3<f64>>,
    accelerations: Vec<Vector3<f64>>,
    indices_by_atom_type: HashMap<AtomType, Vec<usize>>,
    indices_by_molecule_id: HashMap<MoleculeID, Vec<usize>>,
}

impl Atoms {
    /// Returns a slice containing the atom type of each atom.
    ///
    /// A simulation's atom types are immutable because transmutation
    /// of atom types during the course of a simulation is outside the
    /// scope of this project.
    pub fn atom_types(&self) -> &[AtomType] {
        &self.atom_types
    }

    /// Returns a slice containing the id of the molecule that each atom is a member of.
    /// All atoms are members of exactly __one__ molecule.
    ///
    /// A simulation's molecule ID's are immutable because breaking or forming
    /// bonds during the course of a simulation is outside the scope of this project.
    pub fn molecule_ids(&self) -> &[u16] {
        &self.molecule_ids
    }

    /// Returns a slice containing the 3D spatial coordinate of each atom.
    pub fn positions(&self) -> &[Vector3<f64>] {
        &self.positions
    }

    /// Returns a mutable reference to the 3D spatial coordinate of each atom.
    pub fn positions_mut(&mut self) -> &mut [Vector3<f64>] {
        &mut self.positions
    }

    /// Returns a slice containing the 3D velocity of each atom.
    pub fn velocities(&self) -> &[Vector3<f64>] {
        &self.velocities
    }

    /// Returns a mutable reference to the 3D velocity of each atom.
    pub fn velocities_mut(&mut self) -> &mut [Vector3<f64>] {
        &mut self.velocities
    }

    /// Returns a slice containing the 3D acceleration of each atom.
    pub fn accelerations(&self) -> &[Vector3<f64>] {
        &self.accelerations
    }

    /// Returns a mutable reference to the 3D acceleration of each atom.
    pub fn accelerations_mut(&mut self) -> &mut [Vector3<f64>] {
        &mut self.accelerations
    }

    /// Returns the indices of all atoms of the given atom type or [`None`][`std::option::Option`] if the atom type does not exist.
    pub fn atoms_of_type(&self, atom_type: &AtomType) -> Option<&Vec<usize>> {
        self.indices_by_atom_type.get(atom_type)
    }

    /// Returns the indices of all atoms belonging to the given molecule or [`None`][`std::option::Option`] if the atom type does not exist.
    pub fn atoms_in_molecule(&self, molecule_id: &MoleculeID) -> Option<&Vec<usize>> {
        self.indices_by_molecule_id.get(molecule_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chemfiles::{Frame, Trajectory};

    #[test]
    fn test_chemfiles() {
        let lmp_file_path = "/home/seaton/repos/velvet/water.pdb";
        let mut traj = Trajectory::open_with_format(lmp_file_path, 'r', "PDB").unwrap();
        let mut frame = Frame::new();
        traj.read_step(0, &mut frame).unwrap();
        for atom in frame.iter_atoms() {
            let mass = atom.mass();
            let charge = atom.charge();
            let atomic_type = atom.atomic_type();
            let name = atom.name();
            println!(
                "Mass: {:?}, Charge: {:?}, Type: {:?}, Name: {:?}",
                mass, charge, atomic_type, name
            );
        }
    }
}
