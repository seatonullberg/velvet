//! Builder types to construct system components.

use crate::errors::SystemInitializationError;
use crate::system::atoms::{AtomType, Atoms};
use crate::system::cell::Cell;
use crate::system::topology::Topology;

use chemfiles::{Frame, Trajectory};
use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};

use std::collections::HashMap;
use std::path::Path;

/// Constructor for [`Atoms`][`crate::system::atoms::Atoms`], [`Cell`][`crate::system::cell::Cell`],
/// and [`Topology`][`crate::system::topology::Topology`] which uses the [`chemfiles`] library
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
    ///
    /// - [`InvalidTrajectoryFile`](SystemInitializationError::InvalidTrajectoryFile) if the trajectory file is missing or improperly formatted.
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
    ///
    /// - [`MissingFrame`](SystemInitializationError::MissingFrame) if a valid [`chemfiles::Frame`] has not yet been set by calling
    /// either [`frame`](ChemfilesBuilder::frame) or [`trajectory_file`](ChemfilesBuilder::trajectory_file).
    ///
    /// - [`InvalidAtomType`](SystemInitializationError::InvalidAtomType) if any atom types in the [frame](chemfiles::Frame) do not have a valid mapping
    /// as set by [`atom_types`](ChemfilesBuilder::atom_types).
    ///
    /// - [`MissingAtomType`](SystemInitializationError::MissingAtomType) if there are any atom types missing from the [frame](chemfiles::Frame).
    pub fn build_atoms(&self) -> Result<Atoms, SystemInitializationError> {
        // Check that the user has supplied a frame to load atoms from.
        // If no frame has been supplied return an error.
        let mut frame = match &self.frame {
            None => return Err(SystemInitializationError::MissingFrame),
            Some(frame) => frame.clone(),
        };
        // Handle the case where user provides atom type mapping and
        // case where atom types are read directly from file.
        let atom_types = match &self.atom_types {
            // The user has provided a mapping of atom types.
            Some(user_atom_types) => parse_atom_types_with_mapping(&frame, user_atom_types),
            // The user has not provided a mapping of atom types.
            // Atom types will be parsed directly from the frame.
            None => parse_atom_types(&frame),
        };

        // If the atom type parsing failed return the error to the user.
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

    /// Build [`Cell`] from the given data.
    pub fn build_cell(&self) -> Result<Cell, SystemInitializationError> {
        // Check that the user has supplied a frame to load atoms from.
        // If no frame has been supplied return an error.
        let frame = match &self.frame {
            None => return Err(SystemInitializationError::MissingFrame),
            Some(frame) => frame.clone(),
        };
        parse_cell(&frame)
    }

    /// Build [`Topology`] from the given data.
    pub fn build_topology(&self) -> Topology {
        Topology::default()
    }
}

// Extract vec of atom types from chemfiles frame.
// Use pub(crate) to enable unit testing.
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
// Use pub(crate) to enable unit testing.
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
// Use pub(crate) to enable unit testing.
pub(crate) fn parse_positions(frame: &mut Frame) -> Vec<Vector3<f64>> {
    Vec::from_iter(
        frame
            .positions_mut()
            .iter_mut()
            .map(|p| Vector3::new(p[0], p[1], p[2])),
    )
}

// Extract vec of velocities from chemfiles frame if they exist else set velocities to zero.
// Use pub(crate) to enable unit testing.
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
// Use pub(crate) to enable unit testing.
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

// Extract the simulation cell bounds from chemfiles frame.
// Use pub(crate) to enable unit testing.
pub(crate) fn parse_cell(frame: &Frame) -> Result<Cell, SystemInitializationError> {
    let cell = frame.cell();
    let matrix = Matrix3::from(cell.matrix());
    // chemfiles does not tell you if the frame contains a cell but instead silently
    // returns a default (all zeros) matrix. The best way I can handle this is to check
    // if all elements are zero and assume that means the file did not have a cell section.
    // ISSUE FILED: https://github.com/chemfiles/chemfiles.rs/issues/42
    let mut frame_has_cell = false;
    for element in matrix.iter() {
        if element.ne(&0.0) {
            frame_has_cell = true;
            break;
        }
    }
    if frame_has_cell {
        Cell::try_from(matrix)
    } else {
        Err(SystemInitializationError::MissingCell)
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::SystemInitializationError;
    use crate::internal::get_resource_filepath;
    use crate::system::builders::{
        parse_atom_types, parse_atom_types_with_mapping, parse_cell, parse_positions,
    };
    use crate::system::AtomType;

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

    // Check that the `parse_cell` function works with a valid lammps data file.
    #[test]
    fn parse_cell_valid_lammps_data_file() {
        let path = get_resource_filepath("water.lmp");
        let mut trajectory = Trajectory::open_with_format(path, 'r', "LAMMPS Data").unwrap();
        let mut frame = Frame::new();
        trajectory.read_step(0, &mut frame).unwrap();
        let cell = parse_cell(&frame).unwrap();
        assert_eq!(cell.a(), 15.0);
        assert_eq!(cell.b(), 15.0);
        assert_eq!(cell.c(), 15.0);
    }

    // Check that the `parse_cell` function returns the correct error if the frame does not contain a cell.
    #[test]
    fn parse_cell_returns_missing_cell_error() {
        // Use PDB here because it does not store the cell.
        let path = get_resource_filepath("water.pdb");
        let mut trajectory = Trajectory::open_with_format(path, 'r', "PDB").unwrap();
        let mut frame = Frame::new();
        trajectory.read_step(0, &mut frame).unwrap();
        match parse_cell(&frame) {
            Ok(_) => panic!("unexpected ok result"),
            Err(err) => match err {
                SystemInitializationError::MissingCell => {}
                _ => panic!("unexpected error type"),
            },
        }
    }
}
