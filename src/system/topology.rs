//! Collection of all bonds, angles, dihedrals, and impropers in the simulation environment.

/// Collection of all bonds, angles, dihedrals, and impropers in the simulation environment.
/// 
/// This object should not be instantiated directly. Refer to
/// [`ChemfilesBuilder`](super::ChemfilesBuilder) for the
/// preferred constructor.
/// 
/// The [`Topology`]'s fields are all immutable because reactive chemistry is outside the 
/// scope of this project.
#[derive(Clone, Debug, Default)]
pub struct Topology {
    pub(crate) bonds: Vec<[usize; 2]>,
    pub(crate) angles: Vec<[usize; 3]>,
    pub(crate) dihedrals: Vec<[usize; 4]>,
    pub(crate) impropers: Vec<[usize; 4]>,
    pub(crate) molecules: Vec<Vec<usize>>,
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
