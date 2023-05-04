//! Collection of all atoms in the simulation environment.

use crate::errors::SystemInitializationError;
use crate::system::internal::load_frame_from_trajectory_file;
use crate::system::FromTrajectoryFile;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use chemfiles::Frame;
use itertools::Itertools;
use nalgebra::Vector3;

/// Representation of a specific type of atom or chemical species.
#[derive(Clone, Copy, Debug, Default)]
pub struct AtomType {
    id: u16,
    mass: f64,
    charge: f64,
}

impl AtomType {
    pub fn new(id: u16, mass: f64, charge: f64) -> Self {
        AtomType { id, mass, charge }
    }

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

/// Collection of all atoms in the simulation environment.
#[derive(Clone, Debug, Default)]
pub struct Atoms {
    pub(crate) atom_types: Vec<AtomType>,
    pub(crate) positions: Vec<Vector3<f64>>,
    pub(crate) velocities: Vec<Vector3<f64>>,
    pub(crate) accelerations: Vec<Vector3<f64>>,
    pub(crate) indices_by_atom_type: HashMap<AtomType, Vec<usize>>,
}

impl Atoms {
    /// Returns a slice containing the atom type of each atom.
    ///
    /// __Note:__ A simulation's atom types are immutable because transmutation
    /// of atom types during the course of a simulation is outside the
    /// scope of this project.
    pub fn atom_types(&self) -> &[AtomType] {
        &self.atom_types
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
}

impl FromTrajectoryFile for Atoms {
    fn from_trajectory_file<'a, P: AsRef<std::path::Path>, S: Into<&'a str>>(
        path: P,
        format: S,
        step: usize,
    ) -> Result<Self, SystemInitializationError> {
        // Build Atoms without an atom types mapping.
        build_atoms(path, format, step, None)
    }

    fn from_trajectory_file_with_atom_types<'a, P: AsRef<std::path::Path>, S: Into<&'a str>>(
        path: P,
        format: S,
        step: usize,
        atom_types: HashMap<&str, AtomType>,
    ) -> Result<Self, SystemInitializationError> {
        // Build Atoms with an atom types mapping.
        build_atoms(path, format, step, Some(atom_types))
    }
}

// Underlying Atoms builder that handles building with or without an atom types mapping.
fn build_atoms<'a, P: AsRef<std::path::Path>, S: Into<&'a str>>(
    path: P,
    format: S,
    step: usize,
    mapping: Option<HashMap<&str, AtomType>>,
) -> Result<Atoms, SystemInitializationError> {
    // Load the frame from a trajectory file returning an error on unsuccessful read.
    let mut frame = match load_frame_from_trajectory_file(path, format, step) {
        Ok(frame) => frame,
        Err(err) => return Err(err),
    };

    // Use the appropriate parsing function depending on whether or not a mapping has been provided.
    let atom_types = match mapping {
        Some(mapping) => parse_atom_types_with_mapping(&frame, &mapping),
        None => parse_atom_types(&frame),
    };

    // Unwrap the atom types from the result or return an error if parsing was unsuccessful.
    let atom_types = match atom_types {
        Ok(atom_types) => atom_types,
        Err(err) => return Err(err),
    };

    // Parse positions from frame.
    let positions = parse_positions(&mut frame);

    // Parse velocities from frame.
    let velocities = parse_velocities(&mut frame);

    // Instantiate empty accelerations.
    let accelerations: Vec<Vector3<f64>> = vec![Vector3::zeros(); frame.size()];

    // Create the mapping of indices by atom type.
    let indices_by_atom_type = get_indices_by_atom_type(&atom_types);

    Ok(Atoms {
        atom_types,
        positions,
        velocities,
        accelerations,
        indices_by_atom_type,
    })
}

// Extract vec of atom types from chemfiles frame.
pub(crate) fn parse_atom_types(frame: &Frame) -> Result<Vec<AtomType>, SystemInitializationError> {
    // Create a vec to store the parsed atom types.
    let mut atom_types: Vec<AtomType> = Vec::with_capacity(frame.size());
    // Create a hash map to store the atom types we create from information parsed out of the frame.
    let mut created_atom_types: HashMap<String, AtomType> = HashMap::new();

    // Iterate over all atoms in the frame.
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
    Ok(atom_types)
}

// Extract vec of atom types from chemfiles frame with additional data from user-provided mapping.
pub(crate) fn parse_atom_types_with_mapping(
    frame: &Frame,
    mapping: &HashMap<&str, AtomType>,
) -> Result<Vec<AtomType>, SystemInitializationError> {
    // Create a vec to store the parsed atom types.
    let mut atom_types: Vec<AtomType> = Vec::with_capacity(frame.size());

    // Iterate over all atoms in the frame.
    for atom in frame.iter_atoms() {
        let found_atom_type = atom.atomic_type();
        // Return an error if the atom type is an empty string.
        if found_atom_type.is_empty() {
            return Err(SystemInitializationError::MissingAtomType);
        }
        match mapping.get(found_atom_type.as_str()) {
            // The found atom type has a valid mapping.
            // Push the mapped atom type onto the vec.
            Some(mapped_atom_type) => atom_types.push(*mapped_atom_type),
            // The found atom type does not have a valid mapping.
            // Return an error informing the user.
            None => {
                let expected: Vec<String> =
                    Vec::from_iter(mapping.keys().map(|&s| String::from(s)));
                let found = found_atom_type;
                return Err(SystemInitializationError::InvalidAtomType { expected, found });
            }
        }
    }
    Ok(atom_types)
}

// Extract vec of positions from chemfiles frame.
pub(crate) fn parse_positions(frame: &mut Frame) -> Vec<Vector3<f64>> {
    Vec::from_iter(
        frame
            .positions_mut()
            .iter_mut()
            .map(|p| Vector3::new(p[0], p[1], p[2])),
    )
}

// Extract vec of velocities from chemfiles frame if they exist else set velocities to zero.
pub(crate) fn parse_velocities(frame: &mut Frame) -> Vec<Vector3<f64>> {
    match frame.velocities_mut() {
        Some(velocities) => velocities
            .iter_mut()
            .map(|v| Vector3::new(v[0], v[1], v[2]))
            .collect(),
        None => vec![Vector3::zeros(); frame.size()],
    }
}

// Create a hash map which maps the indices of individual atoms to their respective atom types.
// This mapping reduces the time required to produce (i,j) pairs when evaluating nonbonded interactions.
pub(crate) fn get_indices_by_atom_type(atom_types: &[AtomType]) -> HashMap<AtomType, Vec<usize>> {
    let mut indices_by_atom_type: HashMap<AtomType, Vec<usize>> =
        HashMap::from_iter(atom_types.iter().unique().map(|&at| (at, Vec::new())));
    for (i, atom_type) in atom_types.iter().enumerate() {
        // The unwrap is safe here because the hashmap keys were created directly from the values
        // we are currently iterating over.
        let indices = indices_by_atom_type.get_mut(atom_type).unwrap();
        indices.push(i);
    }
    indices_by_atom_type
}

#[cfg(test)]
mod tests {
    use super::{parse_atom_types, parse_atom_types_with_mapping, parse_positions, AtomType};
    use crate::errors::SystemInitializationError;
    use crate::internal::get_resource_filepath;

    use std::collections::HashMap;

    use approx::assert_relative_eq;
    use chemfiles::{Frame, Trajectory};

    // Check that the `parse_positions` function works with a valid lammps data file.
    #[test]
    fn parse_positions_valid_lammps_data_file() {
        let path = get_resource_filepath("water.lmp");
        let mut trajectory = Trajectory::open_with_format(path, 'r', "LAMMPS Data").unwrap();
        let mut frame = Frame::new();
        trajectory.read_step(0, &mut frame).unwrap();
        let positions = parse_positions(&mut frame);
        assert_eq!(positions.len(), 300);
        assert_eq!(positions[0][0], 12.265);
        assert_eq!(positions[0][1], 7.861);
        assert_eq!(positions[0][2], 5.777);
    }

    // Check that the `parse_atom_types` function works with a valid lammps data file.
    #[test]
    fn parse_atom_types_valid_lammps_data_file() {
        let path = get_resource_filepath("water.lmp");
        let mut trajectory = Trajectory::open_with_format(path, 'r', "LAMMPS Data").unwrap();
        let mut frame = Frame::new();
        trajectory.read_step(0, &mut frame).unwrap();
        let atom_types = parse_atom_types(&frame).unwrap();
        assert_eq!(atom_types.len(), 300);
        // Atom type ids are based on the order they occur in the file when none are provided by the user.
        // The atom type labels found in the file (int or otherwise) are ignore and replaced with this schema.
        assert_eq!(atom_types[0].id(), 0);
        assert_eq!(atom_types[1].id(), 1);
        // If available the masses are read from the file.
        assert_relative_eq!(atom_types[0].mass(), 15.9994, epsilon = 1e-5);
        assert_relative_eq!(atom_types[1].mass(), 1.008, epsilon = 1e-5);
        // Charges are set to 0 in this file so we expect both types to have 0
        // despite divergence from physical expectations.
        assert_eq!(atom_types[0].charge(), 0.0);
        assert_eq!(atom_types[1].charge(), 0.0);
        // Atom types are compared directly by their ids.
        assert_ne!(atom_types[0], atom_types[1]);
        assert_eq!(atom_types[0], atom_types[3]);
        assert_eq!(atom_types[1], atom_types[2]);
    }

    // Check that the `parse_atom_types_with_mapping` function works with a valid lammps data file.
    #[test]
    fn parse_atom_types_with_mapping_valid_lammps_data_file() {
        let path = get_resource_filepath("water.lmp");
        let mut trajectory = Trajectory::open_with_format(path, 'r', "LAMMPS Data").unwrap();
        let mut frame = Frame::new();
        trajectory.read_step(0, &mut frame).unwrap();
        let oxygen = AtomType::new(1, 16.0, -2.0);
        let hydrogen = AtomType::new(2, 1.0, 1.0);
        let mapping = HashMap::from([("1", oxygen), ("2", hydrogen)]);
        let atom_types = parse_atom_types_with_mapping(&frame, &mapping).unwrap();
        assert_eq!(atom_types.len(), 300);
        // User-provided atom types have custom ids, in this case they are set to the int conversion of the ones found in the file.
        assert_eq!(atom_types[0].id(), 1);
        assert_eq!(atom_types[1].id(), 2);
        // Custom masses override those found in the file.
        assert_relative_eq!(atom_types[0].mass(), 16.0, epsilon = 1e-5);
        assert_relative_eq!(atom_types[1].mass(), 1.0, epsilon = 1e-5);
        // Charges are set to 0 in the file but we override them in the custom types.
        assert_eq!(atom_types[0].charge(), -2.0);
        assert_eq!(atom_types[1].charge(), 1.0);
        // Atom types are compared directly by their ids.
        assert_ne!(atom_types[0], atom_types[1]);
        assert_eq!(atom_types[0], atom_types[3]);
        assert_eq!(atom_types[1], atom_types[2]);
    }

    // Check that the `parse_atom_types_with_mappings` function returns the correct error if there are mismatched atom types.
    #[test]
    fn parse_atom_types_with_mapping_returns_invalid_atom_type_error() {
        let path = get_resource_filepath("water.lmp");
        let mut trajectory = Trajectory::open_with_format(path, 'r', "LAMMPS Data").unwrap();
        let mut frame = Frame::new();
        trajectory.read_step(0, &mut frame).unwrap();
        let oxygen = AtomType::new(1, 16.0, -2.0);
        let hydrogen = AtomType::new(2, 1.0, 1.0);
        // Intentionally use keys that are not present in the data file.
        let mapping = HashMap::from([("3", oxygen), ("4", hydrogen)]);
        match parse_atom_types_with_mapping(&frame, &mapping) {
            Ok(_) => panic!("unexpected ok result"),
            Err(err) => match err {
                SystemInitializationError::InvalidAtomType {
                    expected: _,
                    found: _,
                } => {}
                _ => panic!("unexpected error type"),
            },
        }
    }

    // Check that the `parse_atom_types` function returns the correct error if there are missing atom types.
    #[test]
    fn parse_atom_types_returns_missing_atom_type_error() {
        // Use PDB file here because it does not store the atom type.
        let path = get_resource_filepath("water.pdb");
        let mut trajectory = Trajectory::open_with_format(path, 'r', "PDB").unwrap();
        let mut frame = Frame::new();
        trajectory.read_step(0, &mut frame).unwrap();
        match parse_atom_types(&frame) {
            Ok(_) => panic!("unexpected ok result"),
            Err(err) => match err {
                SystemInitializationError::MissingAtomType => {}
                _ => panic!("unexpected error type"),
            },
        }
    }

    // Check that the `parse_atom_types_with_mapping` function returns the correct error if there are missing atom types.
    #[test]
    fn parse_atom_types_with_mapping_returns_missing_atom_type_error() {
        // Use PDB file here because it does not store the atom type.
        let path = get_resource_filepath("water.pdb");
        let mut trajectory = Trajectory::open_with_format(path, 'r', "PDB").unwrap();
        let mut frame = Frame::new();
        trajectory.read_step(0, &mut frame).unwrap();
        let oxygen = AtomType::new(1, 16.0, -2.0);
        let hydrogen = AtomType::new(2, 1.0, 1.0);
        let mapping = HashMap::from([("1", oxygen), ("2", hydrogen)]);
        match parse_atom_types_with_mapping(&frame, &mapping) {
            Ok(_) => panic!("unexpected ok result"),
            Err(err) => match err {
                SystemInitializationError::MissingAtomType => {}
                _ => panic!("unexpected error type"),
            },
        }
    }
}
