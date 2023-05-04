//! Data structures that store information about the physical state of the simulation environment.

pub mod atoms;
pub mod cell;
pub(crate) mod internal;
pub mod topology;

pub use atoms::{AtomType, Atoms};
pub use cell::Cell;
pub use topology::Topology;

use std::collections::HashMap;
use std::path::Path;

use crate::errors::SystemInitializationError;

/// Shared behavior for types that can be loaded from a trajectory file.
pub trait FromTrajectoryFile: Sized {
    /// Constructs from a trajectory file with a valid `format`.
    /// The `step` argument specifies which timestep to read from the trajectory.
    ///
    /// Refer to this [table](https://chemfiles.org/chemfiles/latest/formats.html) for a list of valid formats.
    ///
    /// # Errors
    ///
    /// The errors that may be encountered depend on the concrete type being constructed. However, for all
    /// types it is possible to encoutner an [`InvalidTrajectoryFile`](SystemInitializationError) error if the
    /// given trajectory file is missing or improperly formatted.
    fn from_trajectory_file<'a, P: AsRef<Path>, S: Into<&'a str>>(
        path: P,
        format: S,
        step: usize,
    ) -> Result<Self, SystemInitializationError>;

    /// Constructs from a trajectory file with an additional mapping of atom types.
    /// This is helpful when the trajectory file is missing atomic information such
    /// as mass or charge or when atom types have inconsistent labels.
    ///
    /// # Errors
    /// The errors that may be encountered depend on the concrete type being constructed. However, for all
    /// types it is possible to encoutner an [`InvalidTrajectoryFile`](SystemInitializationError) error if the
    /// given trajectory file is missing or improperly formatted. Additionally, this constructor may fail with an
    /// [`InvalidAtomType`](SystemInitializationError) error if any atom type found in the trajectory file does
    /// not have a mapping in `atom_types`.
    fn from_trajectory_file_with_atom_types<'a, P: AsRef<Path>, S: Into<&'a str>>(
        path: P,
        format: S,
        step: usize,
        atom_types: HashMap<&str, AtomType>,
    ) -> Result<Self, SystemInitializationError> {
        // By default just fallback to `from_trajectory_file` without considering the atom types.
        // This is the case for Cell and Topology where atom types are irrelevant.
        let _ = atom_types;
        FromTrajectoryFile::from_trajectory_file(path, format, step)
    }
}

/// Physical state of the simulation environment.
#[derive(Clone, Debug, Default)]
pub struct System {
    pub atoms: Atoms,
    pub cell: Cell,
    pub topology: Topology,
}

impl System {
    pub fn new(atoms: Atoms, cell: Cell, topology: Topology) -> System {
        System {
            atoms,
            cell,
            topology,
        }
    }
}
