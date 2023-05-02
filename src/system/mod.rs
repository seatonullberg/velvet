//! Data structures that store information about the physical state of the simulation environment.
use self::atoms::{AtomType, Atoms};
use self::cell::Cell;
use self::topology::Topology;

use chemfiles::Frame;

use std::collections::HashMap;
use std::path::Path;

pub mod atoms;
pub mod cell;
pub mod topology;

/// Constructor for [`Atoms`][`self::atoms::Atoms`], [`Cell`][`self::cell::Cell`],
/// and [`Topology`][`self::topology::Topology`] which uses the [`chemfiles`] library
/// to support multiple data formats.
#[derive(Clone, Default)]
pub struct ChemfilesBuilder<'a> {
    frame: Option<Frame>,
    atom_types: Option<HashMap<&'a str, AtomType>>,
}

impl<'a> ChemfilesBuilder<'a> {
    pub fn new() -> Self {
        ChemfilesBuilder::default()
    }

    /// Adds a frame from a [`chemfiles`] trajectory to the builder.
    pub fn frame(&mut self, frame: Frame) -> &mut Self {
        self.frame = Some(frame);
        self
    }

    /// Reads a trajectory file with the given `format` and adds the frame at the specified `step` to the builder.
    ///
    /// Refer to this [table](https://chemfiles.org/chemfiles/latest/formats.html) for supported formats.
    pub fn trajectory_file<P, S>(&mut self, path: P, format: S, step: usize) -> &mut Self
    where
        P: AsRef<Path>,
        S: Into<&'a str>,
    {
        self
    }

    /// Optionally add a mapping of atom types found in a [`chemfiles`] frame to a user-specified [`AtomType`].
    /// This is helpful when the frame is missing some atomic information such as mass or charge or when atom types
    /// have inconsistent labels.
    pub fn atom_types(&mut self, atom_types: HashMap<&'a str, AtomType>) -> &mut Self {
        self.atom_types = Some(atom_types);
        self
    }

    /// Build [`Atoms`] from the given data.
    pub fn build_atoms(&self) -> Atoms {
        Atoms::default()
    }

    /// Build [`Cell`] from the given data.
    pub fn build_cell(&self) -> Cell {
        Cell::default()
    }

    /// Build [`Topology`] from the given data.
    pub fn build_topology(&self) -> Topology {
        Topology::default()
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
