//! Data structures that store information about the physical state of the simulation environment.

use crate::errors::SystemInitializationError;

use self::atoms::{AtomType, Atoms};
use self::cell::Cell;
use self::topology::Topology;

use chemfiles::{Frame, Trajectory};
use itertools::Itertools;
use nalgebra::Vector3;

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
    ///
    /// # Errors
    /// - [`SystemInitializationError::InvalidTrajectoryFile`] if the trajectory file is missing or improperly formatted.
    pub fn trajectory_file<P, S>(
        &mut self,
        path: P,
        format: S,
        step: usize,
    ) -> Result<&mut Self, SystemInitializationError>
    where
        P: AsRef<Path>,
        S: Into<&'a str>,
    {
        // Load a trajectory object from file.
        let mut trajectory = match Trajectory::open_with_format(path, 'r', format) {
            Ok(trajectory) => trajectory,
            Err(err) => return Err(SystemInitializationError::InvalidTrajectoryFile(err)),
        };
        // Read a frame from the trajectory and set the internal `frame`
        // attribute or return an error if the read was unsuccessful.
        let mut frame = Frame::new();
        match trajectory.read_step(step, &mut frame) {
            Ok(()) => self.frame = Some(frame),
            Err(err) => return Err(SystemInitializationError::InvalidTrajectoryFile(err)),
        }
        Ok(self)
    }

    /// Optionally add a mapping of atom types found in a [`chemfiles`] frame to a user-specified [`AtomType`].
    /// This is helpful when the frame is missing some atomic information such as mass or charge or when atom types
    /// have inconsistent labels.
    pub fn atom_types(&mut self, atom_types: HashMap<&'a str, AtomType>) -> &mut Self {
        self.atom_types = Some(atom_types);
        self
    }

    /// Build [`Atoms`] from the given data.
    ///
    /// # Errors
    /// - [`SystemInitializationError::MissingFrame`] if a valid [`chemfiles::Frame`] has not yet been set by calling
    /// either [`frame`](ChemfilesBuilder::frame) or [`trajectory_file`](ChemfilesBuilder::trajectory_file).
    /// - [`SystemInitializationError::InvalidAtomType`] if any atom types in the [frame](chemfiles::Frame) do not have a valid mapping
    /// as set by [`atom_types`](ChemfilesBuilder::atom_types).
    /// - [`SystemInitializationError::MissingAtomType`] if there are any atom types missing from the [frame](chemfiles::Frame).
    pub fn build_atoms(&self) -> Result<Atoms, SystemInitializationError> {
        // Check that the user has supplied a frame to load atoms from.
        // If no frame has been supplied return an error.
        let mut frame = match &self.frame {
            None => return Err(SystemInitializationError::MissingFrame),
            Some(frame) => frame.clone(),
        };

        // Create a vec to store the parsed atom types.
        let mut atom_types: Vec<AtomType> = Vec::with_capacity(frame.size());
        // Handle the case where user provides atom type mapping and
        // case where atom types are read directly from file.
        match &self.atom_types {
            // The user has provided a mapping of atom types.
            Some(user_atom_types) => {
                // Iterate over all atoms in the frame.
                for atom in frame.iter_atoms() {
                    let found_atom_type = atom.atomic_type();
                    match user_atom_types.get(found_atom_type.as_str()) {
                        // The found atom type has a valid mapping.
                        // Push the mapped atom type onto the vec.
                        Some(mapped_atom_type) => atom_types.push(*mapped_atom_type),
                        // The found atom type does not have a valid mapping.
                        // Return an error informing the user.
                        None => {
                            let expected: Vec<String> =
                                Vec::from_iter(user_atom_types.keys().map(|&s| String::from(s)));
                            let found = found_atom_type;
                            return Err(SystemInitializationError::InvalidAtomType {
                                expected,
                                found,
                            });
                        }
                    }
                }
            }
            // The user has not provided a mapping of atom types.
            // Atom types will be parsed directly from the frame.
            None => {
                // Create a hash map to store the atom types we create from information parsed out of the frame.
                let mut created_atom_types: HashMap<String, AtomType> = HashMap::new();
                for atom in frame.iter_atoms() {
                    let found_atom_type = atom.atomic_type();
                    // Return an error if the atom type is an empty string.
                    if found_atom_type.is_empty() {
                        return Err(SystemInitializationError::MissingAtomType);
                    }
                    let atom_type = match created_atom_types.get(&found_atom_type) {
                        // If the atom type is already in our created map simply take a copy of it.
                        Some(atom_type) => *atom_type,
                        // If the atom type is new then create a new entry for it and add it to the map.
                        None => {
                            let id = created_atom_types.len() as u16;
                            let mass = atom.mass();
                            let charge = atom.charge();
                            let atom_type = AtomType::new(id, mass, charge);
                            created_atom_types.insert(found_atom_type, atom_type);
                            atom_type
                        }
                    };
                    // Push the determined atom type onto the vec.
                    atom_types.push(atom_type);
                }
            }
        }

        // Parse positions from frame.
        let positions: Vec<Vector3<f64>> = Vec::from_iter(
            frame
                .positions_mut()
                .iter_mut()
                .map(|p| Vector3::new(p[0], p[1], p[2])),
        );

        // Parse velocities from frame.
        let velocities: Vec<Vector3<f64>> = match frame.velocities_mut() {
            Some(velocities) => velocities
                .iter_mut()
                .map(|v| Vector3::new(v[0], v[1], v[2]))
                .collect(),
            None => vec![Vector3::zeros(); frame.size()],
        };

        // Instantiate empty accelerations.
        let accelerations: Vec<Vector3<f64>> = vec![Vector3::zeros(); frame.size()];

        // Create the mapping of indices by atom type.
        let mut indices_by_atom_type: HashMap<AtomType, Vec<usize>> =
            HashMap::from_iter(atom_types.iter().unique().map(|&at| (at, Vec::new())));
        for (i, atom_type) in atom_types.iter().enumerate() {
            // The unwrap is safe here because the hashmap keys were created directly from the values
            // we are currently iterating over.
            let indices = indices_by_atom_type.get_mut(atom_type).unwrap();
            indices.push(i);
        }

        Ok(Atoms {
            atom_types,
            positions,
            velocities,
            accelerations,
            indices_by_atom_type,
        })
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
