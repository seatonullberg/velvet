//! Data structures that store information about the physical state of the simulation environment.

pub mod atoms;
pub mod builders;
pub mod cell;
pub mod topology;

pub use atoms::{AtomType, Atoms};
pub use builders::ChemfilesBuilder;
pub use cell::Cell;
pub use topology::Topology;

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
