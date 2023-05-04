//! Collection of all bonds, angles, dihedrals, and impropers in the simulation environment.

use crate::system::internal::load_frame_from_trajectory_file;
use crate::system::FromTrajectoryFile;

use chemfiles::Frame;

/// Collection of all bonds, angles, dihedrals, and impropers in the simulation environment.
///
/// __Note:__ The [`Topology`]'s fields are all immutable because reactive chemistry is outside the
/// scope of this project.
#[derive(Clone, Debug, Default)]
pub struct Topology {
    pub(crate) bonds: Vec<[usize; 2]>,
    pub(crate) angles: Vec<[usize; 3]>,
    pub(crate) dihedrals: Vec<[usize; 4]>,
    pub(crate) impropers: Vec<[usize; 4]>,
    pub(crate) molecules: Vec<Vec<usize>>,
}

impl FromTrajectoryFile for Topology {
    fn from_trajectory_file<'a, P: AsRef<std::path::Path>, S: Into<&'a str>>(
        path: P,
        format: S,
        step: usize,
    ) -> Result<Self, crate::errors::SystemInitializationError> {
        // Load the frame from a trajectory file returning an error on unsuccessful read.
        let frame = match load_frame_from_trajectory_file(path, format, step) {
            Ok(frame) => frame,
            Err(err) => return Err(err),
        };

        // Parse bonds from frame.
        let bonds = parse_bonds(&frame);

        // Parse angles from frame.
        let angles = parse_angles(&frame);

        // Parse dihedrals from frame.
        let dihedrals = parse_dihedrals(&frame);

        // Parse impropers from frame.
        let impropers = parse_impropers(&frame);

        let molecules = parse_molecules(&frame);

        Ok(Topology {
            bonds,
            angles,
            dihedrals,
            impropers,
            molecules,
        })
    }
}

// Extract vec of (i, j) bond pairs.
pub(crate) fn parse_bonds(frame: &Frame) -> Vec<[usize; 2]> {
    let topology = frame.topology();
    topology.bonds()
}

// Extract vec of (i, j, k) angle triplets.
pub(crate) fn parse_angles(frame: &Frame) -> Vec<[usize; 3]> {
    let topology = frame.topology();
    topology.angles()
}

// Extract vec of (i, j, k, l) dihedral quads.
pub(crate) fn parse_dihedrals(frame: &Frame) -> Vec<[usize; 4]> {
    let topology = frame.topology();
    topology.dihedrals()
}

// Extract vec of (i, j, k, l) improper quads.
pub(crate) fn parse_impropers(frame: &Frame) -> Vec<[usize; 4]> {
    let topology = frame.topology();
    topology.impropers()
}

// Extract vec of molecules. Each element in the vec is a vec of
// indices of atoms which belong to the same molecule.
pub(crate) fn parse_molecules(frame: &Frame) -> Vec<Vec<usize>> {
    let topology = frame.topology();
    let res_count = topology.residues_count() as usize;
    let mut molecules: Vec<Vec<usize>> = Vec::with_capacity(res_count);
    for i in 0..res_count {
        // The unwrap is safe here because I am only iterating
        // over the predetermined number of residues.
        let residue = topology.residue(i as u64).unwrap();
        molecules.push(residue.atoms());
    }
    molecules
}

impl Topology {
    /// Returns a slice containing the indices of bonded atom pairs.
    pub fn bonds(&self) -> &[[usize; 2]] {
        &self.bonds
    }

    /// Returns a slice containing the indices of atoms in angle triplets.
    pub fn angles(&self) -> &[[usize; 3]] {
        &self.angles
    }

    /// Returns a slice containing the indices of atoms in dihedral quads.
    pub fn dihedrals(&self) -> &[[usize; 4]] {
        &self.dihedrals
    }

    /// Returns a slice containing the indices of atoms in improper quads.
    pub fn impropers(&self) -> &[[usize; 4]] {
        &self.impropers
    }

    /// Returns a slice containing the indices of atoms belonging to the same molecule.
    pub fn molecules(&self) -> &[Vec<usize>] {
        &self.molecules
    }
}

#[cfg(test)]
mod tests {
    use crate::internal::get_resource_filepath;
    use crate::system::internal::load_frame_from_trajectory_file;

    use super::{parse_angles, parse_bonds, parse_molecules};

    // Check that the `parse_bonds` function works with a valid lammps data file.
    #[test]
    fn parse_bonds_valid_lammps_data_file() {
        let path = get_resource_filepath("water.lmp");
        let frame = load_frame_from_trajectory_file(path, "LAMMPS Data", 0).unwrap();
        let bonds = parse_bonds(&frame);
        assert_eq!(bonds.len(), 200);
        assert_eq!(bonds[0], [0, 1]);
    }

    // Check that the `parse_angles` function works with a valid lammps data file.
    #[test]
    fn parse_angles_valid_lammps_data_file() {
        let path = get_resource_filepath("water.lmp");
        let frame = load_frame_from_trajectory_file(path, "LAMMPS Data", 0).unwrap();
        let angles = parse_angles(&frame);
        assert_eq!(angles.len(), 100);
        assert_eq!(angles[0], [1, 0, 2]);
    }

    // Check that the `parse_molecules` function works with a valid lammps data file.
    #[test]
    fn parse_molecules_valid_lammps_data_file() {
        let path = get_resource_filepath("water.lmp");
        let frame = load_frame_from_trajectory_file(path, "LAMMPS Data", 0).unwrap();
        let molecules = parse_molecules(&frame);
        assert_eq!(molecules.len(), 100);
        assert_eq!(molecules[0], [297, 298, 299]);
    }

    // TODO: Check dihedrals and impropers.
}
